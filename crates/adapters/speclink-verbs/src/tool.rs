//! The Agent tool definition (design D9): one tool named `speclink` whose input
//! is verb text, with a description that lists every verb an engine command
//! stands for. The list is walked from the same clap definition `parse` uses,
//! so a verb added there reaches the description too.

use crate::families::ToCommand;
use crate::inputs::{CommandTarget, VerbInputs};

/// One tool for a host's Agent framework: its name, the text the model reads,
/// and the JSON Schema of its input.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: String,
    pub input_schema: serde_json::Value,
}

const USAGE: &str = "Run one speclink verb on this project's specs, changes and discussions.

`argv` is the verb text without the program name, one word per item: [\"list\", \"--json\"] or [\"new\", \"change\", \"add-auth\"].
A verb that writes content (an artifact, a discussion round, a stamp's scope) reads it from `stdin`: when the verb has a --stdin flag, put --stdin in argv and the content in `stdin`.
Add --json to get a structured result instead of text.
Add --help to a verb to get its arguments; the help text comes back as the error message.
A \"path\" in a result is the document's key in the store, not a file you can open. To read a document, use \"artifact cat\", \"show\" or \"discuss show\".
";

/// The `speclink` tool: argv and an optional stdin in, the verb list in the
/// description. Verbs that need a checkout are left out.
pub fn tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "speclink",
        description: tool_description(&[]).expect("excluding no verb matches nothing to refuse"),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "argv": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "The verb text without the program name, one word per item."
                },
                "stdin": {
                    "type": "string",
                    "description": "The content a verb reads with --stdin."
                }
            },
            "required": ["argv"],
            "additionalProperties": false
        }),
    }
}

/// The tool's description without the verbs `exclude` names. Each item is a
/// verb's words with one space between them (`review`, `discuss archive`) and
/// leaves out every listed verb whose words start with its words: `review`
/// leaves out `review show`, while `archive` keeps `discuss archive`. The
/// error names each item that leaves out no listed verb.
pub fn tool_description(exclude: &[String]) -> Result<String, String> {
    let excluded: Vec<Vec<String>> = exclude
        .iter()
        .map(|item| item.split(' ').map(str::to_string).collect())
        .collect();
    let mut matched = vec![false; exclude.len()];
    let mut tree = crate::parse::verb_tree();
    tree.build();
    let mut lines = Vec::new();
    for_each_leaf(&mut tree, &mut Vec::new(), &mut |words, leaf| {
        let Some(line) = verb_line(words, leaf) else {
            return;
        };
        let mut kept = true;
        for (item, hit) in excluded.iter().zip(matched.iter_mut()) {
            if words.starts_with(item)
                || (item.len() == 1 && (1..=words.len()).any(|end| words[..end].join("-") == item[0]))
            {
                *hit = true;
                kept = false;
            }
        }
        if kept {
            lines.push(line);
        }
    });
    let unmatched: Vec<String> = exclude
        .iter()
        .zip(&matched)
        .filter(|(_, hit)| !**hit)
        .map(|(item, _)| format!("'{item}'"))
        .collect();
    if !unmatched.is_empty() {
        return Err(format!(
            "exclude: {} matches no verb of the list (a verb that needs a checkout is not in it)",
            unmatched.join(", ")
        ));
    }
    Ok(format!("{USAGE}\nVerbs:\n{}", lines.join("\n")))
}

/// Call `visit` with each leaf verb under `cmd` and its words
/// (`discuss add-round`), in the tree's order. clap's own `help` is no verb.
fn for_each_leaf(
    cmd: &mut clap::Command,
    words: &mut Vec<String>,
    visit: &mut dyn FnMut(&[String], &mut clap::Command),
) {
    if !cmd.get_subcommands().any(|s| s.get_name() != "help") {
        visit(words, cmd);
        return;
    }
    for sub in cmd.get_subcommands_mut().filter(|s| s.get_name() != "help") {
        words.push(sub.get_name().to_string());
        for_each_leaf(sub, words, visit);
        words.pop();
    }
}

/// The description line of a leaf verb that converts to an engine command:
/// its usage and its one-line help. `None` for a leaf with no engine command.
fn verb_line(words: &[String], leaf: &mut clap::Command) -> Option<String> {
    minimal_argv_of(words, leaf)?;
    let usage = leaf.render_usage().to_string();
    let usage = usage
        .trim()
        .trim_start_matches("Usage:")
        .trim()
        .trim_start_matches("speclink ");
    // The stamps' `--stdin` is hidden from the CLI's help, which refuses it; a
    // host takes the scope JSON through it, so the model sees it.
    let hidden_stdin = leaf
        .get_arguments()
        .any(|a| a.get_long() == Some("stdin") && a.is_hide_set());
    let flag = if hidden_stdin { " --stdin" } else { "" };
    let about = leaf.get_about().map(|a| a.to_string()).unwrap_or_default();
    Some(format!("- {usage}{flag}: {about}"))
}

/// The shortest argv that converts this leaf to an engine command, or `None`
/// when the leaf has none: the verb words plus a placeholder for each
/// required argument, then the same with one optional flag (some verbs
/// convert only with one: content from `--stdin`, or `change landed`'s
/// `--commit` when the caller gives no HEAD). The placeholder is a valid
/// kebab-case name, so the conversion's name checks (a tool name) accept it.
fn minimal_argv_of(words: &[String], leaf: &clap::Command) -> Option<Vec<String>> {
    let mut argv = words.to_vec();
    let push = |argv: &mut Vec<String>, arg: &clap::Arg| {
        let value = arg
            .get_possible_values()
            .first()
            .map(|v| v.get_name().to_string())
            // An argument that takes `<key>=<value>` converts only with a pair.
            .unwrap_or_else(|| {
                if takes_a_pair(arg) {
                    "demo=demo"
                } else {
                    "demo"
                }
                .to_string()
            });
        match arg.get_long() {
            None => argv.push(value),
            Some(long) => {
                argv.push(format!("--{long}"));
                if arg.get_action().takes_values() {
                    argv.push(value);
                }
            }
        }
    };
    for arg in leaf.get_arguments().filter(|a| a.is_required_set()) {
        push(&mut argv, arg);
    }
    // A required group (`--before` or `--after`) asks for one of its
    // arguments: the first one stands for the group.
    for group in leaf.get_groups().filter(|g| g.is_required_set()) {
        let first = group.get_args().next();
        if let Some(arg) = first.and_then(|id| leaf.get_arguments().find(|a| a.get_id() == id)) {
            push(&mut argv, arg);
        }
    }
    if converts(&argv) {
        return Some(argv);
    }
    leaf.get_arguments()
        .filter(|a| !a.is_required_set() && !a.is_hide_set() && a.get_long().is_some())
        .find_map(|arg| {
            let mut with_flag = argv.clone();
            push(&mut with_flag, arg);
            converts(&with_flag).then_some(with_flag)
        })
}

fn takes_a_pair(arg: &clap::Arg) -> bool {
    arg.get_value_names()
        .is_some_and(|names| names.iter().any(|name| name.contains('=')))
}

fn converts(argv: &[String]) -> bool {
    match crate::parse::parse(argv) {
        Ok(parsed) => matches!(
            parsed.to_command(VerbInputs::default()),
            Ok(CommandTarget::Command(_))
        ),
        Err(_) => false,
    }
}
