use std::path::Path;

use umya_spreadsheet::{new_file, writer::xlsx::{write, XlsxError}};

use crate::model::EntrySchema;

pub struct XlsxOptions {
    pub include_metadata_row: bool,
    pub label_lang: Option<String>,
    pub metadata_lang: Option<String>,
}

pub fn write_xlsx(schema: &EntrySchema, path: &Path, opts: &XlsxOptions) -> Result<(), XlsxError> {
    let mut book = new_file();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // Header row
    for (idx, attr) in schema.attributes.iter().enumerate() {
        let col = column_name(idx + 1);
        let header = if opts.label_lang.is_some() {
            attr.label.clone().unwrap_or_else(|| attr.name.clone())
        } else {
            attr.name.clone()
        };
        let cell = format!("{}1", col);
        sheet.get_cell_mut(cell.as_str()).set_value(header);
    }

    if opts.include_metadata_row {
        let meta_name = "meta";
        let _ = book.new_sheet(meta_name);
        let meta_sheet = book.get_sheet_by_name_mut(meta_name).unwrap();

        meta_sheet.get_cell_mut("A1").set_value("oca_bundle_said");
        meta_sheet.get_cell_mut("B1").set_value(schema.said.clone());

        meta_sheet.get_cell_mut("A3").set_value("attribute");
        meta_sheet.get_cell_mut("B3").set_value("label");
        meta_sheet.get_cell_mut("C3").set_value("type");

        let mut row_meta = 4u32;
        for attr in schema.attributes.iter() {
            let cell = format!("A{}", row_meta);
            meta_sheet.get_cell_mut(cell.as_str()).set_value(attr.name.clone());
            if let Some(label) = &attr.label {
                let cell = format!("B{}", row_meta);
                meta_sheet.get_cell_mut(cell.as_str()).set_value(label.clone());
            }
            if let Some(t) = &attr.attr_type {
                let cell = format!("C{}", row_meta);
                meta_sheet.get_cell_mut(cell.as_str()).set_value(t.clone());
            }
            row_meta += 1;
        }
    }

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
