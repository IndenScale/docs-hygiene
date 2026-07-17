fn validate_edge_selector(
    root: &Path,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(selector) = edge.selector.as_deref() else {
        return;
    };
    let text = match std::fs::read_to_string(root.join(&target.path)) {
        Ok(text) => text,
        Err(error) => {
            push_selector_diagnostic(
                edge,
                target,
                diagnostics,
                format!(
                    "Selector '#{selector}' for '{}' cannot read its target: {error}.",
                    edge.target
                ),
            );
            return;
        }
    };
    let slugs = markdown_heading_slug_counts(&text);
    match slugs.get(selector).copied().unwrap_or_default() {
        1 => {}
        0 => push_selector_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Selector '#{selector}' does not resolve to an ATX heading in Wiki Link target '{}'.",
                edge.target
            ),
        ),
        matches => push_selector_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Selector '#{selector}' is ambiguous because Wiki Link target '{}' contains {matches} headings with that slug.",
                edge.target
            ),
        ),
    }
}

fn push_selector_diagnostic(
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
    message: String,
) {
    let mut diagnostic = Diagnostic::new(
        "DH_SELECTOR_001",
        Severity::Error,
        edge.source_location.path.clone(),
        message,
    )
    .with_related(RelatedInformation::new(
        target.path.clone(),
        "Selector target is declared here.",
    ));
    if let Some(line) = edge.source_location.line {
        diagnostic = diagnostic.at_line(line);
    }
    diagnostics.push(diagnostic);
}

fn markdown_heading_slug_counts(text: &str) -> BTreeMap<String, usize> {
    let mut slugs = BTreeMap::new();
    for slug in strip_code_blocks(text).lines().filter_map(|line| {
            let trimmed = line.trim_start();
            let hashes = trimmed.chars().take_while(|value| *value == '#').count();
            if !(1..=6).contains(&hashes) || !trimmed[hashes..].starts_with(char::is_whitespace) {
                return None;
            }
            let heading = trimmed[hashes..]
                .trim()
                .trim_end_matches('#')
                .trim();
            heading_slug(heading)
        }) {
        *slugs.entry(slug).or_default() += 1;
    }
    slugs
}

fn heading_slug(heading: &str) -> Option<String> {
    let mut slug = String::new();
    let mut separator = false;
    for value in heading.chars() {
        if value.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(value.to_ascii_lowercase());
            separator = false;
        } else if !slug.is_empty() {
            separator = true;
        }
    }
    (!slug.is_empty()).then_some(slug)
}
