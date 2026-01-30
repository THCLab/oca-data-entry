use oca_sdk_rs::OCABundle;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct EntrySchema {
    pub said: String,
    pub attributes: Vec<AttributeSpec>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AttributeSpec {
    pub name: String,
    pub label: Option<String>,
    pub attr_type: Option<String>,
    pub required: bool,
    pub format: Option<String>,
    pub unit: Option<String>,
    pub cardinality: Option<String>,
    pub entry_values: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct DependencyIndex {
    pub by_said: HashMap<String, OCABundle>,
    pub by_refn: HashMap<String, OCABundle>,
}
