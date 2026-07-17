use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Result, bail};
use globset::{Glob, GlobSetBuilder};
use serde::Serialize;

use crate::config::{Config, CriticalDependencyPolicyConfig, CriticalDependencyRelation};
use crate::{
    ContentAnchorScope, GovernanceEdge, GovernanceEdgeKind, GovernanceGraph, GovernanceNode,
    run_checks,
};

mod pin_update_digest;
mod pin_update_edit;
mod pin_update_output;

use pin_update_digest::{
    audit_metadata_current, compute_pin_digest, governed_pin_source, utc_today_string,
};
use pin_update_edit::apply_pin_changes;
pub use pin_update_output::{print_json_pin_update, print_text_pin_update};

#[derive(Clone, Debug)]
pub struct PinUpdateRequest {
    pub policies: Vec<String>,
    pub targets: Vec<String>,
    pub actor: String,
    pub reason: String,
    pub apply: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinUpdateChange {
    pub policy: String,
    pub source: String,
    pub target: String,
    pub source_path: String,
    pub source_line: Option<usize>,
    pub target_path: String,
    pub relation: CriticalDependencyRelation,
    pub algorithm: String,
    pub scope: ContentAnchorScope,
    pub selector: Option<String>,
    pub old_digest: Option<String>,
    pub new_digest: String,
    pub updated_at: String,
    pub updated_by: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinUpdateBlock {
    pub policy: Option<String>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinUpdateReport {
    pub schema_version: &'static str,
    pub changes: Vec<PinUpdateChange>,
    pub blocked: Vec<PinUpdateBlock>,
    pub audit_log: String,
    pub applied: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Dependency {
    source: String,
    target: String,
    relation: CriticalDependencyRelation,
    edge_index: usize,
}

pub fn update_critical_pins(
    root: &Path,
    config: &Config,
    request: &PinUpdateRequest,
) -> Result<PinUpdateReport> {
    let mut report = PinUpdateReport {
        schema_version: "docs-hygiene.pin-update.v1",
        changes: Vec::new(),
        blocked: Vec::new(),
        audit_log: config.governance.pin_audit_log.display().to_string(),
        applied: false,
    };
    if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
        report.blocked.push(PinUpdateBlock {
            policy: None,
            source: None,
            target: None,
            path: "docs-hygiene.yml".to_owned(),
            reason: "pin update requires non-empty actor and reason".to_owned(),
        });
        return Ok(report);
    }
    let policies = selected_policies(config, request, &mut report.blocked);
    if policies.is_empty() {
        if report.blocked.is_empty() {
            report.blocked.push(PinUpdateBlock {
                policy: None,
                source: None,
                target: None,
                path: "docs-hygiene.yml".to_owned(),
                reason: "no critical dependency policies are selected".to_owned(),
            });
        }
        return Ok(report);
    }
    let checks = run_checks(root, config)?;
    for diagnostic in &checks.diagnostics {
        if diagnostic.code == "DH_PIN_006"
            || (diagnostic.code == "DH_REFERENCE_001"
                && diagnostic.message.contains("anchor declaration"))
        {
            report.blocked.push(PinUpdateBlock {
                policy: None,
                source: None,
                target: None,
                path: diagnostic.path.clone(),
                reason: diagnostic.message.clone(),
            });
        }
    }
    let graph = &checks.governance_graph;
    let dependencies = normalized_dependencies(graph);
    let selected_targets = request.targets.iter().cloned().collect::<BTreeSet<_>>();
    let mut matched_targets = BTreeSet::new();
    let updated_at = utc_today_string();
    for policy in policies {
        for dependency in &dependencies {
            if !selected_targets.is_empty() && !selected_targets.contains(&dependency.target) {
                continue;
            }
            let edge = &graph.edges[dependency.edge_index];
            let Some(source) = graph.node(&dependency.source) else {
                continue;
            };
            let Some(target) = graph.node(&dependency.target) else {
                continue;
            };
            if !policy_matches(policy, dependency, edge, source, target) {
                continue;
            }
            matched_targets.insert(dependency.target.clone());
            match plan_dependency_update(
                root,
                graph,
                policy,
                dependency,
                edge,
                target,
                request,
                &updated_at,
            ) {
                Ok(Some(change)) => report.changes.push(change),
                Ok(None) => {}
                Err(reason) => report.blocked.push(PinUpdateBlock {
                    policy: Some(policy.id.clone()),
                    source: Some(dependency.source.clone()),
                    target: Some(dependency.target.clone()),
                    path: edge.source_location.path.clone(),
                    reason: reason.to_string(),
                }),
            }
        }
    }
    for target in selected_targets.difference(&matched_targets) {
        report.blocked.push(PinUpdateBlock {
            policy: None,
            source: None,
            target: Some(target.clone()),
            path: "docs-hygiene.yml".to_owned(),
            reason: "selected target is not matched by a selected critical dependency policy"
                .to_owned(),
        });
    }
    report.changes.sort_by(|left, right| {
        (&left.policy, &left.source, &left.target, &left.source_path).cmp(&(
            &right.policy,
            &right.source,
            &right.target,
            &right.source_path,
        ))
    });
    report.changes.dedup_by(|left, right| {
        left.source == right.source
            && left.target == right.target
            && left.source_path == right.source_path
            && left.new_digest == right.new_digest
    });
    report.blocked.sort_by(|left, right| {
        (&left.path, &left.reason, &left.target).cmp(&(&right.path, &right.reason, &right.target))
    });
    if request.apply && report.blocked.is_empty() && !report.changes.is_empty() {
        apply_pin_changes(root, config, &report.changes)?;
        report.applied = true;
    }
    Ok(report)
}

fn selected_policies<'a>(
    config: &'a Config,
    request: &PinUpdateRequest,
    blocked: &mut Vec<PinUpdateBlock>,
) -> Vec<&'a CriticalDependencyPolicyConfig> {
    let selected = request.policies.iter().collect::<BTreeSet<_>>();
    for id in &selected {
        if config
            .governance
            .critical_dependencies
            .iter()
            .all(|policy| &policy.id != *id)
        {
            blocked.push(PinUpdateBlock {
                policy: Some((*id).clone()),
                source: None,
                target: None,
                path: "docs-hygiene.yml".to_owned(),
                reason: "selected critical dependency policy does not exist".to_owned(),
            });
        }
    }
    config
        .governance
        .critical_dependencies
        .iter()
        .filter(|policy| selected.is_empty() || selected.contains(&&policy.id))
        .collect()
}

fn normalized_dependencies(graph: &GovernanceGraph) -> Vec<Dependency> {
    let mut dependencies = BTreeMap::<(String, String, CriticalDependencyRelation), usize>::new();
    for (index, edge) in graph.edges.iter().enumerate() {
        let relation = CriticalDependencyRelation::from_edge_kind(edge.relation);
        let key = (edge.source.clone(), edge.target.clone(), relation);
        let replace = dependencies.get(&key).is_none_or(|current| {
            graph.edges[*current].relation != GovernanceEdgeKind::PinnedReference
                && edge.relation == GovernanceEdgeKind::PinnedReference
        });
        if replace {
            dependencies.insert(key, index);
        }
    }
    dependencies
        .into_iter()
        .map(|((source, target, relation), edge_index)| Dependency {
            source,
            target,
            relation,
            edge_index,
        })
        .collect()
}

fn policy_matches(
    policy: &CriticalDependencyPolicyConfig,
    dependency: &Dependency,
    edge: &GovernanceEdge,
    source: &GovernanceNode,
    target: &GovernanceNode,
) -> bool {
    let matcher = &policy.matcher;
    (matcher.source_kinds.is_empty() || matcher.source_kinds.contains(&source.reference_relation))
        && (matcher.target_kinds.is_empty()
            || matcher.target_kinds.contains(&target.reference_relation))
        && (matcher.relations.is_empty() || matcher.relations.contains(&dependency.relation))
        && string_matches(&matcher.source_ids, &dependency.source)
        && string_matches(&matcher.target_ids, &dependency.target)
        && path_matches(&matcher.source_paths, &edge.source_location.path)
        && path_matches(&matcher.target_paths, &target.location.path)
}

fn string_matches(configured: &[String], actual: &str) -> bool {
    configured.is_empty() || configured.iter().any(|value| value == actual)
}

fn path_matches(configured: &[String], actual: &str) -> bool {
    if configured.is_empty() {
        return true;
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in configured {
        let Ok(pattern) = Glob::new(pattern) else {
            return false;
        };
        builder.add(pattern);
    }
    builder
        .build()
        .is_ok_and(|patterns| patterns.is_match(Path::new(actual)))
}

#[allow(clippy::too_many_arguments)]
fn plan_dependency_update(
    root: &Path,
    graph: &GovernanceGraph,
    policy: &CriticalDependencyPolicyConfig,
    dependency: &Dependency,
    edge: &GovernanceEdge,
    target: &GovernanceNode,
    request: &PinUpdateRequest,
    updated_at: &str,
) -> Result<Option<PinUpdateChange>> {
    let existing = graph.edges.iter().find(|pin| {
        pin.source == dependency.source
            && pin.target == dependency.target
            && pin.relation == GovernanceEdgeKind::PinnedReference
    });
    let eligible = graph.edges.iter().find(|pin| {
        pin.source == dependency.source
            && pin.target == dependency.target
            && pin.relation == GovernanceEdgeKind::PinnedReference
            && pin.content_anchor.as_ref().is_some_and(|anchor| {
                policy
                    .require
                    .algorithms
                    .iter()
                    .any(|allowed| allowed == anchor.algorithm)
                    && anchor.scope.meets_minimum(policy.require.minimum_scope)
                    && !(policy.require.forbid_whole_file && anchor.scope.covers_whole_file())
            })
    });
    if graph.edges.iter().any(|pin| {
        pin.source == dependency.source
            && pin.target == dependency.target
            && pin.relation == GovernanceEdgeKind::PinnedReference
            && pin
                .content_anchor
                .as_ref()
                .is_some_and(|anchor| anchor.snapshot.is_some())
    }) {
        bail!(
            "portable snapshot Pins require an explicit snapshot manifest/payload update before their digest can change"
        );
    }
    let (algorithm, scope, selector, old_digest, source_path, source_line) =
        if let Some(pin) = eligible {
            let anchor = pin.content_anchor.as_ref().expect("eligible pin anchor");
            (
                anchor.algorithm.to_owned(),
                anchor.scope,
                anchor.locator.clone().or_else(|| pin.selector.clone()),
                Some(anchor.digest.clone()),
                pin.source_location.path.clone(),
                pin.source_location.line,
            )
        } else {
            let (algorithm, scope) = choose_pin_shape(policy)?;
            let selector = if scope == ContentAnchorScope::Block {
                edge.selector
                    .clone()
                    .or_else(|| existing.and_then(|pin| pin.selector.clone()))
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "policy requires block scope but dependency has no heading selector"
                        )
                    })?
            } else {
                String::new()
            };
            (
                algorithm,
                scope,
                (!selector.is_empty()).then_some(selector),
                existing.and_then(|pin| {
                    pin.content_anchor
                        .as_ref()
                        .map(|anchor| anchor.digest.clone())
                }),
                match existing {
                    Some(pin) => pin.source_location.path.clone(),
                    None => governed_pin_source(root, &edge.source_location.path)?,
                },
                existing
                    .and_then(|pin| pin.source_location.line)
                    .or(edge.source_location.line),
            )
        };
    if Path::new(&source_path)
        .extension()
        .and_then(|value| value.to_str())
        != Some("md")
    {
        bail!("pin update source must be governed Markdown content");
    }
    let new_digest = compute_pin_digest(root, target, &algorithm, scope, selector.as_deref())?;
    let metadata_current = eligible.is_some_and(|pin| {
        pin.content_anchor.as_ref().is_some_and(|anchor| {
            policy
                .require
                .max_age_days
                .is_none_or(|max_age| audit_metadata_current(anchor, max_age))
        })
    });
    if old_digest.as_deref() == Some(new_digest.as_str()) && metadata_current {
        return Ok(None);
    }
    Ok(Some(PinUpdateChange {
        policy: policy.id.clone(),
        source: dependency.source.clone(),
        target: dependency.target.clone(),
        source_path,
        source_line,
        target_path: target.location.path.clone(),
        relation: dependency.relation,
        algorithm,
        scope,
        selector,
        old_digest,
        new_digest,
        updated_at: updated_at.to_owned(),
        updated_by: request.actor.trim().to_owned(),
        reason: request.reason.trim().to_owned(),
    }))
}

