use oca_sdk_rs::overlay_registry::OverlayLocalRegistry;
use oca_sdk_rs::{NestedAttrType, OCABundleModel};

use crate::{errors::EntryError, model::{AttributeSpec, EntrySchema}};

pub fn entry_schema_from_bundle(
    bundle: &OCABundleModel,
    _overlay_registry: &OverlayLocalRegistry,
) -> Result<EntrySchema, EntryError> {
    let said = bundle
        .digest
        .as_ref()
        .map(|s| s.to_string())
        .ok_or(EntryError::MissingSaid)?;

    let mut attributes = Vec::new();

    for (name, attr_type) in &bundle.capture_base.attributes {
        attributes.push(AttributeSpec {
            name: name.clone(),
            label: None,
            attr_type: Some(format_attr_type(attr_type)),
            required: false,
            format: None,
            unit: None,
            cardinality: None,
            entry_values: None,
        });
    }

    // Future: enrich attributes using overlays in bundle.overlays and overlayfile registry.
    // - label -> label
    // - conformance -> required
    // - format -> format
    // - unit -> unit
    // - cardinality -> cardinality
    // - entry/entry_code -> entry_values

    Ok(EntrySchema { said, attributes })
}

fn format_attr_type(attr_type: &NestedAttrType) -> String {
    match attr_type {
        NestedAttrType::Reference(reference) => format!("reference({})", reference),
        NestedAttrType::Value(value) => value.to_string(),
        NestedAttrType::Array(inner) => format!("array<{}>", format_attr_type(inner)),
        NestedAttrType::Null => "null".to_string(),
    }
}
