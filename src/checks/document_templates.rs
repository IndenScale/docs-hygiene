#[derive(Clone, Debug)]
struct ResolvedDocumentProfile {
    id: String,
    matcher: DocumentMatchConfig,
    required_sections: Vec<RequiredSectionConfig>,
    required_fields: Vec<RequiredFieldConfig>,
    ordered_sections: bool,
    enforce_from: MaturityLevel,
    placeholders_allowed_until: MaturityLevel,
    placeholder_patterns: Vec<String>,
}

fn resolve_document_profiles(
    config: &Config,
    diagnostics: &mut Vec<Diagnostic>,
) -> (Vec<ResolvedDocumentProfile>, DocumentTemplateReport) {
    let contracts = &config.document_contracts;
    let mut report = DocumentTemplateReport {
        configured_templates: contracts.templates.len(),
        configured_profiles: contracts.profiles.len(),
        ..DocumentTemplateReport::default()
    };
    let mut templates = BTreeMap::new();
    for template in &contracts.templates {
        if !valid_contract_identity(&template.id) {
            invalid_template_registry(
                &mut report,
                diagnostics,
                format!("Document template identity '{}' is invalid.", template.id),
            );
        }
        match templates.entry(template.id.as_str()) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(template);
                report.bindings.insert(template.id.clone(), Vec::new());
            }
            std::collections::btree_map::Entry::Occupied(_) => invalid_template_registry(
                &mut report,
                diagnostics,
                format!("Document template identity '{}' is declared more than once.", template.id),
            ),
        }
        record_template_revision(
            template,
            contracts.maturity.declared,
            &mut report,
            diagnostics,
        );
    }

    let mut profile_ids = BTreeSet::new();
    let mut resolved = Vec::new();
    for profile in &contracts.profiles {
        if !valid_contract_identity(&profile.id) {
            invalid_template_registry(
                &mut report,
                diagnostics,
                format!("Document profile identity '{}' is invalid.", profile.id),
            );
        }
        if !profile_ids.insert(profile.id.as_str()) {
            invalid_template_registry(
                &mut report,
                diagnostics,
                format!("Document profile identity '{}' is declared more than once.", profile.id),
            );
            continue;
        }

        let template = match profile.template.as_deref() {
            Some(identity) => match templates.get(identity).copied() {
                Some(template) => {
                    report
                        .bindings
                        .get_mut(identity)
                        .expect("unique template initialized its binding list")
                        .push(profile.id.clone());
                    record_profile_revision(
                        profile,
                        template,
                        contracts.maturity.declared,
                        &mut report,
                        diagnostics,
                    );
                    Some(template)
                }
                None => {
                    invalid_template_registry(
                        &mut report,
                        diagnostics,
                        format!(
                            "Document profile '{}' references unknown template '{}'.",
                            profile.id, identity
                        ),
                    );
                    None
                }
            },
            None => {
                report.untemplated_profiles.push(profile.id.clone());
                if profile.template_revision.is_some() {
                    invalid_template_registry(
                        &mut report,
                        diagnostics,
                        format!(
                            "Document profile '{}' pins a template revision without a template binding.",
                            profile.id
                        ),
                    );
                }
                None
            }
        };
        let resolved_profile = merge_document_contract(profile, template);
        validate_resolved_contract(&resolved_profile, &mut report, diagnostics);
        resolved.push(resolved_profile);
    }

    for (template, profiles) in &report.bindings {
        if profiles.is_empty() {
            report.unused_templates.push(template.clone());
            diagnostics.push(Diagnostic::new(
                "DH_TEMPLATE_002",
                Severity::Error,
                ".",
                format!("Document template '{template}' has no profile binding."),
            ));
        }
    }
    report.untemplated_profiles.sort();
    report.unused_templates.sort();
    report.unrevisioned_templates.sort();
    report.unrevisioned_templates.dedup();
    report.unpinned_profiles.sort();
    report.outdated_profiles.sort();
    report.incompatible_profiles.sort();
    for profiles in report.bindings.values_mut() {
        profiles.sort();
    }
    (resolved, report)
}

fn record_template_revision(
    template: &DocumentTemplateConfig,
    declared: MaturityLevel,
    report: &mut DocumentTemplateReport,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(revision) = template.revision else {
        report.unrevisioned_templates.push(template.id.clone());
        if template.compatible_from.is_some() {
            invalid_template_registry(
                report,
                diagnostics,
                format!(
                    "Document template '{}' declares compatibleFrom without revision.",
                    template.id
                ),
            );
        } else if declared == MaturityLevel::Governed {
            template_migration_diagnostic(
                diagnostics,
                declared,
                format!(
                    "Document template '{}' requires a positive revision at governed maturity.",
                    template.id
                ),
            );
        }
        return;
    };
    let compatible_from = template.compatible_from.unwrap_or(revision);
    if revision == 0 || compatible_from == 0 || compatible_from > revision {
        report.unrevisioned_templates.push(template.id.clone());
        invalid_template_registry(
            report,
            diagnostics,
            format!(
                "Document template '{}' has invalid revision window {}..={}.",
                template.id, compatible_from, revision
            ),
        );
        return;
    }
    report.template_revisions.insert(
        template.id.clone(),
        TemplateRevisionReport {
            revision,
            compatible_from,
        },
    );
}

