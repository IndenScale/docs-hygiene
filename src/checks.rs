use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::config::{Config, DocumentProfileConfig, FilenamePatternConfig, MaturityLevel};
use crate::report::{Report, Severity};

#[derive(Debug)]
pub struct Diagnostic {
    pub source: &'static str,
    pub code: &'static str,
    pub severity: Severity,
    pub path: String,
    pub range: DiagnosticRange,
    pub message: String,
    pub related_information: Vec<RelatedInformation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticRange {
    pub start: DiagnosticPosition,
    pub end: DiagnosticPosition,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug)]
pub struct RelatedInformation {
    pub path: String,
    pub range: DiagnosticRange,
    pub message: String,
}

impl Diagnostic {
    fn new(
        code: &'static str,
        severity: Severity,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source: "docs-hygiene",
            code,
            severity,
            path: path.into(),
            range: DiagnosticRange::origin(),
            message: message.into(),
            related_information: Vec::new(),
        }
    }

    fn with_source(mut self, source: &'static str) -> Self {
        self.source = source;
        self
    }

    fn at_line(mut self, line: usize) -> Self {
        self.range = DiagnosticRange::line(line);
        self
    }

    fn with_related(mut self, related: RelatedInformation) -> Self {
        self.related_information.push(related);
        self
    }
}

impl DiagnosticRange {
    fn origin() -> Self {
        Self {
            start: DiagnosticPosition {
                line: 0,
                character: 0,
            },
            end: DiagnosticPosition {
                line: 0,
                character: 0,
            },
        }
    }

    fn line(line: usize) -> Self {
        let zero_based = line.saturating_sub(1);
        Self {
            start: DiagnosticPosition {
                line: zero_based,
                character: 0,
            },
            end: DiagnosticPosition {
                line: zero_based,
                character: 0,
            },
        }
    }
}

impl RelatedInformation {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            range: DiagnosticRange::origin(),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
struct DocFile {
    base_id: String,
    rel: PathBuf,
    lang: Option<String>,
    number: Option<u32>,
    stem: String,
    numbered: bool,
}

struct NormalizedBase {
    id: String,
    root: PathBuf,
    patterns: Vec<FilenamePatternConfig>,
    require_continuous_numbering: bool,
    max_lines: Option<usize>,
    ignore: Vec<String>,
}

pub fn run_checks(root: &Path, config: &Config) -> Result<Report> {
    let ignore = build_ignore(root, config)?;
    let mut diagnostics = Vec::new();

    check_required_files(root, config, &mut diagnostics);
    let docs = collect_docs(root, config, &ignore, &mut diagnostics)?;
    check_numbering(config, &docs, &mut diagnostics);
    check_i18n(config, &docs, &mut diagnostics);
    check_max_lines(root, config, &docs, &mut diagnostics)?;
    check_ascii_art(root, config, &docs, &mut diagnostics)?;
    check_language(root, config, &docs, &mut diagnostics)?;
    check_document_contracts(root, config, &ignore, &docs, &mut diagnostics)?;
    check_concepts(root, config, &docs, &ignore, &mut diagnostics)?;
    check_adapters(root, config, &mut diagnostics)?;
    let diagnostics = apply_suppressions(config, diagnostics)?;

    Ok(Report::new(diagnostics, docs.len(), root))
}

fn check_document_contracts(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    managed_docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if config.document_contracts.profiles.is_empty()
        && config
            .document_contracts
            .maturity
            .recommendations
            .is_empty()
    {
        return Ok(());
    }

    check_maturity_recommendation(root, config, ignore, managed_docs.len(), diagnostics)?;
    let declared = config.document_contracts.maturity.declared;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || entry.file_name() != ".git")
    {
        let entry = entry?;
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
        {
            continue;
        }
        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
        if ignore.is_match(rel) {
            continue;
        }
        let Some(profile) = matching_document_profile(rel, &config.document_contracts.profiles)?
        else {
            continue;
        };
        check_document_contract(root, rel, profile, declared, diagnostics)?;
    }
    Ok(())
}

fn matching_document_profile<'a>(
    rel: &Path,
    profiles: &'a [DocumentProfileConfig],
) -> Result<Option<&'a DocumentProfileConfig>> {
    let filename = rel
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    for profile in profiles {
        let matcher = &profile.matcher;
        if matcher.paths.is_empty() && matcher.filenames.is_empty() {
            continue;
        }
        let path_matches = if matcher.paths.is_empty() {
            true
        } else {
            let mut builder = GlobSetBuilder::new();
            for path in &matcher.paths {
                builder.add(Glob::new(path)?);
            }
            builder.build()?.is_match(rel)
        };
        let filename_matches = if matcher.filenames.is_empty() {
            true
        } else {
            matcher
                .filenames
                .iter()
                .map(|pattern| Regex::new(pattern))
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .any(|pattern| pattern.is_match(filename))
        };
        if path_matches && filename_matches {
            return Ok(Some(profile));
        }
    }
    Ok(None)
}

