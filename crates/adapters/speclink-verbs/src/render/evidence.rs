//! Output of `evidence show`.

use serde_json::json;
use speclink_core::command::EvidenceShowOutcome;

use super::{JsonForm, Paint, RenderOptions, Rendered, Text};

pub fn show_human(o: &EvidenceShowOutcome, paint: &Paint) -> Text {
    let mut t = Text::default();
    let label = if o.archived { " (archived)" } else { "" };
    outln!(t, "{}: {}{}", paint.bold("Evidence"), o.change, label);
    if o.tasks.is_empty() {
        outln!(t, "  (none)");
    } else {
        for task in &o.tasks {
            outln!(t, "  {} {}: {}", paint.cyan("•"), task.task_id, task.files.join(", "));
        }
    }
    t
}

pub fn show(o: &EvidenceShowOutcome, options: RenderOptions) -> Rendered {
    Rendered::of(options, Some(JsonForm::pretty(&json!(o))), |paint| show_human(o, paint))
}
