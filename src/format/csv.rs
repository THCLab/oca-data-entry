use std::io::Write;

use crate::model::EntrySchema;

pub struct CsvOptions {
    pub include_metadata_row: bool,
    pub label_lang: Option<String>,
    pub metadata_lang: Option<String>,
}

pub fn write_csv(schema: &EntrySchema, mut out: impl Write, opts: &CsvOptions) -> std::io::Result<()> {
    // Bundle SAID as a comment tag
    writeln!(out, "# oca_bundle_said={}", schema.said)?;

    // Header row
    let headers: Vec<String> = schema
        .attributes
        .iter()
        .map(|a| {
            if opts.label_lang.is_some() {
                a.label.clone().unwrap_or_else(|| a.name.clone())
            } else {
                a.name.clone()
            }
        })
        .collect();

    writeln!(out, "{}", headers.join(","))?;

    // Metadata row
    if opts.include_metadata_row {
        let meta: Vec<String> = schema
            .attributes
            .iter()
            .map(|a| {
                let mut parts = Vec::new();
                if a.required {
                    parts.push("required".to_string());
                }
                if let Some(t) = &a.attr_type {
                    parts.push(format!("type={}", t));
                }
                if let Some(f) = &a.format {
                    parts.push(format!("format={}", f));
                }
                if let Some(u) = &a.unit {
                    parts.push(format!("unit={}", u));
                }
                if let Some(c) = &a.cardinality {
                    parts.push(format!("cardinality={}", c));
                }
                if let Some(values) = &a.entry_values {
                    parts.push(format!("values={}", values.join("|")));
                }
                parts.join("; ")
            })
            .collect();
        writeln!(out, "{}", meta.join(","))?;
    }

    Ok(())
}
