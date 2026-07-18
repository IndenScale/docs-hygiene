#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryClaimCandidate {
    pub claim: String,
    pub authority_id: String,
    pub authority_path: String,
    pub authority_selector: Option<String>,
    pub candidate_path: String,
    pub candidate_selector: String,
    pub candidate_line: usize,
    pub similarity: f64,
    pub authority_fragment: String,
    pub candidate_fragment: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryClaimScanReport {
    pub schema_version: &'static str,
    pub candidates: Vec<LibraryClaimCandidate>,
}

#[derive(Clone)]
struct MarkdownBlock {
    selector: String,
    line: usize,
    text: String,
}

pub fn scan_library_claim_candidates(
    root: &Path,
    config: &Config,
) -> Result<LibraryClaimScanReport> {
    let assets = load_claim_scan_assets(root, config)?;
    let mut ignored = Vec::new();
    let targets = build_library_target_index(root, config, &assets, &mut ignored);
    if let Some(diagnostic) = ignored.first() {
        anyhow::bail!("cannot scan Library claims: {}", diagnostic.message);
    }
    let mut candidates = Vec::new();
    for claim in &config.governance.core_claims {
        if !(0.0..=1.0).contains(&claim.similarity_threshold) {
            anyhow::bail!(
                "core claim '{}' similarityThreshold must be in 0..=1",
                claim.id
            );
        }
        let authority = targets.get(&claim.authority.id).ok_or_else(|| {
            anyhow::anyhow!(
                "core claim '{}' references unknown Library authority '{}'",
                claim.id,
                claim.authority.id
            )
        })?;
        let authority_text = std::fs::read_to_string(root.join(&authority.path))?;
        let authority_fragment = match claim.authority.selector.as_deref() {
            Some(selector) => markdown_heading_block(&authority_text, selector)
                .map(|block| String::from_utf8_lossy(block).into_owned())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "core claim '{}' authority selector '#{}' must resolve exactly once",
                        claim.id,
                        selector
                    )
                })?,
            None => authority_text,
        };
        let authority_terms = claim_terms(&authority_fragment);
        if authority_terms.is_empty() {
            continue;
        }
        let confirmed = claim
            .occurrences
            .iter()
            .map(|occurrence| {
                (
                    occurrence.path.as_path(),
                    occurrence.selector.as_str(),
                )
            })
            .collect::<BTreeSet<_>>();
        let paths = claim_candidate_paths(root, &claim.candidate_paths)?;
        for path in paths {
            let rel = path.strip_prefix(root)?.to_path_buf();
            if rel == Path::new(&authority.path) {
                continue;
            }
            let text = std::fs::read_to_string(&path)?;
            for block in markdown_blocks(&text) {
                if confirmed.contains(&(rel.as_path(), block.selector.as_str())) {
                    continue;
                }
                let score = jaccard_similarity(&authority_terms, &claim_terms(&block.text));
                if score + f64::EPSILON < claim.similarity_threshold {
                    continue;
                }
                candidates.push(LibraryClaimCandidate {
                    claim: claim.id.clone(),
                    authority_id: claim.authority.id.clone(),
                    authority_path: authority.path.clone(),
                    authority_selector: claim.authority.selector.clone(),
                    candidate_path: rel.display().to_string(),
                    candidate_selector: block.selector,
                    candidate_line: block.line,
                    similarity: (score * 10_000.0).round() / 10_000.0,
                    authority_fragment: evidence_fragment(&authority_fragment),
                    candidate_fragment: evidence_fragment(&block.text),
                });
            }
        }
    }
    candidates.sort_by(|left, right| {
        (&left.claim, &left.candidate_path, left.candidate_line).cmp(&(
            &right.claim,
            &right.candidate_path,
            right.candidate_line,
        ))
    });
    Ok(LibraryClaimScanReport {
        schema_version: "docs-hygiene.library-claim-scan.v1",
        candidates,
    })
}

pub fn print_text_library_claim_scan(report: &LibraryClaimScanReport) {
    for candidate in &report.candidates {
        println!(
            "candidate {} {}#{}:{} -> {}#{} ({:.4})",
            candidate.claim,
            candidate.candidate_path,
            candidate.candidate_selector,
            candidate.candidate_line,
            candidate.authority_id,
            candidate
                .authority_selector
                .as_deref()
                .unwrap_or("<file>"),
            candidate.similarity
        );
    }
    if report.candidates.is_empty() {
        println!("No Library claim duplication candidates found.");
    }
}

pub fn print_json_library_claim_scan(report: &LibraryClaimScanReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

fn load_claim_scan_assets(root: &Path, config: &Config) -> Result<Vec<GovernanceAsset>> {
    let mut assets = Vec::new();
    for rel in &config.governance.manifests {
        let path = root.join(rel);
        let text = std::fs::read_to_string(&path)?;
        let yaml = if path.extension().and_then(|value| value.to_str()) == Some("md") {
            markdown_frontmatter(&text)
                .ok_or_else(|| anyhow::anyhow!("{} requires YAML frontmatter", rel.display()))?
        } else {
            text.as_str()
        };
        let mut asset: GovernanceAsset = serde_yaml::from_str(yaml)?;
        asset.path = rel.display().to_string();
        assets.push(asset);
    }
    Ok(assets)
}

fn claim_candidate_paths(root: &Path, patterns: &[String]) -> Result<Vec<PathBuf>> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    let globs = builder.build()?;
    let mut paths = WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !matches!(name.as_ref(), ".git" | "target")
        })
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let rel = entry.path().strip_prefix(root).ok()?;
            (entry.path().extension().and_then(|value| value.to_str()) == Some("md")
                && globs.is_match(rel))
            .then(|| entry.path().to_path_buf())
        })
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

fn markdown_blocks(text: &str) -> Vec<MarkdownBlock> {
    let mut headings = Vec::new();
    let mut offset = 0;
    let mut in_code = false;
    for (index, segment) in text.split_inclusive('\n').enumerate() {
        let line = segment.trim_end_matches(['\n', '\r']);
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_code = !in_code;
            offset += segment.len();
            continue;
        }
        if !in_code {
            let level = trimmed.chars().take_while(|value| *value == '#').count();
            if (1..=6).contains(&level)
                && trimmed[level..].starts_with(char::is_whitespace)
                && let Some(selector) = heading_slug(
                    trimmed[level..]
                        .trim()
                        .trim_end_matches('#')
                        .trim(),
                )
            {
                headings.push((selector, level, offset, index + 1));
            }
        }
        offset += segment.len();
    }
    headings
        .iter()
        .enumerate()
        .map(|(index, (selector, level, start, line))| {
            let end = headings
                .iter()
                .skip(index + 1)
                .find(|(_, next_level, _, _)| next_level <= level)
                .map(|(_, _, start, _)| *start)
                .unwrap_or(text.len());
            MarkdownBlock {
                selector: selector.clone(),
                line: *line,
                text: text[*start..end].to_owned(),
            }
        })
        .collect()
}

fn claim_terms(text: &str) -> BTreeSet<String> {
    strip_markdown_code(text)
        .split(|character: char| !character.is_alphanumeric())
        .map(str::to_lowercase)
        .filter(|term| term.chars().count() >= 3)
        .collect()
}

fn jaccard_similarity(left: &BTreeSet<String>, right: &BTreeSet<String>) -> f64 {
    if left.is_empty() || right.is_empty() {
        return 0.0;
    }
    let intersection = left.intersection(right).count();
    let union = left.union(right).count();
    intersection as f64 / union as f64
}

fn evidence_fragment(text: &str) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    normalized.chars().take(160).collect()
}
