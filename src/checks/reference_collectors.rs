fn is_semantic_comment_line(line: &str) -> bool {
    let line = line.trim_start();
    ["//", "#", "--", "/*", "*", "<!--"]
        .iter()
        .any(|prefix| line.starts_with(prefix))
}

fn collect_governed_reference_occurrences(
    root: &Path,
    paths: &[PathBuf],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeSet<ReferenceOccurrence> {
    let mut occurrences = BTreeSet::new();
    for rel in paths {
        let text = match std::fs::read_to_string(root.join(rel)) {
            Ok(text) => text,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    rel.display().to_string(),
                    format!("Body content cannot be read for Wiki Link analysis: {error}."),
                ));
                continue;
            }
        };
        let is_markdown = rel.extension().and_then(|value| value.to_str()) == Some("md");
        occurrences.extend(collect_wiki_link_occurrences(
            rel,
            &text,
            is_markdown,
            diagnostics,
        ));
        if is_markdown {
            occurrences.extend(collect_markdown_link_occurrences(rel, &text));
            occurrences.extend(collect_frontmatter_occurrences(
                rel,
                &text,
                diagnostics,
            ));
        }
    }
    occurrences
}

fn collect_wiki_link_occurrences(
    rel: &Path,
    text: &str,
    is_markdown: bool,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeSet<ReferenceOccurrence> {
    let wiki_link = Regex::new(r"\[\[([^\]\n]+)\]\]").expect("static Wiki Link regex");
    let target = Regex::new(r"^([A-Za-z0-9._-]+)(?:#([a-z0-9]+(?:-[a-z0-9]+)*))?(?:@sha256:([A-Fa-f0-9]{64}))?(?:\|[^\]\n]+)?$")
        .expect("static Wiki Link target regex");
    let analyzed = if is_markdown {
        strip_markdown_code(text)
    } else {
        text.to_owned()
    };
    let mut occurrences = BTreeSet::new();
    for (line_index, line) in analyzed.lines().enumerate() {
        if !is_markdown && !is_semantic_comment_line(line) {
            continue;
        }
        for wiki_capture in wiki_link.captures_iter(line) {
            let Some(captures) = target.captures(&wiki_capture[1]) else {
                diagnostics.push(
                    Diagnostic::new(
                        "DH_REFERENCE_001",
                        Severity::Error,
                        rel.display().to_string(),
                        format!(
                            "Invalid semantic Wiki Link '[[{}]]'; expected [[ID]], [[ID#heading-slug|label]], or [[ID#heading-slug@sha256:<64-hex>|label]].",
                            &wiki_capture[1]
                        ),
                    )
                    .at_line(line_index + 1),
                );
                continue;
            };
            occurrences.insert(ReferenceOccurrence::new(
                captures[1].to_owned(),
                SYNTAX_WIKI_LINK,
                CONTEXT_GOVERNED_CONTENT,
                GovernanceLocation {
                    path: rel.display().to_string(),
                    line: Some(line_index + 1),
                },
                ReferencePayload {
                    selector: captures.get(2).map(|value| value.as_str().to_owned()),
                    anchor: captures.get(3).map(|value| ReferenceAnchorPayload {
                        algorithm: "sha256".to_owned(),
                        digest: value.as_str().to_ascii_lowercase(),
                        scope: ContentAnchorScope::File,
                        locator: None,
                        updated_at: None,
                        updated_by: None,
                        reason: None,
                        snapshot: None,
                    }),
                },
            ));
        }
    }
    occurrences
}

fn collect_markdown_link_occurrences(
    rel: &Path,
    text: &str,
) -> BTreeSet<ReferenceOccurrence> {
    let inline_link = Regex::new(
        r#"!?(?:\[[^\]\n]*\])\((?:<([^>\n]+)>|([^\s)\n]+))(?:\s+["'][^)\n]*["'])?\)"#,
    )
    .expect("static Markdown Link regex");
    let reference_definition =
        Regex::new(r#"^\s{0,3}\[[^\]\n]+\]:\s*(?:<([^>\n]+)>|(\S+))"#)
            .expect("static Markdown reference definition regex");
    let surface = strip_markdown_code(text);
    let mut occurrences = BTreeSet::new();
    for (line_index, line) in surface.lines().enumerate() {
        for captures in inline_link
            .captures_iter(line)
            .chain(reference_definition.captures_iter(line))
        {
            let target = captures
                .get(1)
                .or_else(|| captures.get(2))
                .map(|value| value.as_str())
                .unwrap_or("");
            occurrences.insert(ReferenceOccurrence::new(
                target,
                SYNTAX_MARKDOWN_LINK,
                CONTEXT_PROJECT_NAVIGATION,
                GovernanceLocation {
                    path: rel.display().to_string(),
                    line: Some(line_index + 1),
                },
                ReferencePayload::default(),
            ));
        }
    }
    occurrences
}

fn collect_frontmatter_occurrences(
    rel: &Path,
    text: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeSet<ReferenceOccurrence> {
    let Some(frontmatter) = markdown_frontmatter(text) else {
        return BTreeSet::new();
    };
    let mapping = match serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
        Ok(serde_yaml::Value::Mapping(mapping)) => mapping,
        Ok(_) => return BTreeSet::new(),
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                rel.display().to_string(),
                format!("Frontmatter cannot be read for reference analysis: {error}."),
            ));
            return BTreeSet::new();
        }
    };
    let mut occurrences = BTreeSet::new();
    if let Some(identity) = mapping
        .get(serde_yaml::Value::String("id".to_owned()))
        .and_then(serde_yaml::Value::as_str)
    {
        let line = text
            .lines()
            .position(|line| line.trim_start().starts_with("id:"))
            .map(|index| index + 1);
        occurrences.insert(ReferenceOccurrence::new(
            identity,
            SYNTAX_FRONTMATTER,
            CONTEXT_IDENTITY_DECLARATION,
            GovernanceLocation {
                path: rel.display().to_string(),
                line,
            },
            ReferencePayload::default(),
        ));
    }

    let anchors_value = mapping.get(serde_yaml::Value::String("anchors".to_owned()));
    let Some(anchors_value) = anchors_value else {
        return occurrences;
    };
    let Some(anchors) = anchors_value.as_sequence() else {
        push_frontmatter_anchor_diagnostic(
            rel,
            text.lines()
                .position(|line| line.trim_start().starts_with("anchors:"))
                .map(|index| index + 1),
            diagnostics,
            "Frontmatter 'anchors' must be a sequence of anchor declarations.".to_owned(),
        );
        return occurrences;
    };
    let lines = frontmatter_anchor_lines(text);
    for (index, value) in anchors.iter().enumerate() {
        let line = lines.get(index).copied();
        let declaration = match serde_yaml::from_value::<FrontmatterAnchorDeclaration>(
            value.clone(),
        ) {
            Ok(declaration) => declaration,
            Err(error) => {
                push_frontmatter_anchor_diagnostic(
                    rel,
                    line,
                    diagnostics,
                    format!("Invalid frontmatter anchor declaration: {error}."),
                );
                continue;
            }
        };
        if let Err(message) = declaration.validate() {
            push_frontmatter_anchor_diagnostic(rel, line, diagnostics, message);
            continue;
        }
        let selector = (declaration.scope == ContentAnchorScope::Block)
            .then(|| declaration.locator.clone())
            .flatten();
        occurrences.insert(ReferenceOccurrence::new(
            declaration.target,
            SYNTAX_FRONTMATTER,
            CONTEXT_GOVERNED_ANCHOR,
            GovernanceLocation {
                path: rel.display().to_string(),
                line,
            },
            ReferencePayload {
                selector,
                anchor: Some(ReferenceAnchorPayload {
                    algorithm: declaration.algorithm,
                    digest: declaration.digest.to_ascii_lowercase(),
                    scope: declaration.scope,
                    locator: declaration.locator,
                    updated_at: declaration.updated_at,
                    updated_by: declaration.updated_by,
                    reason: declaration.reason,
                    snapshot: declaration.snapshot,
                }),
            },
        ));
    }
    occurrences
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FrontmatterAnchorDeclaration {
    target: String,
    algorithm: String,
    digest: String,
    scope: ContentAnchorScope,
    #[serde(default)]
    locator: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    updated_by: Option<String>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    snapshot: Option<SnapshotProvenance>,
}

