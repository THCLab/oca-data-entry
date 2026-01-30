pub struct ExtractOptions {
    pub label_lang: Option<String>,
    pub metadata_lang: Option<String>,
}

impl Default for ExtractOptions {
    fn default() -> Self {
        Self { label_lang: None, metadata_lang: None }
    }
}

mod from_bundle;

pub use from_bundle::{entry_schema_from_bundle, entry_schema_from_bundle_with_deps};