fn check_document_contract(
    root: &Path,
    rel: &Path,
    profile: &DocumentProfileConfig,
    declared: MaturityLevel,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let text = std::fs::read_to_string(root.join(rel))?;
    let surface = strip_code_blocks(&text);
    let headings = markdown_headings(&surface);
    let severity = contract_severity(declared, profile.enforce_from);
    let mut matched_lines = Vec::new();

    for section in &profile.required_sections {
        let matched = headings.iter().find(|heading| {
            section
                .headings
                .iter()
                .any(|candidate| candidate == &heading.title)
        });
        let Some(heading) = matched else {
            diagnostics.push(Diagnostic::new(
                "DH_CONTRACT_001",
                severity,
                rel.display().to_string(),
                format!(
                    "Document profile '{}' requires section '{}' (accepted headings: {}).",
                    profile.id,
                    section.id,
                    section.headings.join(", ")
                ),
            ));
            continue;
        };
        matched_lines.push((section.id.as_str(), heading.line));

        if !profile.placeholder_patterns.is_empty()
            && section_contains_placeholder(
                &surface,
                &headings,
                heading.line,
                &profile.placeholder_patterns,
            )?
        {
            let placeholder_severity = if declared > profile.placeholders_allowed_until {
                Severity::Error
            } else {
                Severity::Info
            };
            diagnostics.push(
                Diagnostic::new(
                    "DH_CONTRACT_003",
                    placeholder_severity,
                    rel.display().to_string(),
                    format!(
                        "Required section '{}' in profile '{}' still contains a declared placeholder.",
                        section.id, profile.id
                    ),
                )
                .at_line(heading.line),
            );
        }
    }

    if profile.ordered_sections && matched_lines.windows(2).any(|pair| pair[0].1 >= pair[1].1) {
        diagnostics.push(Diagnostic::new(
            "DH_CONTRACT_004",
            severity,
            rel.display().to_string(),
            format!(
                "Required sections for document profile '{}' are not in configured order.",
                profile.id
            ),
        ));
    }

    for field in &profile.required_fields {
        if !Regex::new(&field.pattern)?.is_match(&surface) {
            diagnostics.push(Diagnostic::new(
                "DH_CONTRACT_002",
                severity,
                rel.display().to_string(),
                format!(
                    "Document profile '{}' requires field '{}'.",
                    profile.id, field.id
                ),
            ));
        }
    }
    Ok(())
}

#[derive(Debug)]
struct MarkdownHeading {
    line: usize,
    title: String,
}

fn markdown_headings(text: &str) -> Vec<MarkdownHeading> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_start();
            let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
            if !(1..=6).contains(&hashes) || !trimmed[hashes..].starts_with(' ') {
                return None;
            }
            let title = trimmed[hashes..]
                .trim()
                .trim_end_matches('#')
                .trim()
                .to_string();
            Some(MarkdownHeading {
                line: index + 1,
                title,
            })
        })
        .collect()
}

fn section_contains_placeholder(
    text: &str,
    headings: &[MarkdownHeading],
    heading_line: usize,
    patterns: &[String],
) -> Result<bool> {
    let end_line = headings
        .iter()
        .find(|heading| heading.line > heading_line)
        .map(|heading| heading.line)
        .unwrap_or_else(|| text.lines().count() + 1);
    let body = text
        .lines()
        .skip(heading_line)
        .take(end_line.saturating_sub(heading_line + 1))
        .collect::<Vec<_>>()
        .join("\n");
    for pattern in patterns {
        if Regex::new(pattern)?.is_match(&body) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn contract_severity(declared: MaturityLevel, enforce_from: MaturityLevel) -> Severity {
    if declared >= enforce_from {
        Severity::Error
    } else {
        Severity::Warning
    }
}

fn check_maturity_recommendation(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    managed_documents: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let declared = config.document_contracts.maturity.declared;
    let mut repository_lines = 0usize;
    let mut repository_bytes = 0u64;
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || entry.file_name() != ".git")
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
        if ignore.is_match(rel) {
            continue;
        }
        repository_bytes = repository_bytes.saturating_add(entry.metadata()?.len());
        if let Ok(text) = std::fs::read_to_string(entry.path()) {
            repository_lines = repository_lines.saturating_add(text.lines().count());
        }
    }

    let recommended = config
        .document_contracts
        .maturity
        .recommendations
        .iter()
        .filter(|recommendation| recommendation.level > declared)
        .filter(|recommendation| {
            let has_signal = recommendation.min_repository_lines.is_some()
                || recommendation.min_repository_bytes.is_some()
                || recommendation.min_managed_documents.is_some();
            has_signal
                && recommendation
                    .min_repository_lines
                    .is_none_or(|minimum| repository_lines >= minimum)
                && recommendation
                    .min_repository_bytes
                    .is_none_or(|minimum| repository_bytes >= minimum)
                && recommendation
                    .min_managed_documents
                    .is_none_or(|minimum| managed_documents >= minimum)
        })
        .map(|recommendation| recommendation.level)
        .max();

    if let Some(level) = recommended {
        diagnostics.push(Diagnostic::new(
            "DH_MATURITY_001",
            Severity::Info,
            ".",
            format!(
                "Repository signals recommend document governance maturity {:?}; declared {:?} ({} lines, {} bytes, {} managed docs).",
                level, declared, repository_lines, repository_bytes, managed_documents
            ),
        ));
    }
    Ok(())
}

