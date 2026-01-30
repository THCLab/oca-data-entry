use std::collections::{HashMap, HashSet};

use oca_sdk_rs::overlay_registry::OverlayLocalRegistry;
use oca_sdk_rs::{NestedAttrType, OCABundle, OCABundleModel, RefValue};

use crate::{
    errors::EntryError,
    extract::ExtractOptions,
    model::{AttributeSpec, DependencyIndex, EntrySchema},
};

pub fn entry_schema_from_bundle(
    bundle: &OCABundleModel,
    overlay_registry: &OverlayLocalRegistry,
    options: &ExtractOptions,
) -> Result<EntrySchema, EntryError> {
    entry_schema_from_bundle_with_deps(
        bundle,
        &DependencyIndex::default(),
        overlay_registry,
        options,
    )
}

pub fn entry_schema_from_bundle_with_deps(
    bundle: &OCABundleModel,
    deps: &DependencyIndex,
    _overlay_registry: &OverlayLocalRegistry,
    options: &ExtractOptions,
) -> Result<EntrySchema, EntryError> {
    let said = bundle
        .digest
        .as_ref()
        .map(|s| s.to_string())
        .ok_or(EntryError::MissingSaid)?;

    let mut attributes = Vec::new();
    let mut seen = HashSet::new();
    let label_map = labels_from_bundle_model(bundle, options.label_lang.as_deref());

    collect_attributes(
        "",
        None,
        &bundle.capture_base.attributes,
        deps,
        &label_map,
        &mut attributes,
        &mut seen,
    );

    Ok(EntrySchema { said, attributes })
}

fn collect_attributes(
    prefix: &str,
    label_prefix: Option<String>,
    attributes: &indexmap::IndexMap<String, NestedAttrType>,
    deps: &DependencyIndex,
    label_map: &HashMap<String, String>,
    out: &mut Vec<AttributeSpec>,
    seen: &mut HashSet<String>,
) {
    for (name, attr_type) in attributes {
        let full_name = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", prefix, name)
        };

        let current_label = label_map.get(name).cloned();
        let combined_label = match (&label_prefix, current_label.as_ref()) {
            (Some(prefix_label), Some(label)) => Some(format!("{}.{}", prefix_label, label)),
            (Some(prefix_label), None) => Some(format!("{}.{}", prefix_label, name)),
            (None, Some(label)) => Some(label.clone()),
            (None, None) => None,
        };

        if let Some(target_bundle) = resolve_reference(attr_type, deps) {
            let target_label_map = labels_from_bundle(target_bundle, None);
            collect_attributes(
                &full_name,
                current_label,
                &target_bundle.capture_base.attributes,
                deps,
                &target_label_map,
                out,
                seen,
            );
            continue;
        }

        if let NestedAttrType::Array(inner) = attr_type {
            if let Some(target_bundle) = resolve_reference(inner.as_ref(), deps) {
                let target_label_map = labels_from_bundle(target_bundle, None);
                collect_attributes(
                    &full_name,
                    current_label,
                    &target_bundle.capture_base.attributes,
                    deps,
                    &target_label_map,
                    out,
                    seen,
                );
                continue;
            }
        }

        if seen.insert(full_name.clone()) {
            out.push(AttributeSpec {
                name: full_name,
                label: combined_label,
                attr_type: Some(format_attr_type(attr_type)),
                required: false,
                format: None,
                unit: None,
                cardinality: None,
                entry_values: None,
            });
        }
    }
}

fn resolve_reference<'a>(
    attr_type: &NestedAttrType,
    deps: &'a DependencyIndex,
) -> Option<&'a OCABundle> {
    match attr_type {
        NestedAttrType::Reference(RefValue::Said(said)) => deps.by_said.get(&said.to_string()),
        NestedAttrType::Reference(RefValue::Name(name)) => deps.by_refn.get(name),
        _ => None,
    }
}

fn labels_from_bundle_model(
    bundle: &OCABundleModel,
    lang: Option<&str>,
) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for overlay in &bundle.overlays {
        if !overlay.name.contains("label") {
            continue;
        }
        if let Some(props) = &overlay.properties {
            if let Ok(value) = serde_json::to_value(props) {
                if !language_matches(&value, lang) {
                    continue;
                }
                if let Some(map) = value.get("attribute_labels").and_then(|v| v.as_object()) {
                    for (k, v) in map {
                        if let Some(label) = v.as_str() {
                            out.insert(k.to_string(), label.to_string());
                        }
                    }
                }
            }
        }
    }
    out
}

fn labels_from_bundle(bundle: &OCABundle, lang: Option<&str>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for overlay in &bundle.overlays {
        let name = &overlay.model.name;
        if !name.contains("label") {
            continue;
        }
        if let Some(props) = &overlay.model.properties {
            if let Ok(value) = serde_json::to_value(props) {
                if !language_matches(&value, lang) {
                    continue;
                }
                if let Some(map) = value.get("attribute_labels").and_then(|v| v.as_object()) {
                    for (k, v) in map {
                        if let Some(label) = v.as_str() {
                            out.insert(k.to_string(), label.to_string());
                        }
                    }
                }
            }
        }
    }
    out
}

fn language_matches(value: &serde_json::Value, lang: Option<&str>) -> bool {
    match lang {
        None => true,
        Some(lang) => value
            .get("language")
            .and_then(|v| v.as_str())
            .map(|l| l == lang)
            .unwrap_or(false),
    }
}

fn format_attr_type(attr_type: &NestedAttrType) -> String {
    match attr_type {
        NestedAttrType::Reference(reference) => format!("reference({})", reference),
        NestedAttrType::Value(value) => value.to_string(),
        NestedAttrType::Array(inner) => format!("array<{}>", format_attr_type(inner)),
        NestedAttrType::Null => "null".to_string(),
    }
}
