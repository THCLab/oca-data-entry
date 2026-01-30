#[derive(Default)]
pub struct ExtractOptions {
    pub label_lang: Option<String>,
    pub metadata_lang: Option<String>,
}

mod from_bundle;

pub use from_bundle::{entry_schema_from_bundle, entry_schema_from_bundle_with_deps};
