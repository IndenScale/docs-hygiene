//! Public implementation reference for Docs Hygiene policies.
//!
//! This crate is governed as `SDK-001` by `sdk-manifest.yml`. The manifest declares
//! its Glossary projection. The CLI is one consumer of this API.
//! Semantic source: [[GLOSSARY-001]].

pub mod checks;
pub mod config;
pub mod report;

pub use checks::run_checks;
pub use config::Config;
pub use report::{Report, print_json_report, print_text_report};