fn record_profile_revision(
    profile: &DocumentProfileConfig,
    template: &DocumentTemplateConfig,
    declared: MaturityLevel,
    report: &mut DocumentTemplateReport,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(pin) = profile.template_revision else {
        report.unpinned_profiles.push(profile.id.clone());
        if template.revision.is_some() || declared == MaturityLevel::Governed {
            template_migration_diagnostic(
                diagnostics,
                declared,
                format!(
                    "Document profile '{}' requires a templateRevision pin for template '{}'.",
                    profile.id, template.id
                ),
            );
        }
        return;
    };
    report.profile_revision_pins.insert(profile.id.clone(), pin);
    let Some(window) = report.template_revisions.get(&template.id) else {
        invalid_template_registry(
            report,
            diagnostics,
            format!(
                "Document profile '{}' pins revision {} but template '{}' has no valid revision.",
                profile.id, pin, template.id
            ),
        );
        return;
    };
    if pin == window.revision {
        return;
    }
    if pin >= window.compatible_from && pin < window.revision {
        report.outdated_profiles.push(profile.id.clone());
        template_migration_diagnostic(
            diagnostics,
            declared,
            format!(
                "Document profile '{}' pins compatible template revision {}; migrate to {}.",
                profile.id, pin, window.revision
            ),
        );
        return;
    }
    report.incompatible_profiles.push(profile.id.clone());
    diagnostics.push(Diagnostic::new(
        "DH_TEMPLATE_004",
        Severity::Error,
        ".",
        format!(
            "Document profile '{}' pins revision {}, outside template '{}' compatibility window {}..={}.",
            profile.id, pin, template.id, window.compatible_from, window.revision
        ),
    ));
}

fn template_migration_diagnostic(
    diagnostics: &mut Vec<Diagnostic>,
    declared: MaturityLevel,
    message: String,
) {
    let severity = if declared == MaturityLevel::Governed {
        Severity::Error
    } else {
        Severity::Warning
    };
    diagnostics.push(Diagnostic::new(
        "DH_TEMPLATE_003",
        severity,
        ".",
        message,
    ));
}

fn merge_document_contract(
    profile: &DocumentProfileConfig,
    template: Option<&DocumentTemplateConfig>,
) -> ResolvedDocumentProfile {
    let base = template.map(|template| &template.contract);
    let mut required_sections = base
        .map(|contract| contract.required_sections.clone())
        .unwrap_or_default();
    required_sections.extend(profile.contract.required_sections.clone());
    let mut required_fields = base
        .map(|contract| contract.required_fields.clone())
        .unwrap_or_default();
    required_fields.extend(profile.contract.required_fields.clone());
    let mut placeholder_patterns = base
        .map(|contract| contract.placeholder_patterns.clone())
        .unwrap_or_default();
    placeholder_patterns.extend(profile.contract.placeholder_patterns.clone());

    ResolvedDocumentProfile {
        id: profile.id.clone(),
        matcher: profile.matcher.clone(),
        required_sections,
        required_fields,
        ordered_sections: profile
            .contract
            .ordered_sections
            .or_else(|| base.and_then(|contract| contract.ordered_sections))
            .unwrap_or_default(),
        enforce_from: profile
            .contract
            .enforce_from
            .or_else(|| base.and_then(|contract| contract.enforce_from))
            .unwrap_or(MaturityLevel::Maintained),
        placeholders_allowed_until: profile
            .contract
            .placeholders_allowed_until
            .or_else(|| base.and_then(|contract| contract.placeholders_allowed_until))
            .unwrap_or(MaturityLevel::Growing),
        placeholder_patterns,
    }
}

fn validate_resolved_contract(
    profile: &ResolvedDocumentProfile,
    report: &mut DocumentTemplateReport,
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_unique_members(
        profile,
        "section",
        profile.required_sections.iter().map(|section| section.id.as_str()),
        report,
        diagnostics,
    );
    validate_unique_members(
        profile,
        "field",
        profile.required_fields.iter().map(|field| field.id.as_str()),
        report,
        diagnostics,
    );
    for (kind, pattern) in profile
        .required_fields
        .iter()
        .map(|field| ("required field", field.pattern.as_str()))
        .chain(
            profile
                .placeholder_patterns
                .iter()
                .map(|pattern| ("placeholder", pattern.as_str())),
        )
    {
        if let Err(error) = Regex::new(pattern) {
            invalid_template_registry(
                report,
                diagnostics,
                format!(
                    "Document profile '{}' has invalid {kind} pattern '{pattern}': {error}.",
                    profile.id
                ),
            );
        }
    }
    for path in &profile.matcher.paths {
        if let Err(error) = Glob::new(path) {
            invalid_template_registry(
                report,
                diagnostics,
                format!(
                    "Document profile '{}' has invalid path expression '{path}': {error}.",
                    profile.id
                ),
            );
        }
    }
    for filename in &profile.matcher.filenames {
        if let Err(error) = Regex::new(filename) {
            invalid_template_registry(
                report,
                diagnostics,
                format!(
                    "Document profile '{}' has invalid filename expression '{filename}': {error}.",
                    profile.id
                ),
            );
        }
    }
}

fn validate_unique_members<'a>(
    profile: &ResolvedDocumentProfile,
    kind: &str,
    identities: impl Iterator<Item = &'a str>,
    report: &mut DocumentTemplateReport,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen = BTreeSet::new();
    for identity in identities {
        if !seen.insert(identity) {
            invalid_template_registry(
                report,
                diagnostics,
                format!(
                    "Resolved document profile '{}' contains duplicate {kind} identity '{identity}'.",
                    profile.id
                ),
            );
        }
    }
}

fn valid_contract_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
}

fn invalid_template_registry(
    report: &mut DocumentTemplateReport,
    diagnostics: &mut Vec<Diagnostic>,
    message: String,
) {
    report.registry_valid = false;
    diagnostics.push(Diagnostic::new(
        "DH_TEMPLATE_001",
        Severity::Error,
        ".",
        message,
    ));
}
