use anyhow::Result;

use crate::PinUpdateReport;

pub fn print_text_pin_update(report: &PinUpdateReport) {
    for change in &report.changes {
        println!(
            "{} {} -> {} {}: {} -> {}",
            change.policy,
            change.source,
            change.target,
            change.source_path,
            change.old_digest.as_deref().unwrap_or("<missing>"),
            change.new_digest
        );
    }
    for blocked in &report.blocked {
        println!("blocked {}: {}", blocked.path, blocked.reason);
    }
    if report.changes.is_empty() && report.blocked.is_empty() {
        println!("Critical dependency pins are current.");
    } else if report.applied {
        println!("Applied critical dependency pin updates and audit records.");
    }
}

pub fn print_json_pin_update(report: &PinUpdateReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
