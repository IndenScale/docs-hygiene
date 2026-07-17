use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use docs_hygiene::{
    Config, PinUpdateRequest, print_json_pin_update, print_text_pin_update, update_critical_pins,
};

use super::OutputFormat;

#[derive(Debug, Parser)]
pub(super) struct PinUpdateArgs {
    /// Project root containing the policy and governed dependencies.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Limit the plan to this critical dependency policy; repeat as needed.
    #[arg(long = "policy")]
    policies: Vec<String>,

    /// Limit the plan to this target identity; repeat as needed.
    #[arg(long = "target")]
    targets: Vec<String>,

    /// Audit actor accepting the new digest.
    #[arg(long)]
    actor: String,

    /// Audit reason for accepting the new digest.
    #[arg(long)]
    reason: String,

    /// Apply the complete valid plan; omission is read-only.
    #[arg(long)]
    apply: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

pub(super) fn update_pins(args: PinUpdateArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let config = Config::load(&config_path)?;
    let request = PinUpdateRequest {
        policies: args.policies,
        targets: args.targets,
        actor: args.actor,
        reason: args.reason,
        apply: args.apply,
    };
    let report = update_critical_pins(&root, &config, &request)?;
    match args.format {
        OutputFormat::Text => print_text_pin_update(&report),
        OutputFormat::Json => print_json_pin_update(&report)?,
    }
    if !report.blocked.is_empty() {
        anyhow::bail!("critical dependency pin update is blocked; no changes were applied");
    }
    Ok(())
}
