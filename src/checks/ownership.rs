#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IdentityGovernanceMetadata {
    ownership: Option<OwnershipDeclaration>,
    review: Option<ReviewDeclaration>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OwnershipDeclaration {
    #[serde(default)]
    owner: String,
    #[serde(default)]
    understood_by: Vec<UnderstandingConfirmation>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnderstandingConfirmation {
    #[serde(default)]
    principal: String,
    #[serde(default)]
    confirmed_at: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewDeclaration {
    #[serde(default)]
    review_by: String,
    last_reset: Option<ReviewResetDeclaration>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewResetDeclaration {
    #[serde(default)]
    at: String,
    #[serde(default)]
    by: String,
    #[serde(default)]
    reason: String,
}

fn check_document_ownership(
    root: &Path,
    config: &Config,
    records: &[GovernedIdentityRecord],
    diagnostics: &mut Vec<Diagnostic>,
) -> OwnershipReport {
    if !config.governance.ownership.is_configured() {
        return OwnershipReport::default();
    }
    let principals = validate_principal_directory(config, diagnostics);
    let required = records
        .iter()
        .filter(|record| matches!(record.status.as_str(), "baselined" | "current"))
        .collect::<Vec<_>>();
    if records.is_empty() {
        for (code, obligation) in [
            ("DH_OWNERSHIP_001", "responsibility"),
            ("DH_REVIEW_001", "review sunset"),
            ("DH_KNOWLEDGE_001", "knowledge redundancy"),
        ] {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Ownership governance cannot prove {obligation} without configured governance manifests."
                ),
            ));
        }
    }

    let today = today_utc();
    let mut responsibility_covered = 0;
    let mut review_covered = 0;
    let mut knowledge_covered = 0;
    let mut reviews_due_soon = 0;
    let mut reviews_expired = 0;
    let mut identities = Vec::new();
    for record in required {
        let metadata = read_identity_governance_metadata(root, record).unwrap_or_default();
        let owner = metadata
            .ownership
            .as_ref()
            .map(|ownership| ownership.owner.clone())
            .filter(|owner| !owner.is_empty());
        let owner_valid = owner
            .as_deref()
            .is_some_and(|owner| valid_owner(owner, &principals));
        if owner_valid {
            responsibility_covered += 1;
        } else {
            diagnostics.push(Diagnostic::new(
                "DH_OWNERSHIP_001",
                Severity::Error,
                record.path.clone(),
                format!(
                    "Governed identity '{}' requires one active, resolvable person or expanded group owner.",
                    record.id
                ),
            ));
        }

        let review_by = metadata
            .review
            .as_ref()
            .map(|review| review.review_by.clone())
            .filter(|review_by| !review_by.is_empty());
        let review_state = validate_review(
            record,
            metadata.review.as_ref(),
            &principals,
            today,
            config.governance.ownership.review_warning_days,
            diagnostics,
        );
        match review_state {
            ReviewState::Current => review_covered += 1,
            ReviewState::DueSoon => {
                review_covered += 1;
                reviews_due_soon += 1;
            }
            ReviewState::Expired => reviews_expired += 1,
            ReviewState::Invalid | ReviewState::Missing => {}
        }

        let valid_understanders = validate_understanding(
            record,
            metadata.ownership.as_ref(),
            &principals,
            today,
            config.governance.ownership.confirmation_max_age_days,
            diagnostics,
        );
        if valid_understanders.len() >= 2 {
            knowledge_covered += 1;
        }
        identities.push(OwnershipIdentityEvidence {
            identity: record.id.clone(),
            path: record.path.clone(),
            owner,
            owner_valid,
            review_by,
            review_state,
            knowledge_bus_factor: valid_understanders.len(),
            valid_understanders,
        });
    }
    identities.sort_by(|left, right| {
        (&left.identity, &left.path).cmp(&(&right.identity, &right.path))
    });
    let total = identities.len();
    OwnershipReport {
        enabled: true,
        responsibility_coverage: Coverage::new(responsibility_covered, total),
        review_coverage: Coverage::new(review_covered, total),
        knowledge_redundancy_coverage: Coverage::new(knowledge_covered, total),
        reviews_due_soon,
        reviews_expired,
        identities,
    }
}

fn validate_principal_directory<'a>(
    config: &'a Config,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<&'a str, &'a GovernancePrincipalConfig> {
    let mut principals = BTreeMap::new();
    for principal in &config.governance.ownership.principals {
        let duplicate = principals.insert(principal.id.as_str(), principal).is_some();
        let valid_prefix = match principal.kind {
            GovernancePrincipalKind::Person => principal.id.starts_with("person:"),
            GovernancePrincipalKind::Group => principal.id.starts_with("group:"),
        };
        let members_valid = match principal.kind {
            GovernancePrincipalKind::Person => principal.members.is_empty(),
            GovernancePrincipalKind::Group => {
                !principal.members.is_empty()
                    && principal.members.iter().collect::<BTreeSet<_>>().len()
                        == principal.members.len()
            }
        };
        if duplicate || !valid_prefix || !valid_principal_suffix(&principal.id) || !members_valid {
            diagnostics.push(Diagnostic::new(
                "DH_OWNERSHIP_001",
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Principal '{}' requires a unique kind-matching stable id; people have no members and groups have a non-empty unique member list.",
                    principal.id
                ),
            ));
        }
    }
    for principal in principals.values() {
        if principal.kind != GovernancePrincipalKind::Group {
            continue;
        }
        for member in &principal.members {
            if principals
                .get(member.as_str())
                .is_none_or(|member| member.kind != GovernancePrincipalKind::Person)
            {
                diagnostics.push(Diagnostic::new(
                    "DH_OWNERSHIP_001",
                    Severity::Error,
                    "docs-hygiene.yml",
                    format!(
                        "Group principal '{}' member '{}' must resolve directly to a person principal.",
                        principal.id, member
                    ),
                ));
            }
        }
    }
    principals
}

