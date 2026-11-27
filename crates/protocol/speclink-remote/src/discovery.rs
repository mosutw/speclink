//! Login-method discovery and choice (client-browser-login D1, D2).
//!
//! A client asks the server which login methods it supports before starting
//! one. The server's answer decides *which* methods are possible; the order
//! they are tried in is the client's own — browser, device, token — never the
//! order the server listed them in.
//!
//! A server without the route (404/405) has declared nothing. That is not an
//! error: the client keeps its pre-discovery behavior (device flow first, an
//! access token when the server has no device flow). Every other failure is an
//! error — a broken server must never read as "use a different method".

use crate::{translate_protocol_error, RemoteError};
use speclink_protocol::auth::{AuthMethodsResponse, BrowserLogin};

/// A login method this client knows how to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Browser,
    Device,
    Token,
}

impl Method {
    /// The order an unspecified choice tries methods in.
    const PREFERENCE: [Method; 3] = [Method::Browser, Method::Device, Method::Token];

    /// The method's wire name in the declaration.
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Browser => "browser",
            Method::Device => "device",
            Method::Token => "token",
        }
    }

    fn from_wire(name: &str) -> Option<Method> {
        Method::PREFERENCE.into_iter().find(|m| m.as_str() == name)
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What a server declared about its login methods.
#[derive(Debug, Clone, PartialEq)]
pub enum Discovery {
    /// The server has no discovery route (404/405).
    Undeclared,
    /// The usable part of the server's declaration: unknown methods are
    /// dropped, and `browser` is dropped unless its login service is complete.
    Declared {
        methods: Vec<Method>,
        browser: Option<BrowserLogin>,
    },
}

impl Discovery {
    fn from_response(resp: AuthMethodsResponse) -> Discovery {
        let browser = resp
            .browser
            .filter(|b| !b.issuer.trim().is_empty() && !b.client_id.trim().is_empty());
        let mut methods = Vec::new();
        for method in resp.methods.iter().filter_map(|name| Method::from_wire(name)) {
            // A browser method with no login service to send the user to
            // cannot be run; the rest of the declaration still counts.
            if method == Method::Browser && browser.is_none() {
                continue;
            }
            if !methods.contains(&method) {
                methods.push(method);
            }
        }
        let browser = browser.filter(|_| methods.contains(&Method::Browser));
        Discovery::Declared { methods, browser }
    }
}

/// `GET {origin}/api/speclink/v1/auth-methods`, without a credential.
pub fn discover(origin: &str) -> Result<Discovery, RemoteError> {
    let url = format!("{}/api/speclink/v1/auth-methods", origin.trim_end_matches('/'));
    let result = crate::http::agent_builder()
        .redirects(0)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .get(&url)
        .call();
    match result {
        Ok(resp) => resp
            .into_json::<AuthMethodsResponse>()
            .map(Discovery::from_response)
            .map_err(|_| RemoteError {
                message: "unexpected server response — the server did not return valid JSON"
                    .into(),
                reason: None,
                status: None,
                evidence: None,
            }),
        Err(ureq::Error::Status(404 | 405, _)) => Ok(Discovery::Undeclared),
        Err(ureq::Error::Status(status, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(translate_protocol_error(status, &body))
        }
        Err(ureq::Error::Transport(error)) => Err(crate::http::translate_transport_error(&error)),
    }
}

/// The login a client will run.
#[derive(Debug, Clone, PartialEq)]
pub enum Plan {
    Browser(BrowserLogin),
    /// The device flow. On a server that declared nothing this is the old
    /// behavior: a server without the device flow then asks for a token.
    Device,
    /// The server only takes an access token.
    Token,
}

/// Why no login could be chosen.
#[derive(Debug, Clone, PartialEq)]
pub enum ChooseError {
    /// The preferred method is not among the ones the server declared.
    NotOffered { wanted: Method, supported: Vec<Method> },
    /// A browser login was asked for, but the server declared nothing — there
    /// is no login service to send the user to.
    BrowserUndeclared,
    /// The server declared only methods this client does not know.
    NoUsableMethod,
}

impl std::fmt::Display for ChooseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChooseError::NotOffered { wanted, supported } => {
                let list: Vec<&str> = supported.iter().map(|m| m.as_str()).collect();
                write!(
                    f,
                    "this server does not support {wanted} login — it supports: {}",
                    list.join(", ")
                )
            }
            ChooseError::BrowserUndeclared => f.write_str(
                "this server does not declare browser login — it publishes no login service to sign in at",
            ),
            ChooseError::NoUsableMethod => f.write_str(
                "this server declares no login method this speclink supports — update speclink",
            ),
        }
    }
}

impl std::error::Error for ChooseError {}

/// Pick the login to run (D2). `preference` is what the user asked for, if
/// anything.
pub fn choose(discovery: &Discovery, preference: Option<Method>) -> Result<Plan, ChooseError> {
    let (methods, browser) = match discovery {
        Discovery::Undeclared => {
            return match preference {
                Some(Method::Browser) => Err(ChooseError::BrowserUndeclared),
                Some(Method::Token) => Ok(Plan::Token),
                Some(Method::Device) | None => Ok(Plan::Device),
            };
        }
        Discovery::Declared { methods, browser } => (methods, browser),
    };

    let method = match preference {
        Some(wanted) if methods.contains(&wanted) => wanted,
        Some(wanted) => {
            return Err(ChooseError::NotOffered { wanted, supported: methods.clone() });
        }
        None => Method::PREFERENCE
            .into_iter()
            .find(|m| methods.contains(m))
            .ok_or(ChooseError::NoUsableMethod)?,
    };
    Ok(match method {
        // `from_response` keeps Browser in `methods` only with its settings.
        Method::Browser => Plan::Browser(browser.clone().ok_or(ChooseError::NoUsableMethod)?),
        Method::Device => Plan::Device,
        Method::Token => Plan::Token,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin_with_reply(status: u16, body: &'static str) -> String {
        let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
        let addr = server.server_addr().to_ip().unwrap();
        std::thread::spawn(move || {
            let request = server.recv().unwrap();
            assert_eq!(request.url(), "/api/speclink/v1/auth-methods");
            assert!(!request.headers().iter().any(|header| header.field.equiv("Authorization")));
            request.respond(tiny_http::Response::from_string(body).with_status_code(status)).unwrap();
        });
        format!("http://{addr}")
    }

    #[test]
    fn discovery_respects_client_preference_and_older_servers() {
        let declared = discover(&origin_with_reply(200, r#"{"methods":["token","device","browser"],"browser":{"issuer":"https://login.example.test","clientId":"speclink"}}"#)).unwrap();
        assert!(matches!(choose(&declared, None).unwrap(), Plan::Browser(_)));
        assert_eq!(choose(&declared, Some(Method::Device)).unwrap(), Plan::Device);
        let old = discover(&origin_with_reply(404, "no route")).unwrap();
        assert_eq!(choose(&old, None).unwrap(), Plan::Device);
    }

    #[test]
    fn a_service_failure_never_falls_back_to_device_or_token() {
        let error = discover(&origin_with_reply(503, r#"{"reason":"unavailable","message":"temporarily down"}"#)).unwrap_err();
        assert_eq!(error.status, Some(503));
        assert!(error.message.contains("server unavailable"));
    }
}
