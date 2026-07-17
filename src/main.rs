use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use docs_hygiene::{
    Config, Report, evaluate_hygiene_profile, evaluate_rule_activation, migrate_document_kinds,
    migrate_document_template_bindings, print_json_activation, print_json_kind_migration,
    print_json_profile, print_json_report, print_json_template_migration, print_text_activation,
    print_text_kind_migration, print_text_profile, print_text_report,
    print_text_template_migration, run_checks,
};

#[path = "main/scaffold.rs"]
mod scaffold;

use scaffold::{ScaffoldArgs, scaffold};

// Governance Library: [[SDK-001]]

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check project documentation against docs-hygiene policy.
    Check(CheckArgs),
    /// Explain which rule families are active and why.
    ExplainRules(ExplainRulesArgs),
    /// Evaluate the multidimensional documentation hygiene profile.
    Profile(ProfileArgs),
    /// Pin or advance compatible document-template revisions.
    MigrateTemplates(MigrateTemplatesArgs),
    /// Atomically migrate compatible Document Kind schemas and template pins.
    MigrateKinds(MigrateKindsArgs),
    /// Create a starter docs-hygiene.yml policy file.
    Init {
        /// Path to write.
        #[arg(long, default_value = "docs-hygiene.yml")]
        path: PathBuf,
    },
    /// Create a starter project or one configured Document Kind file.
    Scaffold(ScaffoldArgs),
    /// Manage language policy in docs-hygiene.yml.
    Lang {
        #[command(subcommand)]
        command: LangCommand,
    },
    /// Explain a docs-hygiene diagnostic code.
    Explain {
        /// Diagnostic code, such as DH_SEQ_001.
        code: String,
    },
}

#[derive(Debug, Subcommand)]
enum LangCommand {
    /// List configured languages and thresholds.
    List(ConfigArgs),
    /// Add a supported language.
    Add(LangAddArgs),
    /// Remove a supported language.
    Remove(LangRemoveArgs),
    /// Set CJK thresholds for a language.
    SetThreshold(LangSetThresholdArgs),
}

#[derive(Debug, Parser)]
struct ConfigArgs {
    /// Config file path.
    #[arg(long, default_value = "docs-hygiene.yml")]
    config: PathBuf,
}

#[derive(Debug, Parser)]
struct LangAddArgs {
    /// Language code, such as en, zh, or ja.
    code: String,

    /// Config file path.
    #[arg(long, default_value = "docs-hygiene.yml")]
    config: PathBuf,

    /// Mark this language as the canonical representation.
    #[arg(long)]
    canonical: bool,

    /// Minimum CJK ratio for this language.
    #[arg(long)]
    min_cjk_ratio: Option<f64>,

    /// Maximum CJK ratio for this language.
    #[arg(long)]
    max_cjk_ratio: Option<f64>,
}

#[derive(Debug, Parser)]
struct LangRemoveArgs {
    /// Language code to remove.
    code: String,

    /// Config file path.
    #[arg(long, default_value = "docs-hygiene.yml")]
    config: PathBuf,
}

#[derive(Debug, Parser)]
struct LangSetThresholdArgs {
    /// Language code to update.
    code: String,

    /// Config file path.
    #[arg(long, default_value = "docs-hygiene.yml")]
    config: PathBuf,

    /// Minimum CJK ratio for this language. Omit to keep current value.
    #[arg(long)]
    min_cjk_ratio: Option<f64>,

    /// Maximum CJK ratio for this language. Omit to keep current value.
    #[arg(long)]
    max_cjk_ratio: Option<f64>,

    /// Clear the minimum CJK ratio.
    #[arg(long)]
    clear_min: bool,

    /// Clear the maximum CJK ratio.
    #[arg(long)]
    clear_max: bool,
}

#[derive(Debug, Parser)]
struct CheckArgs {
    /// Project root to check.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the checked root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Exit with status 1 when diagnostics are present.
    #[arg(long)]
    fail_on_warning: bool,
}

#[derive(Debug, Parser)]
struct ExplainRulesArgs {
    /// Project root to inspect.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Debug, Parser)]
struct ProfileArgs {
    /// Project root to inspect.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Exit with status 1 when a required dimension is below its target.
    #[arg(long)]
    fail_below_target: bool,
}

#[derive(Debug, Parser)]
struct MigrateTemplatesArgs {
    /// Project root containing the policy file.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Report required changes without writing and fail if migration is needed.
    #[arg(long)]
    check: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Debug, Parser)]
struct MigrateKindsArgs {
    /// Project root containing the policy and typed documents.
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Config file path. Defaults to docs-hygiene.yml under the project root.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Report required changes without writing and fail if migration is needed.
    #[arg(long)]
    check: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or_else(default_command) {
        Command::Check(args) => check(args),
        Command::ExplainRules(args) => explain_rules(args),
        Command::Profile(args) => profile(args),
        Command::MigrateTemplates(args) => migrate_templates(args),
        Command::MigrateKinds(args) => migrate_kinds(args),
        Command::Init { path } => init(path),
        Command::Scaffold(args) => scaffold(args),
        Command::Lang { command } => lang(command),
        Command::Explain { code } => explain(&code),
    }
}

fn migrate_templates(args: MigrateTemplatesArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let mut config = Config::load(&config_path)?;
    let report = migrate_document_template_bindings(&mut config, !args.check);
    if !args.check && report.applied && !report.changes.is_empty() {
        config.save(&config_path)?;
    }
    match args.format {
        OutputFormat::Text => print_text_template_migration(&report),
        OutputFormat::Json => print_json_template_migration(&report)?,
    }
    if !report.blocked.is_empty() {
        anyhow::bail!("document-template migration is blocked");
    }
    if args.check && !report.changes.is_empty() {
        anyhow::bail!("document-template migration is required");
    }
    Ok(())
}