fn check_required_files(root: &Path, config: &Config, diagnostics: &mut Vec<Diagnostic>) {
    let mut required = config.entry_docs.required.clone();
    required.extend(config.required_files.iter().cloned());
    for file in &required {
        if !root.join(file).exists() {
            diagnostics.push(Diagnostic::new(
                "DH_REQUIRED_001",
                Severity::Error,
                file.display().to_string(),
                "Required documentation file is missing.",
            ));
        }
    }
}

fn collect_docs(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<DocFile>> {
    let bases = normalized_bases(config);
    let lang_set: BTreeSet<_> = config.i18n.languages.iter().cloned().collect();
    let mut docs = Vec::new();

    for base in bases {
        let docs_root = root.join(&base.root);
        if !docs_root.exists() {
            continue;
        }
        let patterns = compile_patterns(&base.patterns)?;
        let base_ignore = build_base_ignore(&base)?;
        for entry in WalkDir::new(&docs_root) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let rel = path.strip_prefix(root).unwrap_or(path);
            if ignore.is_match(rel)
                || base_ignore.is_match(rel)
                || path.extension().and_then(|value| value.to_str()) != Some("md")
            {
                continue;
            }

            let file_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("");
            let Some(pattern) = matching_pattern(file_name, &patterns) else {
                diagnostics.push(Diagnostic::new(
                    "DH_NAME_001",
                    Severity::Error,
                    rel.display().to_string(),
                    format!("File name does not match any pattern for base {}.", base.id),
                ));
                continue;
            };

            let parent = path.parent().unwrap_or(&docs_root);
            let lang = parent
                .strip_prefix(&docs_root)
                .ok()
                .and_then(|value| value.components().next())
                .and_then(|value| value.as_os_str().to_str())
                .filter(|value| lang_set.contains(*value))
                .map(|value| value.to_string());
            let (number, stem) = numbered_parts(file_name);
            docs.push(DocFile {
                base_id: base.id.clone(),
                rel: rel.to_path_buf(),
                lang,
                number,
                stem,
                numbered: pattern.numbered,
            });
        }
    }

    Ok(docs)
}

fn check_numbering(config: &Config, docs: &[DocFile], diagnostics: &mut Vec<Diagnostic>) {
    for base in normalized_bases(config) {
        if !base.require_continuous_numbering {
            continue;
        }
        let base_docs = docs
            .iter()
            .filter(|doc| doc.base_id == base.id && doc.numbered)
            .collect::<Vec<_>>();
        for (lang, group) in grouped_docs(&base_docs) {
            let mut seen = BTreeMap::<u32, Vec<&DocFile>>::new();
            for doc in group {
                if let Some(number) = doc.number {
                    seen.entry(number).or_default().push(doc);
                }
            }

            for (number, files) in &seen {
                if files.len() > 1 {
                    let diagnostic = files.iter().fold(
                        Diagnostic::new(
                            "DH_SEQ_002",
                            Severity::Error,
                            group_label(&base.id, &lang),
                            format!("Duplicate document number {number:02}."),
                        ),
                        |diagnostic, file| {
                            diagnostic.with_related(RelatedInformation::new(
                                file.rel.display().to_string(),
                                format!("Uses duplicate number {number:02}."),
                            ))
                        },
                    );
                    diagnostics.push(diagnostic);
                }
            }

            let Some(max) = seen.keys().next_back().copied() else {
                continue;
            };
            for number in 1..=max {
                if !seen.contains_key(&number) {
                    diagnostics.push(Diagnostic::new(
                        "DH_SEQ_001",
                        Severity::Error,
                        group_label(&base.id, &lang),
                        format!("Missing document number {number:02}."),
                    ));
                }
            }
        }
    }
}

