use anyhow::Result;

use super::ActivationReport;

pub fn print_text_activation(report: &ActivationReport) {
    println!("Rule                              State      Mode       Basis");
    for decision in &report.decisions {
        println!(
            "{:<33} {:<10} {:<10} {}",
            decision.rule,
            decision.state.label(),
            format!("{:?}", decision.mode).to_lowercase(),
            decision.evidence.join("; ")
        );
    }
    println!(
        "\nFacts: {} Markdown documents, {} Markdown lines, {} code lines, {} Manifests, {} localized documents.",
        report.facts.markdown_documents,
        report.facts.markdown_lines,
        report.facts.code_lines,
        report.facts.manifest_files,
        report.facts.localized_documents
    );
}

pub fn print_json_activation(report: &ActivationReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
