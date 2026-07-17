use anyhow::{Result, bail};

use super::{CapabilityDimension, HygieneMaturity};
use crate::activation::{ActivationReport, RuleState};
use crate::config::{
    Config, DimensionApplicability, DimensionProfileConfig, HygieneProfileDimensionsConfig,
    MaturityLevel,
};

pub(super) fn configured_dimension(
    dimensions: &HygieneProfileDimensionsConfig,
    dimension: CapabilityDimension,
) -> Option<&DimensionProfileConfig> {
    match dimension {
        CapabilityDimension::Structure => dimensions.structure.as_ref(),
        CapabilityDimension::Identity => dimensions.identity.as_ref(),
        CapabilityDimension::Dependency => dimensions.dependency.as_ref(),
        CapabilityDimension::Topology => dimensions.topology.as_ref(),
    }
}

pub(super) fn dimension_is_inferred_applicable(
    dimension: CapabilityDimension,
    activation: &ActivationReport,
) -> bool {
    match dimension {
        CapabilityDimension::Structure => true,
        CapabilityDimension::Topology => {
            activation.facts.configured_governance_manifests > 0
                || activation.facts.semantic_wiki_links > 0
        }
        CapabilityDimension::Identity | CapabilityDimension::Dependency => crate::RULE_SPECS
            .iter()
            .filter(|spec| {
                spec.capabilities
                    .iter()
                    .any(|capability| capability.dimension == dimension)
            })
            .any(|spec| activation.decision(spec.id).state != RuleState::Inactive),
    }
}

pub(super) fn validate_profile_config(config: &Config) -> Result<()> {
    for dimension in CapabilityDimension::ALL {
        let Some(value) = configured_dimension(&config.hygiene_profile.dimensions, dimension)
        else {
            continue;
        };
        if value.applicability == DimensionApplicability::NotApplicable {
            if value.target.is_some() {
                bail!(
                    "hygieneProfile dimension '{}' cannot declare a target when notApplicable",
                    dimension.label()
                );
            }
            if value.rationale.as_deref().is_none_or(str::is_empty) {
                bail!(
                    "hygieneProfile dimension '{}' requires a rationale when notApplicable",
                    dimension.label()
                );
            }
        } else if value.required && value.target.is_none() {
            bail!(
                "required hygieneProfile dimension '{}' must declare a target",
                dimension.label()
            );
        }
    }
    if let Some(structure) = &config.hygiene_profile.dimensions.structure {
        let legacy = legacy_structure_target(config.document_contracts.maturity.declared);
        if structure.applicability == DimensionApplicability::NotApplicable {
            bail!("structure cannot be notApplicable while legacy contract maturity is configured");
        }
        if structure.target.is_some_and(|target| target != legacy) {
            bail!(
                "hygieneProfile structure target conflicts with legacy documentContracts maturity; expected '{}'",
                legacy.label()
            );
        }
    }
    Ok(())
}

pub(super) fn legacy_structure_target(level: MaturityLevel) -> HygieneMaturity {
    match level {
        MaturityLevel::Seed => HygieneMaturity::Basic,
        MaturityLevel::Growing | MaturityLevel::Maintained => HygieneMaturity::Controlled,
        MaturityLevel::Governed => HygieneMaturity::Governed,
    }
}
