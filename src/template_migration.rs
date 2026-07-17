use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::Config;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMigrationReport {
    pub schema_version: &'static str,
    pub changes: Vec<TemplateMigrationChange>,
    pub unchanged_profiles: Vec<String>,
    pub blocked: Vec<TemplateMigrationBlock>,
    pub applied: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMigrationChange {
    pub profile: String,
    pub template: String,
    pub from_revision: Option<u64>,
    pub to_revision: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMigrationBlock {
    pub profile: Option<String>,
    pub template: String,
    pub reason: String,
}

impl TemplateMigrationReport {
    pub fn is_clean(&self) -> bool {
        self.changes.is_empty() && self.blocked.is_empty()
    }
}

pub fn migrate_document_template_bindings(
    config: &mut Config,
    apply: bool,
) -> TemplateMigrationReport {
    let mut report = TemplateMigrationReport {
        schema_version: "docs-hygiene.template-migration.v1",
        changes: Vec::new(),
        unchanged_profiles: Vec::new(),
        blocked: Vec::new(),
        applied: false,
    };
    let mut templates = BTreeMap::new();
    for template in &config.document_contracts.templates {
        if templates.contains_key(&template.id) {
            report.blocked.push(TemplateMigrationBlock {
                profile: None,
                template: template.id.clone(),
                reason: "template identity is declared more than once".to_owned(),
            });
            continue;
        }
        let Some(revision) = template.revision else {
            report.blocked.push(TemplateMigrationBlock {
                profile: None,
                template: template.id.clone(),
                reason: "template has no revision".to_owned(),
            });
            continue;
        };
        let compatible_from = template.compatible_from.unwrap_or(revision);
        if revision == 0 || compatible_from == 0 || compatible_from > revision {
            report.blocked.push(TemplateMigrationBlock {
                profile: None,
                template: template.id.clone(),
                reason: format!("invalid revision window {compatible_from}..={revision}"),
            });
            continue;
        }
        templates.insert(template.id.clone(), (revision, compatible_from));
    }

    let mut profile_ids = BTreeSet::new();
    for profile in &config.document_contracts.profiles {
        if !profile_ids.insert(profile.id.clone()) {
            report.blocked.push(TemplateMigrationBlock {
                profile: Some(profile.id.clone()),
                template: profile.template.clone().unwrap_or_default(),
                reason: "profile identity is declared more than once".to_owned(),
            });
            continue;
        }
        let Some(template) = profile.template.as_ref() else {
            continue;
        };
        let Some(&(revision, compatible_from)) = templates.get(template) else {
            report.blocked.push(TemplateMigrationBlock {
                profile: Some(profile.id.clone()),
                template: template.clone(),
                reason: "template is unknown or has no valid revision".to_owned(),
            });
            continue;
        };
        match profile.template_revision {
            Some(pin) if pin == revision => report.unchanged_profiles.push(profile.id.clone()),
            Some(pin) if pin >= compatible_from && pin < revision => {
                report.changes.push(TemplateMigrationChange {
                    profile: profile.id.clone(),
                    template: template.clone(),
                    from_revision: Some(pin),
                    to_revision: revision,
                });
            }
            None => report.changes.push(TemplateMigrationChange {
                profile: profile.id.clone(),
                template: template.clone(),
                from_revision: None,
                to_revision: revision,
            }),
            Some(pin) => report.blocked.push(TemplateMigrationBlock {
                profile: Some(profile.id.clone()),
                template: template.clone(),
                reason: format!(
                    "revision {pin} is outside compatibility window {compatible_from}..={revision}"
                ),
            }),
        }
    }

    report
        .changes
        .sort_by(|left, right| left.profile.cmp(&right.profile));
    report.unchanged_profiles.sort();
    report.blocked.sort_by(|left, right| {
        (&left.template, &left.profile, &left.reason).cmp(&(
            &right.template,
            &right.profile,
            &right.reason,
        ))
    });
    if apply && report.blocked.is_empty() {
        let revisions = report
            .changes
            .iter()
            .map(|change| (change.profile.as_str(), change.to_revision))
            .collect::<BTreeMap<_, _>>();
        for profile in &mut config.document_contracts.profiles {
            if let Some(revision) = revisions.get(profile.id.as_str()) {
                profile.template_revision = Some(*revision);
            }
        }
        report.applied = true;
    }
    report
}

pub fn print_text_template_migration(report: &TemplateMigrationReport) {
    println!(
        "Template migration: {} change(s), {} unchanged, {} blocked, applied: {}.",
        report.changes.len(),
        report.unchanged_profiles.len(),
        report.blocked.len(),
        report.applied,
    );
    for change in &report.changes {
        println!(
            "  {}: {} {:?} -> {}",
            change.profile, change.template, change.from_revision, change.to_revision
        );
    }
    for blocked in &report.blocked {
        println!(
            "  blocked {}{}: {}",
            blocked.template,
            blocked
                .profile
                .as_ref()
                .map(|profile| format!("/{profile}"))
                .unwrap_or_default(),
            blocked.reason
        );
    }
}

pub fn print_json_template_migration(report: &TemplateMigrationReport) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_pins_are_planned_and_applied_without_partial_blocked_updates() {
        let mut compatible: Config = serde_yaml::from_str(
            r#"
documentContracts:
  templates:
    - id: base
      revision: 2
      compatibleFrom: 1
  profiles:
    - id: current
      template: base
      templateRevision: 2
      match: {}
    - id: old
      template: base
      templateRevision: 1
      match: {}
    - id: floating
      template: base
      match: {}
"#,
        )
        .unwrap();

        let report = migrate_document_template_bindings(&mut compatible, true);
        assert!(report.blocked.is_empty());
        assert!(report.applied);
        assert_eq!(report.changes.len(), 2);
        assert_eq!(
            compatible.document_contracts.profiles[1].template_revision,
            Some(2)
        );
        assert_eq!(
            compatible.document_contracts.profiles[2].template_revision,
            Some(2)
        );

        let mut incompatible: Config = serde_yaml::from_str(
            r#"
documentContracts:
  templates:
    - id: base
      revision: 3
      compatibleFrom: 2
  profiles:
    - id: blocked
      template: base
      templateRevision: 1
      match: {}
    - id: otherwise-compatible
      template: base
      templateRevision: 2
      match: {}
"#,
        )
        .unwrap();
        let report = migrate_document_template_bindings(&mut incompatible, true);
        assert!(!report.applied);
        assert_eq!(report.blocked.len(), 1);
        assert_eq!(
            incompatible.document_contracts.profiles[1].template_revision,
            Some(2)
        );
    }
}