impl FrontmatterAnchorDeclaration {
    fn validate(&self) -> std::result::Result<(), String> {
        let sha256 = Regex::new(r"^[A-Fa-f0-9]{64}$").expect("static SHA-256 regex");
        let git_commit =
            Regex::new(r"^(?:[A-Fa-f0-9]{40}|[A-Fa-f0-9]{64})$").expect("static Git OID regex");
        let selector = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
            .expect("static selector regex");
        if let Some(snapshot) = &self.snapshot
            && (!valid_claim_identity(&snapshot.manifest)
                || !valid_repository_identity(&snapshot.repository)
                || !valid_commit_oid(&snapshot.commit)
                || !safe_snapshot_path(&snapshot.path))
        {
            return Err(
                "Snapshot provenance requires a stable manifest id, credential-free repository identity, full commit OID, and safe relative path."
                    .to_owned(),
            );
        }
        match self.scope {
            ContentAnchorScope::File => {
                if self.algorithm != "sha256" || !sha256.is_match(&self.digest) {
                    return Err(
                        "File anchor requires algorithm 'sha256' and a 64-hex digest."
                            .to_owned(),
                    );
                }
                if self.locator.is_some() {
                    return Err("File anchor must not declare a locator.".to_owned());
                }
            }
            ContentAnchorScope::Block => {
                if self.algorithm != "sha256" || !sha256.is_match(&self.digest) {
                    return Err(
                        "Block anchor requires algorithm 'sha256' and a 64-hex digest."
                            .to_owned(),
                    );
                }
                if !self
                    .locator
                    .as_deref()
                    .is_some_and(|value| selector.is_match(value))
                {
                    return Err(
                        "Block anchor requires a lowercase heading-slug locator.".to_owned(),
                    );
                }
            }
            ContentAnchorScope::Commit => {
                if self.algorithm != "git" || !git_commit.is_match(&self.digest) {
                    return Err(
                        "Commit anchor requires algorithm 'git' and a full 40- or 64-hex object ID."
                            .to_owned(),
                    );
                }
                if self.locator.is_some() {
                    return Err("Commit anchor must not declare a locator.".to_owned());
                }
                if self.snapshot.is_some() {
                    return Err("Commit anchor must not declare snapshot provenance.".to_owned());
                }
            }
        }
        Ok(())
    }
}

