use std::collections::BTreeMap;
use std::path::PathBuf;

mod registry;
mod scaffold;
mod schema;

pub use registry::validate_document_kind_registry;
pub use scaffold::plan_scaffold_document;
pub use schema::{parse_frontmatter_mapping, validate_kind_frontmatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KindIssueCategory {
    Registry,
    SchemaRevision,
    Frontmatter,
}

#[derive(Clone, Debug)]
pub struct KindIssue {
    pub category: KindIssueCategory,
    pub field: Option<String>,
    pub message: String,
    pub blocking: bool,
}

#[derive(Clone, Debug)]
pub struct ScaffoldDocumentRequest {
    pub kind: String,
    pub identity: String,
    pub slug: String,
    pub locale: String,
    pub target_dir: Option<PathBuf>,
    pub fields: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ScaffoldDocumentPlan {
    pub relative_path: PathBuf,
    pub content: String,
}
