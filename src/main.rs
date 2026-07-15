mod checks;
mod config;
mod report;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use crate::checks::run_checks;
use crate::config::Config;
use crate::report::{Report, print_json_report, print_text_report};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check repository documentation against docs-hygiene policy.
    Check(CheckArgs),
    /// Create a starter docs-hygiene.yml policy file.
    Init {
        /// Path to write.
        #[arg(long, default_value = "docs-hygiene.yml")]
        path: PathBuf,
    },
    /// Create a starter docs tree and policy files.
    Scaffold {
        /// Directory to scaffold.
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Overwrite files that already exist.
        #[arg(long)]
        force: bool,
    },
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

    /// Mark this language as the root language.
    #[arg(long)]
    root: bool,

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
    /// Repository root to check.
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

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or_else(default_command) {
        Command::Check(args) => check(args),
        Command::Init { path } => init(path),
        Command::Scaffold { path, force } => scaffold(path, force),
        Command::Lang { command } => lang(command),
        Command::Explain { code } => explain(&code),
    }
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

fn scaffold(path: PathBuf, force: bool) -> Result<()> {
    std::fs::create_dir_all(path.join("docs/zh"))
        .with_context(|| format!("failed to create {}", path.join("docs/zh").display()))?;
    std::fs::create_dir_all(path.join("concept"))
        .with_context(|| format!("failed to create {}", path.join("concept").display()))?;

    write_scaffold_file(
        &path.join("docs-hygiene.yml"),
        Config::starter_yaml(),
        force,
    )?;
    write_scaffold_file(&path.join(".markdownlint.yaml"), "MD013: false\n", force)?;
    write_scaffold_file(
        &path.join("README.md"),
        "# Project\n\nThis repository uses Docs Hygiene.\n",
        force,
    )?;
    write_scaffold_file(
        &path.join("README_ZH.md"),
        "# Project\n\n本仓库使用 Docs Hygiene。\n",
        force,
    )?;
    write_scaffold_file(&path.join("CHANGELOG.md"), "# Changelog\n", force)?;
    write_scaffold_file(&path.join("docs/01_overview.md"), "# Overview\n", force)?;
    write_scaffold_file(&path.join("docs/zh/01_overview.md"), "# 概览\n", force)?;
    write_scaffold_file(
        &path.join("concept/Policy Engine.md"),
        "# Policy Engine\n",
        force,
    )?;
    Ok(())
}

fn write_scaffold_file(path: &std::path::Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Ok(());
    }
    std::fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))
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
    let root = config.i18n.root_lang.as_deref().unwrap_or("-");
    println!("rootLang: {root}");
    for lang in &config.i18n.languages {
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
    if args.root {
        config.i18n.root_lang = Some(args.code.clone());
    } else if !config.i18n.languages.contains(&args.code) {
        config.i18n.languages.push(args.code.clone());
        config.i18n.languages.sort();
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
    config.i18n.languages.retain(|lang| lang != &args.code);
    config.language.remove(&args.code);
    if config.i18n.root_lang.as_deref() == Some(args.code.as_str()) {
        config.i18n.root_lang = None;
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