fn choose_pin_shape(
    policy: &CriticalDependencyPolicyConfig,
) -> Result<(String, ContentAnchorScope)> {
    if policy.require.minimum_scope == ContentAnchorScope::Block || policy.require.forbid_whole_file
    {
        if policy
            .require
            .algorithms
            .iter()
            .any(|value| value == "sha256")
        {
            return Ok(("sha256".to_owned(), ContentAnchorScope::Block));
        }
        bail!("block scope requires allowed algorithm 'sha256'");
    }
    if policy.require.minimum_scope == ContentAnchorScope::Commit
        && policy.require.algorithms.iter().any(|value| value == "git")
    {
        return Ok(("git".to_owned(), ContentAnchorScope::Commit));
    }
    if policy.require.minimum_scope == ContentAnchorScope::Commit
        && policy
            .require
            .algorithms
            .iter()
            .any(|value| value == "sha256")
    {
        return Ok(("sha256".to_owned(), ContentAnchorScope::Block));
    }
    if policy
        .require
        .algorithms
        .iter()
        .any(|value| value == "sha256")
    {
        return Ok(("sha256".to_owned(), ContentAnchorScope::File));
    }
    if policy.require.algorithms.iter().any(|value| value == "git") {
        return Ok(("git".to_owned(), ContentAnchorScope::Commit));
    }
    bail!("policy has no supported algorithm")
}
