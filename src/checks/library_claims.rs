fn check_core_library_claims(
    root: &Path,
    config: &Config,
    assets: &[GovernanceAsset],
    targets: &BTreeMap<String, SemanticTarget>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut claim_ids = BTreeMap::<&str, &str>::new();
    let body_paths = assets
        .iter()
        .filter(|asset| asset.reference_relation == ReferenceRelation::Body)
        .flat_map(|asset| asset_content_paths(root, asset))
        .collect::<BTreeSet<_>>();
    for claim in &config.governance.core_claims {
        if let Some(first_authority) = claim_ids.insert(&claim.id, &claim.authority.id) {
            diagnostics.push(Diagnostic::new(
                "DH_CLAIM_001",
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Core claim '{}' declares multiple canonical Library authorities '{}' and '{}'; keep exactly one.",
                    claim.id, first_authority, claim.authority.id
                ),
            ));
            continue;
        }
        if !valid_claim_identity(&claim.id)
            || !(0.0..=1.0).contains(&claim.similarity_threshold)
        {
            diagnostics.push(Diagnostic::new(
                "DH_CLAIM_001",
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Core claim '{}' requires a non-empty identity and similarityThreshold in 0..=1.",
                    claim.id
                ),
            ));
        }
        for pattern in &claim.candidate_paths {
            if let Err(error) = Glob::new(pattern) {
                diagnostics.push(Diagnostic::new(
                    "DH_CLAIM_001",
                    Severity::Error,
                    "docs-hygiene.yml",
                    format!(
                        "Core claim '{}' has invalid candidate path glob '{}': {error}.",
                        claim.id, pattern
                    ),
                ));
            }
        }
        let Some(authority) = targets.get(&claim.authority.id) else {
            diagnostics.push(Diagnostic::new(
                "DH_CLAIM_001",
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Core claim '{}' references unknown canonical Library authority '{}'.",
                    claim.id, claim.authority.id
                ),
            ));
            continue;
        };
        let authority_block = resolve_claim_authority(root, claim, authority, diagnostics);
        if LifecycleStatus::parse(&authority.status).is_some_and(LifecycleStatus::is_terminal) {
            let remediation = authority
                .superseded_by
                .as_deref()
                .map(|successor| format!("; set authority.id to '{successor}'"))
                .unwrap_or_default();
            diagnostics.push(
                Diagnostic::new(
                    "DH_CLAIM_001",
                    Severity::Error,
                    "docs-hygiene.yml",
                    format!(
                        "Core claim '{}' targets non-authoritative {} Library identity '{}'{}.",
                        claim.id, authority.status, claim.authority.id, remediation
                    ),
                )
                .with_related(RelatedInformation::new(
                    authority.path.clone(),
                    "The former Library authority is declared here.",
                )),
            );
        }
        let mut occurrences = BTreeSet::new();
        for occurrence in &claim.occurrences {
            let key = (occurrence.path.clone(), occurrence.selector.clone());
            if !occurrences.insert(key) {
                diagnostics.push(Diagnostic::new(
                    "DH_CLAIM_001",
                    Severity::Error,
                    "docs-hygiene.yml",
                    format!(
                        "Core claim '{}' repeats confirmed occurrence '{}#{}'.",
                        claim.id,
                        occurrence.path.display(),
                        occurrence.selector
                    ),
                ));
                continue;
            }
            check_claim_occurrence(
                root,
                claim,
                occurrence,
                authority,
                authority_block.as_deref(),
                &body_paths,
                diagnostics,
            );
        }
    }
}

fn resolve_claim_authority(
    root: &Path,
    claim: &crate::config::CoreClaimConfig,
    authority: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Vec<u8>> {
    let text = match std::fs::read_to_string(root.join(&authority.path)) {
        Ok(text) => text,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                "DH_CLAIM_001",
                Severity::Error,
                authority.path.clone(),
                format!(
                    "Core claim '{}' cannot read Library authority '{}': {error}.",
                    claim.id, claim.authority.id
                ),
            ));
            return None;
        }
    };
    match claim.authority.selector.as_deref() {
        Some(selector) => match markdown_heading_block(&text, selector) {
            Some(block) => Some(block.to_vec()),
            None => {
                diagnostics.push(Diagnostic::new(
                    "DH_CLAIM_001",
                    Severity::Error,
                    authority.path.clone(),
                    format!(
                        "Core claim '{}' authority selector '#{}' must resolve exactly once in '{}'.",
                        claim.id, selector, claim.authority.id
                    ),
                ));
                None
            }
        },
        None => Some(text.into_bytes()),
    }
}