fn check_i18n(config: &Config, docs: &[DocFile], diagnostics: &mut Vec<Diagnostic>) {
    if !config.i18n.require_docs_parity && !config.i18n.require_number_parity {
        return;
    }
    let Some(root_lang) = config.i18n.root_lang.as_ref() else {
        return;
    };

    let mut by_lang = BTreeMap::<String, BTreeSet<(u32, String)>>::new();
    for doc in docs {
        if !doc.numbered {
            continue;
        }
        let lang = doc.lang.clone().unwrap_or_else(|| root_lang.clone());
        let Some(number) = doc.number else {
            continue;
        };
        by_lang
            .entry(lang)
            .or_default()
            .insert((number, doc.stem.clone()));
    }

    let root_docs = by_lang.get(root_lang).cloned().unwrap_or_default();
    for lang in &config.i18n.languages {
        let localized = by_lang.get(lang).cloned().unwrap_or_default();
        for key in &root_docs {
            if !localized.contains(key) {
                let root_path = root_doc_path(docs, root_lang, key);
                let mut diagnostic = Diagnostic::new(
                    "DH_I18N_001",
                    Severity::Error,
                    lang.to_string(),
                    format!(
                        "Missing localized counterpart for {:02}_{}.md.",
                        key.0, key.1
                    ),
                );
                if let Some(path) = root_path {
                    diagnostic = diagnostic.with_related(RelatedInformation::new(
                        path,
                        "Root document that requires localization.",
                    ));
                }
                diagnostics.push(diagnostic);
            }
        }
        for key in &localized {
            if !root_docs.contains(key) {
                let path = localized_doc_path(docs, lang, key).unwrap_or_else(|| lang.to_string());
                diagnostics.push(Diagnostic::new(
                    "DH_I18N_002",
                    Severity::Warning,
                    path,
                    format!(
                        "Localized document has no root counterpart: {:02}_{}.md.",
                        key.0, key.1
                    ),
                ));
            }
        }
    }
}

fn check_max_lines(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let base_limits = normalized_bases(config)
        .into_iter()
        .map(|base| (base.id, base.max_lines))
        .collect::<BTreeMap<_, _>>();
    for doc in docs {
        let Some(max_lines) = base_limits.get(&doc.base_id).and_then(|value| *value) else {
            continue;
        };
        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let lines = text.lines().count();
        if lines > max_lines {
            diagnostics.push(Diagnostic::new(
                "DH_SIZE_001",
                Severity::Warning,
                doc.rel.display().to_string(),
                format!("Document has {lines} lines, exceeding maxLines {max_lines}."),
            ));
        }
    }
    Ok(())
}

fn check_ascii_art(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if !config.docs.forbid_ascii_art {
        return Ok(());
    }

    for doc in docs {
        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let surface = ascii_art_surface(&text);
        let lines = surface.lines().collect::<Vec<_>>();
        let mut index = 0;
        while index < lines.len() {
            if !is_ascii_art_line(lines[index]) {
                index += 1;
                continue;
            }

            let start = index;
            let mut end = index + 1;
            while end < lines.len() && is_ascii_art_line(lines[end]) {
                end += 1;
            }
            let block = &lines[start..end];
            if block.len() >= 2 && block.iter().any(|line| is_strong_ascii_art_line(line)) {
                diagnostics.push(
                    Diagnostic::new(
                        "DH_ASCII_001",
                        Severity::Error,
                        doc.rel.display().to_string(),
                        "ASCII art is forbidden in documentation; use Markdown structure or an image instead.",
                    )
                    .at_line(start + 1),
                );
            }
            index = end;
        }
    }

    Ok(())
}

fn is_ascii_art_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return false;
    }
    if trimmed.starts_with("- ") {
        return false;
    }

    // Do not treat ordinary Markdown tables as diagrams.
    let table_cells = trimmed
        .strip_prefix('|')
        .and_then(|value| value.strip_suffix('|'))
        .map(|value| {
            value
                .split('|')
                .filter(|cell| !cell.trim().is_empty())
                .count()
        })
        .unwrap_or(0);
    if table_cells >= 2 && !trimmed.contains("->") && !trimmed.contains("<-") {
        return false;
    }

    let art_count = trimmed
        .chars()
        .filter(|ch| "+-|=/_\\<>[]{}()#*.:".contains(*ch))
        .count();
    if art_count < 2 {
        return false;
    }

    let has_alphanumeric = trimmed.chars().any(|ch| ch.is_ascii_alphanumeric());
    if !has_alphanumeric {
        // Horizontal rules are Markdown, not ASCII art.
        return !trimmed.chars().all(|ch| matches!(ch, '-' | '*' | '_'));
    }

    trimmed.contains('|')
        || trimmed.contains("->")
        || trimmed.contains("<-")
        || trimmed
            .chars()
            .next()
            .is_some_and(|ch| matches!(ch, '+' | '[' | '(' | '/' | '\\'))
        || trimmed
            .chars()
            .next_back()
            .is_some_and(|ch| matches!(ch, '+' | ']' | ')' | '/' | '\\'))
}

fn is_strong_ascii_art_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.chars().any(|ch| ch.is_ascii_alphanumeric())
        || trimmed.contains("->")
        || trimmed.contains("<-")
        || trimmed.starts_with('+')
        || trimmed.ends_with('+')
}

