//! Public implementation reference for Docs Hygiene policies.
//!
//! The library API is the Implementation Layer projection of the governed
//! terminology and specifications. The CLI is one consumer of this API.

pub mod checks;
pub mod config;
pub mod report;

pub use checks::run_checks;
pub use config::Config;
pub use report::{Report, print_json_report, print_text_report};