fn migrate_kinds(args: MigrateKindsArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let mut config = Config::load(&config_path)?;
    let report = migrate_document_kinds(&root, &mut config, !args.check)?;
    if !args.check && report.applied && !report.template_changes.is_empty() {
        config.save(&config_path)?;
    }
    match args.format {
        OutputFormat::Text => print_text_kind_migration(&report),
        OutputFormat::Json => print_json_kind_migration(&report)?,
    }
    if !report.blocked.is_empty() {
        anyhow::bail!("Document Kind migration is blocked; no changes were applied");
    }
    if args.check && (!report.schema_changes.is_empty() || !report.template_changes.is_empty()) {
        anyhow::bail!("Document Kind migration is required");
    }
    Ok(())
}

fn profile(args: ProfileArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let config = Config::load(&config_path)?;
    let report = evaluate_hygiene_profile(&root, &config)?;
    match args.format {
        OutputFormat::Text => print_text_profile(&report),
        OutputFormat::Json => print_json_profile(&report)?,
    }
    if args.fail_below_target && !report.meets_targets {
        anyhow::bail!("docs-hygiene profile is below a required target");
    }
    Ok(())
}

fn explain_rules(args: ExplainRulesArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let config = Config::load(&config_path)?;
    let report = evaluate_rule_activation(&root, &config)?;
    match args.format {
        OutputFormat::Text => print_text_activation(&report),
        OutputFormat::Json => print_json_activation(&report)?,
    }
    Ok(())
}

fn default_command() -> Command {
    Command::Check(CheckArgs {
        root: PathBuf::from("."),
        config: None,
        format: OutputFormat::Text,
        fail_on_warning: false,
    })
}

fn check(args: CheckArgs) -> Result<()> {
    let root = args.root.canonicalize()?;
    let config_path = args.config.unwrap_or_else(|| root.join("docs-hygiene.yml"));
    let config = Config::load(&config_path)?;
    let report = run_checks(&root, &config)?;

    match args.format {
        OutputFormat::Text => print_text_report(&report),
        OutputFormat::Json => print_json_report(&report)?,
    }

    if report.summary.error_count > 0 || (args.fail_on_warning && report.summary.warning_count > 0)
    {
        anyhow::bail!("docs-hygiene found diagnostics");
    }

    Ok(())
}

fn init(path: PathBuf) -> Result<()> {
    if path.exists() {
        anyhow::bail!("{} already exists", path.display());
    }
    std::fs::write(path, Config::starter_yaml())?;
    Ok(())
}

fn lang(command: LangCommand) -> Result<()> {
    match command {
        LangCommand::List(args) => lang_list(args.config),
        LangCommand::Add(args) => lang_add(args),
        LangCommand::Remove(args) => lang_remove(args),
        LangCommand::SetThreshold(args) => lang_set_threshold(args),
    }
}

fn lang_list(path: PathBuf) -> Result<()> {
    let config = Config::load(&path)?;
    let canonical = config
        .language_representations
        .canonical
        .as_deref()
        .unwrap_or("-");
    println!("canonical: {canonical}");
    for lang in &config.language_representations.localized {
        let threshold = config.language.get(lang);
        let min = threshold
            .and_then(|value| value.min_cjk_ratio)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        let max = threshold
            .and_then(|value| value.max_cjk_ratio)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!("{lang}\tminCjkRatio={min}\tmaxCjkRatio={max}");
    }
    Ok(())
}

fn lang_add(args: LangAddArgs) -> Result<()> {
    let mut config = Config::load(&args.config)?;
    if args.canonical {
        config.language_representations.canonical = Some(args.code.clone());
    } else if !config
        .language_representations
        .localized
        .contains(&args.code)
    {
        config
            .language_representations
            .localized
            .push(args.code.clone());
        config.language_representations.localized.sort();
    }
    let entry = config.language.entry(args.code).or_default();
    if args.min_cjk_ratio.is_some() {
        entry.min_cjk_ratio = args.min_cjk_ratio;
    }
    if args.max_cjk_ratio.is_some() {
        entry.max_cjk_ratio = args.max_cjk_ratio;
    }
    config.save(&args.config)
}

fn lang_remove(args: LangRemoveArgs) -> Result<()> {
    let mut config = Config::load(&args.config)?;
    config
        .language_representations
        .localized
        .retain(|lang| lang != &args.code);
    config.language.remove(&args.code);
    if config.language_representations.canonical.as_deref() == Some(args.code.as_str()) {
        config.language_representations.canonical = None;
    }
    config.save(&args.config)
}

fn lang_set_threshold(args: LangSetThresholdArgs) -> Result<()> {
    let mut config = Config::load(&args.config)?;
    let entry = config.language.entry(args.code).or_default();
    if args.clear_min {
        entry.min_cjk_ratio = None;
    }
    if args.clear_max {
        entry.max_cjk_ratio = None;
    }
    if args.min_cjk_ratio.is_some() {
        entry.min_cjk_ratio = args.min_cjk_ratio;
    }
    if args.max_cjk_ratio.is_some() {
        entry.max_cjk_ratio = args.max_cjk_ratio;
    }
    config.save(&args.config)
}

fn explain(code: &str) -> Result<()> {
    let explanation = Report::explain(code).unwrap_or("Unknown diagnostic code.");
    println!("{explanation}");
    Ok(())
}