fn valid_principal_suffix(id: &str) -> bool {
    id.split_once(':').is_some_and(|(_, suffix)| {
        !suffix.is_empty()
            && suffix
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
    })
}

fn valid_owner(
    owner: &str,
    principals: &BTreeMap<&str, &GovernancePrincipalConfig>,
) -> bool {
    let Some(principal) = principals.get(owner) else {
        return false;
    };
    if principal.status != GovernancePrincipalStatus::Active {
        return false;
    }
    match principal.kind {
        GovernancePrincipalKind::Person => true,
        GovernancePrincipalKind::Group => principal.members.iter().any(|member| {
            principals.get(member.as_str()).is_some_and(|member| {
                member.kind == GovernancePrincipalKind::Person
                    && member.status == GovernancePrincipalStatus::Active
            })
        }),
    }
}

fn validate_review(
    record: &GovernedIdentityRecord,
    review: Option<&ReviewDeclaration>,
    principals: &BTreeMap<&str, &GovernancePrincipalConfig>,
    today: Option<(i32, u32, u32)>,
    warning_days: u64,
    diagnostics: &mut Vec<Diagnostic>,
) -> ReviewState {
    let Some(review) = review else {
        push_review_error(record, "requires review.reviewBy", diagnostics);
        return ReviewState::Missing;
    };
    let Some(deadline) = parse_iso_date(&review.review_by) else {
        push_review_error(record, "has an invalid review.reviewBy date", diagnostics);
        return ReviewState::Invalid;
    };
    if let Some(reset) = &review.last_reset {
        let reset_valid = parse_iso_date(&reset.at).is_some_and(|at| {
            today.is_some_and(|today| at <= today) && at <= deadline
        }) && !reset.reason.trim().is_empty()
            && principals.get(reset.by.as_str()).is_some_and(|principal| {
                principal.kind == GovernancePrincipalKind::Person
                    && principal.status == GovernancePrincipalStatus::Active
            });
        if !reset_valid {
            push_review_error(
                record,
                "has invalid review.lastReset at/by/reason evidence",
                diagnostics,
            );
            return ReviewState::Invalid;
        }
    }
    let Some(today) = today else {
        push_review_error(record, "cannot evaluate the current UTC date", diagnostics);
        return ReviewState::Invalid;
    };
    let days_until = ownership_days_from_civil(deadline) - ownership_days_from_civil(today);
    if days_until < 0 {
        push_review_error(record, "review deadline is expired", diagnostics);
        return ReviewState::Expired;
    }
    if u64::try_from(days_until).is_ok_and(|days| days <= warning_days) {
        diagnostics.push(Diagnostic::new(
            "DH_REVIEW_002",
            Severity::Warning,
            record.path.clone(),
            format!(
                "Governed identity '{}' review is due by {}; perform an explicit reset before expiry.",
                record.id, review.review_by
            ),
        ));
        return ReviewState::DueSoon;
    }
    ReviewState::Current
}

fn push_review_error(
    record: &GovernedIdentityRecord,
    reason: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    diagnostics.push(Diagnostic::new(
        "DH_REVIEW_001",
        Severity::Error,
        record.path.clone(),
        format!("Governed identity '{}': {reason}.", record.id),
    ));
}

fn validate_understanding(
    record: &GovernedIdentityRecord,
    ownership: Option<&OwnershipDeclaration>,
    principals: &BTreeMap<&str, &GovernancePrincipalConfig>,
    today: Option<(i32, u32, u32)>,
    max_age_days: u64,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<String> {
    let mut valid = BTreeSet::new();
    let mut declared = BTreeSet::new();
    let confirmations = ownership
        .map(|ownership| ownership.understood_by.as_slice())
        .unwrap_or_default();
    let mut invalid = false;
    for confirmation in confirmations {
        if !declared.insert(confirmation.principal.as_str()) {
            invalid = true;
            continue;
        }
        let person_valid = principals
            .get(confirmation.principal.as_str())
            .is_some_and(|principal| {
                principal.kind == GovernancePrincipalKind::Person
                    && principal.status == GovernancePrincipalStatus::Active
            });
        let confirmation_valid = parse_iso_date(&confirmation.confirmed_at)
            .zip(today)
            .is_some_and(|(confirmed, today)| {
                let age = ownership_days_from_civil(today)
                    - ownership_days_from_civil(confirmed);
                age >= 0 && u64::try_from(age).is_ok_and(|age| age <= max_age_days)
            });
        if person_valid && confirmation_valid {
            valid.insert(confirmation.principal.clone());
        } else {
            invalid = true;
        }
    }
    if invalid || valid.len() < 2 {
        diagnostics.push(Diagnostic::new(
            "DH_KNOWLEDGE_001",
            Severity::Error,
            record.path.clone(),
            format!(
                "Governed identity '{}' requires current confirmations from at least two unique active person principals; groups do not count as people.",
                record.id
            ),
        ));
    }
    valid.into_iter().collect()
}

fn read_identity_governance_metadata(
    root: &Path,
    record: &GovernedIdentityRecord,
) -> Option<IdentityGovernanceMetadata> {
    let text = std::fs::read_to_string(root.join(&record.path)).ok()?;
    let yaml = if Path::new(&record.path)
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("md")
    {
        markdown_frontmatter(&text)?
    } else {
        text.as_str()
    };
    serde_yaml::from_str(yaml).ok()
}

fn ownership_days_from_civil((year, month, day): (i32, u32, u32)) -> i64 {
    let year = i64::from(year) - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5
        + i64::from(day)
        - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}
