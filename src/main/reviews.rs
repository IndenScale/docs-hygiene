use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use docs_hygiene::{
    Config, ReviewResetRequest, print_json_review_reset, print_text_review_reset,
    reset_governed_review,
};

use super::OutputFormat;

#[derive(Debug, Parser)]
pub(super) struct ReviewResetArgs {
    /// Stable governed identity whose review deadline should advance.
    identity: String,

    /// Project root containing the policy and governed identity.
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Active person principal recording the manual semantic review.
    #[arg(long)]
    actor: String,

    /// Audit reason for the manual review.
    #[arg(long)]
    reason: String,

    /// New review deadline in YYYY-MM-DD format.
    #[arg(long)]
    review_by: String,

    /// Apply the valid plan; omission is a read-only dry-run.
    #[arg(long)]
    apply: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

pub(super) fn reset_review(args: ReviewResetArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let config = Config::load(&config_path)?;
    let request = ReviewResetRequest {
        identity: args.identity,
        actor: args.actor,
        reason: args.reason,
        review_by: args.review_by,
        apply: args.apply,
    };
    let report = reset_governed_review(&root, &config, &request)?;
    match args.format {
        OutputFormat::Text => print_text_review_reset(&report),
        OutputFormat::Json => print_json_review_reset(&report)?,
    }
    if !report.blocked.is_empty() {
        anyhow::bail!("review reset is blocked; no changes were applied");
    }
    Ok(())
}
