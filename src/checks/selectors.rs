fn validate_edge_selector(
    root: &Path,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    let Some(selector) = edge.selector.as_deref() else {
        return true;
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
            return false;
        }
    };
    let slugs = markdown_heading_slug_counts(&text);
    match slugs.get(selector).copied().unwrap_or_default() {
        1 => true,
        0 => {
            push_selector_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Selector '#{selector}' does not resolve to an ATX heading in Wiki Link target '{}'.",
                edge.target
            ),
            );
            false
        }
        matches => {
            push_selector_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Selector '#{selector}' is ambiguous because Wiki Link target '{}' contains {matches} headings with that slug.",
                edge.target
            ),
            );
            false
        }
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
