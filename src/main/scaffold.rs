use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use docs_hygiene::{Config, ScaffoldDocumentRequest, plan_scaffold_document};

#[derive(Debug, Parser)]
pub(super) struct ScaffoldArgs {
    /// Project root to scaffold.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Generate one document from this configured Document Kind.
    #[arg(long)]
    kind: Option<String>,

    /// Stable identity supplied to the Kind Schema.
    #[arg(long, requires = "kind")]
    identity: Option<String>,

    /// Slug supplied to filename and frontmatter templates.
    #[arg(long, requires = "kind")]
    slug: Option<String>,

    /// Language representation; defaults to the configured canonical locale.
    #[arg(long, requires = "kind")]
    locale: Option<String>,

    /// Project-relative target directory overriding the Kind's configured root.
    #[arg(long, requires = "kind")]
    target: Option<PathBuf>,

    /// Typed frontmatter input as FIELD=VALUE; repeat for multiple fields.
    #[arg(long = "field", requires = "kind")]
    fields: Vec<String>,

    /// Print the generated path and content without writing.
    #[arg(long, requires = "kind")]
    dry_run: bool,

    /// Explicitly overwrite an existing generated file or starter file.
    #[arg(long)]
    force: bool,
}

pub(super) fn scaffold(args: ScaffoldArgs) -> Result<()> {
    if args.kind.is_some() {
        return scaffold_document(args);
    }
    scaffold_starter(args.path, args.force)
}

fn scaffold_starter(path: PathBuf, force: bool) -> Result<()> {
    std::fs::create_dir_all(path.join("docs/zh/guide"))
        .with_context(|| format!("failed to create {}", path.join("docs/zh/guide").display()))?;
    std::fs::create_dir_all(path.join("docs/guide"))
        .with_context(|| format!("failed to create {}", path.join("docs/guide").display()))?;
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
        "# Project\n\nThis project uses Docs Hygiene.\n",
        force,
    )?;
    write_scaffold_file(
        &path.join("README_ZH.md"),
        "# Project\n\n本项目使用 Docs Hygiene。\n",
        force,
    )?;
    write_scaffold_file(&path.join("CHANGELOG.md"), "# Changelog\n", force)?;
    write_scaffold_file(
        &path.join("docs/README.md"),
        "# Documentation\n\n- [Overview](guide/overview.md)\n",
        force,
    )?;
    write_scaffold_file(
        &path.join("docs/zh/README.md"),
        "# 文档\n\n- [概览](guide/overview.md)\n",
        force,
    )?;
    write_scaffold_file(&path.join("docs/guide/overview.md"), "# Overview\n", force)?;
    write_scaffold_file(&path.join("docs/zh/guide/overview.md"), "# 概览\n", force)?;
    write_scaffold_file(
        &path.join("concept/Policy Engine.md"),
        "# Policy Engine\n",
        force,
    )?;
    Ok(())
}

fn scaffold_document(args: ScaffoldArgs) -> Result<()> {
    let root = args.path.canonicalize()?;
    let config = Config::load(&root.join("docs-hygiene.yml"))?;
    let locale = args.locale.unwrap_or_else(|| {
        config
            .language_representations
            .canonical
            .clone()
            .unwrap_or_else(|| "canonical".to_owned())
    });
    let fields = parse_scaffold_fields(&args.fields)?;
    let request = ScaffoldDocumentRequest {
        kind: args.kind.expect("kind dispatch checked"),
        identity: args
            .identity
            .ok_or_else(|| anyhow::anyhow!("--identity is required with --kind"))?,
        slug: args
            .slug
            .ok_or_else(|| anyhow::anyhow!("--slug is required with --kind"))?,
        locale,
        target_dir: args.target,
        fields,
    };
    let plan = plan_scaffold_document(&config, &request)?;
    let destination = root.join(&plan.relative_path);
    if destination.exists() && !args.force {
        anyhow::bail!(
            "{} already exists; no files were written (use --force to overwrite explicitly)",
            plan.relative_path.display()
        );
    }
    if args.dry_run {
        println!("{}\n{}", plan.relative_path.display(), plan.content);
        return Ok(());
    }
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    std::fs::write(&destination, plan.content)
        .with_context(|| format!("failed to write {}", destination.display()))?;
    println!("created {}", plan.relative_path.display());
    Ok(())
}

fn parse_scaffold_fields(fields: &[String]) -> Result<BTreeMap<String, String>> {
    let mut parsed = BTreeMap::new();
    for field in fields {
        let Some((name, value)) = field.split_once('=') else {
            anyhow::bail!("--field must use FIELD=VALUE syntax: '{field}'");
        };
        if name.is_empty() {
            anyhow::bail!("--field name must not be empty");
        }
        if parsed.insert(name.to_owned(), value.to_owned()).is_some() {
            anyhow::bail!("--field '{name}' is supplied more than once");
        }
    }
    Ok(parsed)
}

fn write_scaffold_file(path: &std::path::Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Ok(());
    }
    std::fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))
}
