# oca-data-entry

Generate data-entry templates (XLSX/ODS/CSV) from OCA bundles.

This crate focuses on **data capture templates** only. It does not generate bundles.

## Features

- `writers-csv` (default: off)
- `writers-xlsx` (default: off)
- `writers-ods` (default: off)

## Example (CSV)

```rust
use oca_data_entry::{entry_schema_from_bundle, write_csv};
use oca_data_entry::format::csv::CsvOptions;
use oca_sdk_rs::overlay_registry::OverlayLocalRegistry;

# // load bundle ...
# let bundle = todo!();
let registry = OverlayLocalRegistry::from_dir("./core_overlays").unwrap();
let schema = entry_schema_from_bundle(&bundle, &registry).unwrap();

let mut out = Vec::new();
write_csv(&schema, &mut out, &CsvOptions { include_metadata_row: true, use_labels: true }).unwrap();
```

## Roadmap

- XLSX/ODS writers
- Overlay enrichment (labels, conformance, formats, units)
- Reading filled templates for validation pipelines

## CLI

Use the `oca` CLI (in oca-bin) once the data-entry command is wired in there.
