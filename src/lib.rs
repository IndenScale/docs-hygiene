//! Public implementation reference for Docs Hygiene policies.
//!
//! This crate is `SDK-001` version `0.1.0`. It projects `GLOSSARY-001` version
//! `0.1.0` into the Implementation Layer. The CLI is one consumer of this API.

pub mod checks;
pub mod config;
pub mod report;

pub use checks::run_checks;
pub use config::Config;
pub use report::{Report, print_json_report, print_text_report};