fn check_language(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let Some(root_lang) = config.i18n.root_lang.as_ref() else {
        return Ok(());
    };
    if config.language.is_empty() {
        return Ok(());
    }

    for doc in docs {
        let lang = doc.lang.as_ref().unwrap_or(root_lang);
        let Some(rule) = config.language.get(lang) else {
            continue;
        };
        let text = strip_code_blocks(&std::fs::read_to_string(root.join(&doc.rel))?);
        let ratio = cjk_ratio(&text);
        if let Some(min) = rule.min_cjk_ratio {
            if ratio < min {
                diagnostics.push(Diagnostic::new(
                    "DH_LANG_001",
                    Severity::Warning,
                    doc.rel.display().to_string(),
                    format!(
                        "CJK ratio {:.3} is below configured minCjkRatio {:.3} for {lang}.",
                        ratio, min
                    ),
                ));
            }
        }
        if let Some(max) = rule.max_cjk_ratio {
            if ratio > max {
                diagnostics.push(Diagnostic::new(
                    "DH_LANG_002",
                    Severity::Warning,
                    doc.rel.display().to_string(),
                    format!(
                        "CJK ratio {:.3} is above configured maxCjkRatio {:.3} for {lang}.",
                        ratio, max
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn check_concepts(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    ignore: &GlobSet,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if !config.concepts.require_concept_file {
        return Ok(());
    }

    let concept_dir = root.join(&config.concepts.dir);
    let mut concept_names = BTreeSet::new();
    if concept_dir.exists() {
        for entry in WalkDir::new(&concept_dir) {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
            {
                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
                if ignore.is_match(rel) {
                    continue;
                }
                if let Some(stem) = entry.path().file_stem().and_then(|value| value.to_str()) {
                    concept_names.insert(stem.to_string());
                }
            }
        }
    }

    let bold = Regex::new(r"\*\*(?P<term>[^*\n][^*\n]{1,80})\*\*")?;
    let mut referenced = BTreeSet::new();
    for doc in docs {
        let text = strip_code_blocks(&std::fs::read_to_string(root.join(&doc.rel))?);
        for (idx, line) in text.lines().enumerate() {
            for captures in bold.captures_iter(line) {
                let term = captures["term"].trim();
                if is_probable_concept(term) {
                    referenced.insert(term.to_string());
                    if !concept_names.contains(term) {
                        diagnostics.push(
                            Diagnostic::new(
                                "DH_CONCEPT_001",
                                Severity::Error,
                                doc.rel.display().to_string(),
                                format!(
                                    "Term \"{term}\" is not defined in {}/{}.md.",
                                    config.concepts.dir.display(),
                                    term
                                ),
                            )
                            .at_line(idx + 1),
                        );
                    }
                }
            }
        }
    }

    if config.concepts.fail_on_orphan_concept.as_deref() == Some("warn") {
        for concept in concept_names.difference(&referenced) {
            diagnostics.push(Diagnostic::new(
                "DH_CONCEPT_002",
                Severity::Warning,
                format!("{}/{}.md", config.concepts.dir.display(), concept),
                "Concept file is not referenced by docs.",
            ));
        }
    }

    Ok(())
}

fn check_adapters(root: &Path, config: &Config, diagnostics: &mut Vec<Diagnostic>) -> Result<()> {
    let markdownlint = &config.adapters.markdownlint;
    if !markdownlint.enabled {
        return Ok(());
    }

    let command = markdownlint
        .command
        .as_deref()
        .unwrap_or("markdownlint-cli2");
    let output = Command::new(command)
        .args(&markdownlint.args)
        .current_dir(root)
        .output();

    let output = match output {
        Ok(output) => output,
        Err(error) => {
            diagnostics.push(
                Diagnostic::new(
                    "DH_ADAPTER_001",
                    Severity::Error,
                    ".",
                    format!("Failed to run markdownlint adapter `{command}`: {error}."),
                )
                .with_source("markdownlint"),
            );
            return Ok(());
        }
    };

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let details = [stdout.trim(), stderr.trim()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        diagnostics.push(
            Diagnostic::new(
                "DH_ADAPTER_001",
                Severity::Error,
                ".",
                format!("markdownlint adapter failed:\n{details}"),
            )
            .with_source("markdownlint"),
        );
    }

    Ok(())
}

fn apply_suppressions(config: &Config, diagnostics: Vec<Diagnostic>) -> Result<Vec<Diagnostic>> {
    let suppressions = config
        .suppressions
        .iter()
        .map(|suppression| {
            let mut builder = GlobSetBuilder::new();
            for path in &suppression.paths {
                builder.add(Glob::new(path)?);
            }
            Ok((suppression, builder.build()?))
        })
        .collect::<Result<Vec<_>, globset::Error>>()?;

    Ok(diagnostics
        .into_iter()
        .filter(|diagnostic| {
            !suppressions.iter().any(|(suppression, paths)| {
                let _reason = suppression.reason.as_deref();
                let code_matches = suppression.code == "*" || suppression.code == diagnostic.code;
                let path_matches =
                    suppression.paths.is_empty() || paths.is_match(Path::new(&diagnostic.path));
                code_matches && path_matches
            })
        })
        .collect())
}

fn grouped_docs<'a>(docs: &[&'a DocFile]) -> BTreeMap<Option<String>, Vec<&'a DocFile>> {
    let mut groups = BTreeMap::<Option<String>, Vec<&DocFile>>::new();
    for doc in docs {
        groups.entry(doc.lang.clone()).or_default().push(doc);
    }
    groups
}

fn group_label(base_id: &str, lang: &Option<String>) -> String {
    match lang.as_deref() {
        Some(lang) => format!("{base_id}:{lang}"),
        None => base_id.to_string(),
    }
}

fn root_doc_path(docs: &[DocFile], root_lang: &str, key: &(u32, String)) -> Option<String> {
    docs.iter()
        .find(|doc| {
            doc.numbered
                && doc.number == Some(key.0)
                && doc.stem == key.1
                && doc.lang.as_deref().unwrap_or(root_lang) == root_lang
        })
        .map(|doc| doc.rel.display().to_string())
}

fn localized_doc_path(docs: &[DocFile], lang: &str, key: &(u32, String)) -> Option<String> {
    docs.iter()
        .find(|doc| {
            doc.numbered
                && doc.number == Some(key.0)
                && doc.stem == key.1
                && doc.lang.as_deref() == Some(lang)
        })
        .map(|doc| doc.rel.display().to_string())
}

fn normalized_bases(config: &Config) -> Vec<NormalizedBase> {
    if !config.docs.bases.is_empty() {
        return config
            .docs
            .bases
            .iter()
            .map(|base| NormalizedBase {
                id: base.id.clone(),
                root: base.root.clone(),
                patterns: if base.patterns.is_empty() {
                    default_patterns(&config.docs.filename_pattern)
                } else {
                    base.patterns.clone()
                },
                require_continuous_numbering: base
                    .require_continuous_numbering
                    .unwrap_or(config.docs.require_continuous_numbering),
                max_lines: base.max_lines.or(config.docs.max_lines),
                ignore: base.ignore.clone(),
            })
            .collect();
    }

    vec![NormalizedBase {
        id: "default".to_string(),
        root: config.docs.root.clone(),
        patterns: default_patterns(&config.docs.filename_pattern),
        require_continuous_numbering: config.docs.require_continuous_numbering,
        max_lines: config.docs.max_lines,
        ignore: Vec::new(),
    }]
}

fn default_patterns(regex: &str) -> Vec<FilenamePatternConfig> {
    vec![FilenamePatternConfig {
        id: "numbered".to_string(),
        regex: regex.to_string(),
        role: "numbered".to_string(),
        numbered: true,
    }]
}

fn compile_patterns(
    patterns: &[FilenamePatternConfig],
) -> Result<Vec<(Regex, &FilenamePatternConfig)>> {
    patterns
        .iter()
        .map(|pattern| Ok((Regex::new(&pattern.regex)?, pattern)))
        .collect()
}

fn matching_pattern<'a>(
    file_name: &str,
    patterns: &'a [(Regex, &'a FilenamePatternConfig)],
) -> Option<&'a FilenamePatternConfig> {
    patterns
        .iter()
        .find_map(|(regex, pattern)| regex.is_match(file_name).then_some(*pattern))
}