#[allow(clippy::too_many_arguments)]
fn check_claim_occurrence(
    root: &Path,
    claim: &crate::config::CoreClaimConfig,
    occurrence: &crate::config::CoreClaimOccurrenceConfig,
    authority: &SemanticTarget,
    authority_block: Option<&[u8]>,
    body_paths: &BTreeSet<PathBuf>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let display = occurrence.path.display().to_string();
    if !body_paths.contains(&occurrence.path) {
        diagnostics.push(Diagnostic::new(
            "DH_CLAIM_001",
            Severity::Error,
            display,
            format!(
                "Core claim '{}' occurrence must be a declared governed Body member.",
                claim.id
            ),
        ));
        return;
    }
    let text = match std::fs::read_to_string(root.join(&occurrence.path)) {
        Ok(text) => text,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                "DH_CLAIM_001",
                Severity::Error,
                display,
                format!("Confirmed core-claim occurrence cannot be read: {error}."),
            ));
            return;
        }
    };
    let line = heading_line(&text, &occurrence.selector);
    if markdown_heading_block(&text, &occurrence.selector).is_none() {
        diagnostics.push(Diagnostic::new(
            "DH_CLAIM_001",
            Severity::Error,
            display,
            format!(
                "Core claim '{}' confirmed occurrence selector '#{}' must resolve exactly once.",
                claim.id, occurrence.selector
            ),
        ));
        return;
    }
    if occurrence.policy != CoreClaimOccurrencePolicy::Migrate
        && occurrence.migrate_by.is_some()
    {
        push_claim_occurrence_diagnostic(
            occurrence,
            line,
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Core claim '{}' occurrence may declare migrateBy only with policy 'migrate'.",
                claim.id
            ),
        );
    }
    match occurrence.policy {
        CoreClaimOccurrencePolicy::Forbidden => push_claim_occurrence_diagnostic(
            occurrence,
            line,
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Confirmed duplicate definition of core claim '{}' is forbidden; replace it with a semantic reference to '{}'.",
                claim.id, claim.authority.id
            ),
        ),
        CoreClaimOccurrencePolicy::Migrate => check_migration_occurrence(
            claim,
            occurrence,
            line,
            authority,
            diagnostics,
        ),
        CoreClaimOccurrencePolicy::ControlledExcerpt => check_controlled_excerpt(
            root,
            claim,
            occurrence,
            line,
            authority,
            authority_block,
            diagnostics,
        ),
    }
}

fn check_migration_occurrence(
    claim: &crate::config::CoreClaimConfig,
    occurrence: &crate::config::CoreClaimOccurrenceConfig,
    line: Option<usize>,
    authority: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(deadline) = occurrence.migrate_by.as_deref().and_then(parse_iso_date) else {
        push_claim_occurrence_diagnostic(
            occurrence,
            line,
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Core claim '{}' migration occurrence requires a valid migrateBy date in YYYY-MM-DD form.",
                claim.id
            ),
        );
        return;
    };
    let expired = today_utc().is_some_and(|today| today > deadline);
    push_claim_occurrence_diagnostic(
        occurrence,
        line,
        authority,
        diagnostics,
        if expired {
            Severity::Error
        } else {
            Severity::Warning
        },
        format!(
            "Confirmed duplicate definition of core claim '{}' {} its migration deadline {}; replace it with a semantic reference to '{}'.",
            claim.id,
            if expired { "passed" } else { "must be removed by" },
            occurrence.migrate_by.as_deref().unwrap_or_default(),
            claim.authority.id
        ),
    );
}

#[allow(clippy::too_many_arguments)]
fn check_controlled_excerpt(
    root: &Path,
    claim: &crate::config::CoreClaimConfig,
    occurrence: &crate::config::CoreClaimOccurrenceConfig,
    line: Option<usize>,
    authority: &SemanticTarget,
    authority_block: Option<&[u8]>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(locator) = claim.authority.selector.as_deref() else {
        push_claim_occurrence_diagnostic(
            occurrence,
            line,
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Controlled excerpt for core claim '{}' requires authority.selector so it can pin one authoritative block.",
                claim.id
            ),
        );
        return;
    };
    let Some(authority_block) = authority_block else {
        return;
    };
    let expected = format!("{:x}", Sha256::digest(authority_block));
    let mut reference_diagnostics = Vec::new();
    let occurrences = collect_governed_reference_occurrences(
        root,
        std::slice::from_ref(&occurrence.path),
        &mut reference_diagnostics,
    );
    diagnostics.extend(reference_diagnostics);
    let matching = occurrences.iter().find(|reference| {
        reference.raw_target == claim.authority.id
            && reference.context == CONTEXT_GOVERNED_ANCHOR
            && reference.payload.selector.as_deref() == Some(locator)
            && reference.payload.anchor.as_ref().is_some_and(|anchor| {
                anchor.scope == ContentAnchorScope::Block && anchor.locator.as_deref() == Some(locator)
            })
    });
    let Some(reference) = matching else {
        push_claim_occurrence_diagnostic(
            occurrence,
            line,
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Controlled excerpt for core claim '{}' must declare a block-scope frontmatter anchor to '{}#{}' with sha256:{}.",
                claim.id, claim.authority.id, locator, expected
            ),
        );
        return;
    };
    let actual = reference
        .payload
        .anchor
        .as_ref()
        .map(|anchor| anchor.digest.as_str())
        .unwrap_or_default();
    if actual != expected {
        push_claim_occurrence_diagnostic(
            occurrence,
            reference.location.line.or(line),
            authority,
            diagnostics,
            Severity::Error,
            format!(
                "Controlled excerpt for core claim '{}' is stale: expected authoritative block sha256:{expected}, found sha256:{actual}; review the authority change and refresh the pin.",
                claim.id
            ),
        );
    }
}

fn push_claim_occurrence_diagnostic(
    occurrence: &crate::config::CoreClaimOccurrenceConfig,
    line: Option<usize>,
    authority: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
    severity: Severity,
    message: String,
) {
    let mut diagnostic = Diagnostic::new(
        "DH_CLAIM_001",
        severity,
        occurrence.path.display().to_string(),
        message,
    )
    .with_related(RelatedInformation::new(
        authority.path.clone(),
        "Canonical Library authority is here.",
    ));
    if let Some(line) = line {
        diagnostic = diagnostic.at_line(line);
    }
    diagnostics.push(diagnostic);
}

fn heading_line(text: &str, selector: &str) -> Option<usize> {
    let matches = markdown_blocks(text)
        .into_iter()
        .filter(|block| block.selector == selector)
        .map(|block| block.line)
        .collect::<Vec<_>>();
    matches.as_slice().first().copied().filter(|_| matches.len() == 1)
}

fn valid_claim_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
}
