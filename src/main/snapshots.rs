use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use docs_hygiene::{
    import_portable_snapshot, print_json_snapshot_import, print_text_snapshot_import,
};

use super::OutputFormat;

#[derive(Debug, Parser)]
pub(super) struct SnapshotImportArgs {
    /// Project root containing the registered snapshot manifest.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Project-relative portable snapshot manifest path.
    #[arg(long)]
    manifest: PathBuf,

    /// Explicit local Git checkout containing the declared commit.
    #[arg(long)]
    source: PathBuf,

    /// Apply the complete valid import; omission is read-only.
    #[arg(long)]
    apply: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

pub(super) fn import_snapshot(args: SnapshotImportArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let source = args.source.canonicalize()?;
    let report = import_portable_snapshot(&root, &args.manifest, &source, args.apply)?;
    match args.format {
        OutputFormat::Text => print_text_snapshot_import(&report),
        OutputFormat::Json => print_json_snapshot_import(&report)?,
    }
    if !report.blocked.is_empty() {
        anyhow::bail!("portable snapshot import is blocked; no payloads were written");
    }
    Ok(())
}