fn numbered_parts(file_name: &str) -> (Option<u32>, String) {
    let numbered = Regex::new(r"^(?<number>\d{2})_(?<stem>.+)\.md$").unwrap();
    if let Some(captures) = numbered.captures(file_name) {
        return (
            captures["number"].parse().ok(),
            captures["stem"].to_string(),
        );
    }
    (
        None,
        file_name
            .strip_suffix(".md")
            .unwrap_or(file_name)
            .to_string(),
    )
}

fn build_ignore(root: &Path, config: &Config) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("target/**")?);
    builder.add(Glob::new(".git/**")?);
    for path in &config.ignore.paths {
        builder.add(Glob::new(path)?);
    }
    let _ = root;
    Ok(builder.build()?)
}

fn build_base_ignore(base: &NormalizedBase) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for path in &base.ignore {
        builder.add(Glob::new(path)?);
    }
    Ok(builder.build()?)
}

fn strip_code_blocks(text: &str) -> String {
    let mut in_code = false;
    let mut out = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            out.push('\n');
            continue;
        }
        if !in_code {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn ascii_art_surface(text: &str) -> String {
    let mut in_code = false;
    let mut include_code = false;
    let mut out = String::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(info) = trimmed.strip_prefix("```") {
            if !in_code {
                let language = info
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_ascii_lowercase();
                include_code = matches!(
                    language.as_str(),
                    "text" | "txt" | "ascii" | "diagram" | "plaintext"
                );
            }
            in_code = !in_code;
            out.push('\n');
            continue;
        }
        if !in_code || include_code {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn is_probable_concept(term: &str) -> bool {
    let trimmed = term.trim();
    if trimmed.len() < 2 || trimmed.len() > 80 {
        return false;
    }
    !trimmed.contains('/')
        && !trimmed.ends_with('.')
        && !trimmed.chars().all(|value| value.is_ascii_punctuation())
}

fn cjk_ratio(text: &str) -> f64 {
    let mut cjk = 0usize;
    let mut letters = 0usize;
    for ch in text.chars() {
        if ch.is_whitespace() || ch.is_ascii_punctuation() {
            continue;
        }
        if is_cjk(ch) {
            cjk += 1;
            letters += 1;
        } else if ch.is_alphabetic() {
            letters += 1;
        }
    }
    if letters == 0 {
        return 0.0;
    }
    cjk as f64 / letters as f64
}

fn is_cjk(ch: char) -> bool {
    matches!(
        ch as u32,
        0x4E00..=0x9FFF
            | 0x3400..=0x4DBF
            | 0x20000..=0x2A6DF
            | 0x2A700..=0x2B73F
            | 0x2B740..=0x2B81F
            | 0x2B820..=0x2CEAF
            | 0xF900..=0xFAFF
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn config() -> Config {
        serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
  maxLines: 20
i18n:
  rootLang: en
  languages: [zh]
  requireDocsParity: true
  requireNumberParity: true
concepts:
  dir: concept
  requireConceptFile: true
  failOnOrphanConcept: warn
"#,
        )
        .unwrap()
    }

    fn codes(report: &Report) -> Vec<&'static str> {
        report
            .diagnostics
            .iter()
            .map(|diag| diag.code)
            .collect::<Vec<_>>()
    }

    fn assert_has_code(report: &Report, code: &str) {
        assert!(
            report.diagnostics.iter().any(|diag| diag.code == code),
            "expected {code}, got {:?}",
            report.diagnostics
        );
    }

    #[test]
    fn detects_missing_required_number_and_concept() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(
            temp.path().join("docs/02_architecture.md"),
            "# Architecture\n\nUses **Event Sourcing**.\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();
        let codes = codes(&report);

        assert!(codes.contains(&"DH_REQUIRED_001"));
        assert!(codes.contains(&"DH_SEQ_001"));
        assert!(codes.contains(&"DH_I18N_001"));
        assert!(codes.contains(&"DH_CONCEPT_001"));
    }

    #[test]
    fn accepts_clean_numbered_i18n_docs_with_concept() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/zh")).unwrap();
        fs::create_dir_all(temp.path().join("concept")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\nUses **Event Sourcing**.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/zh/01_architecture.md"),
            "# 架构\n\n使用 **Event Sourcing**。\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("concept/Event Sourcing.md"),
            "# Event Sourcing\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn detects_ascii_art_when_enabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\n+---------+\n| Client  | ---> API\n+---------+\n",
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .expect("ASCII art diagnostic");
        assert_eq!(diagnostic.path, "docs/01_architecture.md");
        assert_eq!(diagnostic.range.start.line, 2);
    }

    #[test]
    fn detects_mixed_language_ascii_art_diagram() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            r#"# Architecture

Specification IR                         Assembly graph
需求 / 不变式 / 验收条件 / Closure target   对象 / 接口 / 关系 / 证据
                  \                       /
                   \--- compile(...) ----/
                              |
            diagnostics / metrics / Closure / acceptance
"#,
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .expect("mixed-language ASCII art diagnostic");
        assert_eq!(diagnostic.range.start.line, 4);
    }

    #[test]
    fn detects_ascii_art_in_text_fences_but_ignores_code_and_markdown_tables() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\n```text\n+-----+\n| API |\n+-----+\n```\n\n```python\n+-----+\n| API |\n+-----+\n```\n\n| A | B |\n|---|---|\n| C | D |\n",
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostics = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .collect::<Vec<_>>();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start.line, 3);
    }

    #[test]
    fn detects_invalid_filename_and_skips_dependent_doc_rules_for_that_file() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/intro.md"), "# Intro\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_NAME_001");
        assert!(!codes(&report).contains(&"DH_SEQ_001"));
    }

    #[test]
    fn ignores_non_markdown_files_under_docs() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/02_notes.txt"), "not markdown\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 1);
    }

    #[test]
    fn ignores_configured_directories() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/generated")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/generated/bad.md"), "# Generated\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
