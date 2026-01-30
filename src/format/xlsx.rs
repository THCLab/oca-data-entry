use std::path::Path;

use umya_spreadsheet::{new_file, writer::xlsx::{write, XlsxError}};

use crate::model::EntrySchema;

pub struct XlsxOptions {
    pub include_metadata_row: bool,
    pub use_labels: bool,
}

pub fn write_xlsx(schema: &EntrySchema, path: &Path, opts: &XlsxOptions) -> Result<(), XlsxError> {
    let mut book = new_file();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // Header row
    for (idx, attr) in schema.attributes.iter().enumerate() {
        let col = column_name(idx + 1);
        let header = if opts.use_labels {
            attr.label.clone().unwrap_or_else(|| attr.name.clone())
        } else {
            attr.name.clone()
        };
        let cell = format!("{}1", col);
        sheet.get_cell_mut(cell.as_str()).set_value(header);
    }

    let mut row = 2;

    if opts.include_metadata_row {
        for (idx, attr) in schema.attributes.iter().enumerate() {
            let col = column_name(idx + 1);
            let mut parts = Vec::new();
            if attr.required {
                parts.push("required".to_string());
            }
            if let Some(t) = &attr.attr_type {
                parts.push(format!("type={}", t));
            }
            if let Some(f) = &attr.format {
                parts.push(format!("format={}", f));
            }
            if let Some(u) = &attr.unit {
                parts.push(format!("unit={}", u));
            }
            if let Some(c) = &attr.cardinality {
                parts.push(format!("cardinality={}", c));
            }
            if let Some(values) = &attr.entry_values {
                parts.push(format!("values={}", values.join("|")));
            }
            let cell = format!("{}{}", col, row);
            sheet.get_cell_mut(cell.as_str()).set_value(parts.join("; "));
        }
        row += 1;
    }

    // Bundle SAID row (A{row} key, B{row} value)
    let cell_a = format!("A{}", row);
    sheet.get_cell_mut(cell_a.as_str()).set_value("__oca_bundle_said__");
    let cell_b = format!("B{}", row);
    sheet.get_cell_mut(cell_b.as_str()).set_value(schema.said.clone());

    write(&book, path)?;
    Ok(())
}

fn column_name(mut idx: usize) -> String {
    let mut name = String::new();
    while idx > 0 {
        let rem = (idx - 1) % 26;
        name.insert(0, (b'A' + rem as u8) as char);
        idx = (idx - 1) / 26;
    }
    name
}
