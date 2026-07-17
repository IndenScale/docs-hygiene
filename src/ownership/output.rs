use anyhow::Result;

use super::ReviewResetReport;

pub fn print_text_review_reset(report: &ReviewResetReport) {
    for change in &report.changes {
        println!(
            "Review reset {}: {} -> {} by {} ({}) [{}].",
            change.identity,
            change.previous_review_by.as_deref().unwrap_or("<missing>"),
            change.review_by,
            change.reset_by,
            change.reason,
            if report.applied { "applied" } else { "dry-run" },
        );
    }
    for blocked in &report.blocked {
        println!(
            "Blocked review reset {} at {}: {}.",
            blocked.identity, blocked.path, blocked.reason
        );
    }
}

pub fn print_json_review_reset(report: &ReviewResetReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