ignore:
  paths:
    - docs/generated/**
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 1);
    }

    #[test]
    fn root_markdown_files_are_allowed_when_not_declared() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("AGENTS.md"), "# Agent Notes\n").unwrap();
        fs::write(temp.path().join("CLAUDE.md"), "# Claude Notes\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn docs_base_denies_unknown_markdown_but_allows_index_pattern() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/INDEX.md"), "# Index\n").unwrap();
        fs::write(temp.path().join("docs/freeform.md"), "# Freeform\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          role: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          role: index
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_NAME_001");
        assert_eq!(
            report
                .diagnostics
                .iter()
                .filter(|diag| diag.code == "DH_NAME_001")
                .count(),
            1
        );
        assert!(!codes(&report).contains(&"DH_SEQ_001"));
    }

    #[test]
    fn multiple_bases_use_their_own_patterns() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/guide")).unwrap();
        fs::create_dir_all(temp.path().join("docs/adr")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/guide/01_intro.md"), "# Intro\n").unwrap();
        fs::write(
            temp.path().join("docs/adr/ADR-0001_record.md"),
            "# Record\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: guide
      root: docs/guide
      requireContinuousNumbering: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          role: numbered
          numbered: true
    - id: adr
      root: docs/adr
      patterns:
        - id: adr
          regex: "^ADR-\\d{4}_[a-z0-9_-]+\\.md$"
          role: freeform
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn base_ignore_does_not_hide_files_from_other_bases() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/records")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/records/0001-note.md"), "# Note\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      ignore:
        - docs/records/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          role: numbered
          numbered: true
    - id: records
      root: docs/records
      requireContinuousNumbering: false
      patterns:
        - id: record
          regex: "^\\d{4}-[a-z0-9_-]+\\.md$"
          role: record
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 2);
    }

    #[test]
    fn detects_duplicate_document_numbers() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/01_setup.md"), "# Setup\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_SEQ_002");
    }

    #[test]
    fn detects_docs_that_exceed_max_lines() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        let long_doc = (0..25)
            .map(|idx| format!("line {idx}"))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(temp.path().join("docs/01_long.md"), long_doc).unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_SIZE_001");
    }

    #[test]
    fn detects_language_threshold_violations_and_ignores_code_blocks() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\nEnglish text.\n\n```text\n大量中文大量中文大量中文\n```\n\n正文中文过多。\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
i18n:
  rootLang: en
language:
  en:
    maxCjkRatio: 0.05
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_LANG_002");
    }

    #[test]
    fn detects_orphan_concepts_when_enabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::create_dir_all(temp.path().join("concept")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("concept/Unused.md"), "# Unused\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_CONCEPT_002");
    }

    #[test]
    fn detects_missing_adapter_command() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
adapters:
  markdownlint:
    enabled: true
    command: definitely-not-a-real-docs-hygiene-test-command
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_ADAPTER_001");
    }

    #[test]
    fn suppresses_diagnostics_by_code_and_path() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
i18n:
  rootLang: en
language:
  en:
    maxCjkRatio: 0.05
suppressions:
  - code: DH_LANG_002
    paths:
      - docs/01_case.md
    reason: Test case intentionally includes Chinese text.
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn wildcard_suppression_is_limited_by_path() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/examples")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/examples/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
i18n:
  rootLang: en
language:
  en:
    maxCjkRatio: 0.05
suppressions:
  - code: "*"
    paths:
      - docs/examples/**
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();
        let lang_diagnostics = report
            .diagnostics
            .iter()
            .filter(|diag| diag.code == "DH_LANG_002")
            .collect::<Vec<_>>();

        assert_eq!(lang_diagnostics.len(), 1, "{:?}", report.diagnostics);
        assert_eq!(lang_diagnostics[0].path, "docs/01_case.md");
    }

    #[test]
    fn path_and_filename_infer_contract_with_localized_heading_aliases() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/decisions")).unwrap();
        fs::write(
            temp.path().join("docs/decisions/0001-record-contracts.md"),
            "# 记录文档契约\n\n## 上下文\n\n背景。\n\n## 决策\n\n采用路径推导。\n\n## 后果\n\n保持开放扩展。\n\n## 实施说明\n\n额外章节合法。\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
docs:
  bases:
    - id: decisions
      root: docs/decisions
      patterns:
        - id: adr
          regex: "^\\d{4}-[a-z0-9-]+\\.md$"
          role: adr
documentContracts:
  maturity:
    declared: maintained
  profiles:
    - id: adr
      match:
        paths: ["docs/**/decisions/*.md"]
        filenames: ["^\\d{4}-[a-z0-9-]+\\.md$"]
      orderedSections: true
      requiredSections:
        - id: context
          headings: [Context, 上下文]
        - id: decision
          headings: [Decision, 决策]
        - id: consequences
          headings: [Consequences, 后果]
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn contract_reports_missing_sections_fields_order_and_mature_placeholders() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("ROADMAP.md"),
            "# Roadmap\n\n## 验收\n\nTODO\n\n## 目标\n\n形成稳定入口。\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: governed
  profiles:
    - id: roadmap
      match:
        paths: [ROADMAP.md]
        filenames: ["^ROADMAP\\.md$"]
      enforceFrom: maintained
      placeholdersAllowedUntil: growing
      placeholderPatterns: ["(?i)\\bTODO\\b"]
      orderedSections: true
      requiredSections:
        - id: goal
          headings: [目标]
        - id: acceptance
          headings: [验收]
        - id: exit
          headings: [退出条件]
      requiredFields:
        - id: owner
          pattern: "(?m)^负责人："
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        for code in [
            "DH_CONTRACT_001",
            "DH_CONTRACT_002",
            "DH_CONTRACT_003",
            "DH_CONTRACT_004",
        ] {
            assert_has_code(&report, code);
        }
    }

    #[test]
    fn repository_signals_recommend_but_do_not_force_maturity() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("README.md"),
            "# Project\n\nA small project.\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: seed
    recommendations:
      - level: growing
        minRepositoryLines: 2
        minRepositoryBytes: 10
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_MATURITY_001");
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_MATURITY_001")
            .unwrap();
        assert!(matches!(diagnostic.severity, Severity::Info));
        assert_eq!(report.summary.warning_count, 0);
        assert_eq!(report.summary.info_count, 1);
    }
}
