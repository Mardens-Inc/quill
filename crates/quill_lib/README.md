# quill_lib

A Windows printing library for Rust, focused on enumerating printers and sending
raw jobs to label printers. It wraps the Win32 print spooler (`winspool`) APIs and
includes a helper for rasterising images into TSPL bitmap commands for thermal
label printers (e.g. TSC).

## Platform

**Windows only.** `quill_lib` builds on the [`windows`](https://crates.io/crates/windows)
crate and calls the Win32 spooler directly (`EnumPrinters`, `OpenPrinter`,

[//]: # (`StartDocPrinter`, `WritePrinter`, ...&#41;. It will not compile or run on other)
platforms.

## Features

- Enumerate locally available printers and read their status and metadata.
- Open a printer handle and query the data types its print processor supports.
- Send arbitrary bytes to a printer as a spooler job (`RAW` by default).
- Render an `image::RgbImage` to a TSPL job sized to a physical label stock,
  with rotation, fit-to-label scaling, and gap registration for die-cut labels.

## Installation

`quill_lib` is part of this workspace and is consumed as a path dependency:

```toml
[dependencies]
quill_lib = { path = "../quill_lib" }
```

It depends on `windows`, `image` (with `png`/`jpeg`), `thiserror`, and `tracing`.

## Quick start

### List printers

```rust
use quill_lib::printers::Printers;

let printers = Printers::get_available_printers() ?;
for printer in printers {
println!("[{:?}] {}", printer.status, printer.printer_name);
}
```

### Print an image to a label printer

```rust
use quill_lib::print_orientation::PageOrientation;
use quill_lib::printers::Printers;
use quill_lib::stock::Stock;

// Find a printer by (partial) name and open a handle.
let printers = Printers::get_available_printers() ?;
let printer = printers
.iter()
.find( | p| p.printer_name.to_lowercase().contains("tsc"))
.expect("no matching printer");
let handle = printer.handle() ?;

// Load an image.
let bytes = std::fs::read("tag.png") ?;
let img = image::load_from_memory( & bytes) ?.into_rgb8();

// Describe the physical media: a 1in x 0.75in die-cut label with 0.05in of
// exposed liner on each side and a 0.12in gap between labels.
let stock = Stock::inches(1.0, 0.75)
.with_exposed_liner(0.05, 0.05)
.with_gap(0.12);

handle.print_png("my-job", & img, stock, PageOrientation::Rotate180, 1.0) ?;
```

## API overview

| Module              | Item                                              | Purpose                                                                             |
|---------------------|---------------------------------------------------|-------------------------------------------------------------------------------------|
| `printers`          | `Printers::get_available_printers()`              | Enumerate local printers as `Vec<PrinterInfo>`.                                     |
| `printers`          | `Printers::get_printer_handle(name)`              | Open a `PrinterHandle` by printer name.                                             |
| `printer_info`      | `PrinterInfo`                                     | Printer metadata (name, port, driver, status, ...). Call `.handle()` to open it.    |
| `printer_status`    | `PrinterStatus`                                   | Spooler status flags (`Idle`, `Offline`, `PaperOut`, ...).                          |
| `printer_handle`    | `PrinterHandle::print_bytes(...)`                 | Spool raw bytes as a print job.                                                     |
| `printer_handle`    | `PrinterHandle::print_png(...)`                   | Rasterise an image into a TSPL label job.                                           |
| `printer_handle`    | `PrinterHandle::get_print_processor_data_types()` | List data types the print processor accepts.                                        |
| `printer_handle`    | `PrinterHandle::close()`                          | Close the underlying handle.                                                        |
| `stock`             | `Stock`                                           | Physical label media: size, exposed liner, gap, unit.                               |
| `print_orientation` | `PageOrientation`                                 | Rotation applied before rasterising (`Normal`, `Rotate90/180/270`, `Degrees(f32)`). |
| `errors`            | `QuillError`                                      | Library error type.                                                                 |

### Stock

`Stock` mirrors a printer driver's "Edit Stock" dialog and controls how
`print_png` lays out the label:

```rust
use quill_lib::stock::Stock;

// Construct in inches or millimetres, then refine with builder methods.
let stock = Stock::millimeters(25.4, 19.05)
.with_exposed_liner(1.27, 1.27) // liner exposed left/right of the label
.with_gap(3.0);                 // feed-direction gap between die-cut labels
```

- The label `width`/`height` drive the TSPL `SIZE` command.
- The image is scaled to fill the printable area (label minus exposed liner),
  preserving aspect ratio, and centred on the label. The `scale` argument to
  `print_png` zooms on top of that fit (`1.0` = fill).
- The `gap` drives the TSPL `GAP` command so the printer registers on each
  label. Leaving it `0.0` selects continuous media and will cause multi-label
  prints to drift off the labels.

## Error handling

All fallible calls return `Result<_, QuillError>`. Variants cover string/FFI
conversion failures, handle-open failures, use of an unopened handle, and a
transparent wrapper around `windows::core::Error`.

## Examples

Runnable examples live in `examples/`:

- `list_printers` — enumerate printers and their status.
- `list_printer_processor_data_types` — print each printer's supported data types.
- `print_tag` — render `tag.png` to a TSC label printer using a `Stock`.

```sh
cargo run -p quill_lib --example list_printers
```

## Notes

- `print_png` emits TSPL (`SIZE`, `GAP`, `CLS`, `BITMAP`, `PRINT`) and assumes a
  203 dpi (8 dots/mm) print head. It targets TSPL-compatible thermal label
  printers; use `print_bytes` for other raw command languages.
- Printing bypasses the GDI driver pipeline, so driver-side page setup, margins,
  and stock settings do not affect `print_png` output — the TSPL job carries its
  own geometry.