fn frontmatter_anchor_lines(text: &str) -> Vec<usize> {
    let mut in_frontmatter = false;
    let mut in_anchors = false;
    let mut anchors_indent = 0;
    let mut lines = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if index == 0 && line.trim() == "---" {
            in_frontmatter = true;
            continue;
        }
        if !in_frontmatter || line.trim() == "---" {
            break;
        }
        let trimmed = line.trim_start();
        let indent = line.len().saturating_sub(trimmed.len());
        if !in_anchors && trimmed == "anchors:" {
            in_anchors = true;
            anchors_indent = indent;
            continue;
        }
        if !in_anchors {
            continue;
        }
        if !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && indent <= anchors_indent
        {
            break;
        }
        if trimmed.starts_with("- ") {
            lines.push(index + 1);
        }
    }
    lines
}

fn push_frontmatter_anchor_diagnostic(
    rel: &Path,
    line: Option<usize>,
    diagnostics: &mut Vec<Diagnostic>,
    message: String,
) {
    let mut diagnostic = Diagnostic::new(
        "DH_REFERENCE_001",
        Severity::Error,
        rel.display().to_string(),
        message,
    );
    if let Some(line) = line {
        diagnostic = diagnostic.at_line(line);
    }
    diagnostics.push(diagnostic);
}
