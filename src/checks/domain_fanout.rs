fn check_domain_fanout(
    id: &str,
    path: &str,
    direct_members: usize,
    budget: &crate::config::DomainFanoutConfig,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let (severity, threshold) = if direct_members >= budget.error_at {
        (Severity::Error, budget.error_at)
    } else if direct_members >= budget.warning_at {
        (Severity::Warning, budget.warning_at)
    } else {
        return;
    };

    diagnostics.push(Diagnostic::new(
        "DH_DOMAIN_001",
        severity,
        path,
        format!(
            "Library Domain '{id}' declares {direct_members} direct members, reaching the {threshold}-member {} threshold. Introduce semantic Sub Domains or explicitly adjust governance.domainFanout.",
            match severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                _ => unreachable!("fan-out diagnostics use warning or error severity"),
            }
        ),
    ));
}
