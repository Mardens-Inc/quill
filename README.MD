# Quill
[![wakatime](https://wakatime.com/badge/user/ff32311f-9fbb-4819-9f8b-f2fd0665c2a6/project/d7c36658-f50d-416e-a681-d657f2ac7e4c.svg)](https://wakatime.com/badge/user/ff32311f-9fbb-4819-9f8b-f2fd0665c2a6/project/d7c36658-f50d-416e-a681-d657f2ac7e4c)   
A local print agent that lets the Mardens Pricing App print price/label tags to a
TSC label printer **without the browser's native print dialog**.

Quill is a small Rust service that runs on each workstation. It listens on
localhost, receives a label from the Pricing App, and sends raw bytes straight to
the locally-attached TSC printer through the Windows print spooler. The browser
never prints anything, so there is no dialog to suppress — it has simply been
removed from the path.

> The name is a placeholder; rename freely. "Quill" was chosen as a sibling to the
> existing *Quay* project — a short, archaic marking instrument that does exactly
> what this agent does: put a mark on a surface on command.

---

## The problem

The Pricing App is a web app (Actix + React, served centrally) used at 15+ store
locations. Today, printing a label routes through `pricetagger.mardens.com`, which
returns rendered **HTML**, which is opened in a window and printed via the
browser's native print dialog.

The requirement: the user clicks once and a label comes out. No dialog, no printer
picker, no extra clicks.

Browsers deliberately block silent printing — `window.print()` always shows the
dialog, and there is no web API that pushes a job to a specific printer without
user interaction. So a website cannot do this on its own. Quill is the piece that
bridges the gap: the web app talks to a trusted local helper, and the helper talks
to the hardware.

## Target hardware

- **Printers:** TSC TX200 / TX210 (203 dpi thermal transfer, 4-inch desktop).
- **Language:** TSPL / TSPL-EZD (the printers' native command language; they also
  emulate ZPL, EPL, and DPL).
- **Connection:** USB, one printer per workstation.
- **OS:** Windows. The TSC driver is already installed, so each printer shows up as
  a normal Windows print queue.

---

## What Quill is

- A **single static Rust binary** deployed to every workstation.
- A **localhost HTTP/WebSocket server** the Pricing App can reach from the browser.
- A **thin wrapper over the Windows print spooler** that sends raw TSPL to the
  target printer using the `RAW` datatype (spooler pass-through — the bytes reach
  the printer untouched, with no rendering and no dialog).
- Runs unattended: a tray app or Windows service that starts with the machine.

## What Quill is **not**

- It is **not** a label designer or renderer. Label layout/content lives on the
  server side; Quill receives a print-ready payload and forwards it. (See
  "Open decisions" — this boundary still needs to be locked in.)
- It does **not** use WebUSB / libusb / Zadig. Replacing the `usbprint.sys` driver
  with WinUSB would break normal driver-based printing across every store. Quill
  keeps the stock TSC driver intact and rides the spooler instead.
- It does **not** depend on the browser's `--kiosk-printing` flag. That flag is a
  viable stopgap (prints silently to the *default* printer) but can't target a
  specific printer and is purely a per-machine browser setting, not part of this
  project.

---

## How it fits together

```
Pricing App (browser)
        │   fetch / WebSocket to 127.0.0.1
        ▼
     Quill (local agent on the workstation)
        │   winspool RAW pass-through
        ▼
  Windows print spooler ──► USB port ──► TSC TX200 / TX210
```

The Pricing App's existing "auto print" toggle (`AutoPrintButton`) selects the
path: when on, the print action posts to Quill instead of opening the HTML print
window.

---

## What Quill needs to do

### Core responsibilities

1. **Listen locally.** Bind an HTTP (and/or WebSocket) endpoint on
   `127.0.0.1:<port>`. The port should be fixed/configurable and documented so the
   Pricing App knows where to reach it.
2. **Accept print requests** from the Pricing App. Because the app is served over
   HTTPS and Quill is `http://localhost`, expect a CORS preflight (Private Network
   Access) — respond with the headers needed to allow it. `http://localhost` is
   exempt from mixed-content blocking, so the request itself is permitted.
3. **Resolve the target printer** by its Windows print-queue name. Support a
   configured default and ideally let a request name a specific printer.
4. **Send the job.** Open the printer queue, start a document with the **`RAW`**
   datatype, write the TSPL byte buffer, and close it cleanly
   (`OpenPrinter` → `StartDocPrinter` → `StartPagePrinter` → `WritePrinter` →
   `EndPagePrinter` → `EndDocPrinter` → `ClosePrinter`).
5. **Report results.** Return success/failure to the caller with enough detail for
   the UI to surface a useful message (printer not found, spooler error, offline,
   out of media, etc.). Don't fail silently — a tag that didn't print needs to be
   visible to the user.
6. **Log.** Keep a local rolling log of jobs and errors for remote troubleshooting
   across stores.

### Operational responsibilities

- **Run unattended.** Auto-start on boot (tray app or Windows service) and survive
  reboots without manual intervention.
- **Be deployable at scale.** One binary, no runtime to install, easy to push to
  15+ locations. A self-update mechanism is desirable so fixes don't require a
  store visit.
- **Be configurable.** Port, default printer name, and log level via a small config
  file or environment, with sane defaults so a fresh install "just works."

### Nice-to-have / later

- Health/status endpoint (is the printer present and ready?) so the app can show
  printer state.
- Test-print / calibration command.
- Support for the networked path: the TX200/TX210 also accept TSPL on TCP port
  9100, so the same payload could be sent over the network if a printer is ever
  moved off USB. Designing the "send bytes" layer to be transport-agnostic keeps
  this option open.

---

## Tech stack

- **Language:** Rust. The printing core is a dozen FFI calls; the value is in the
  surrounding server, JSON, packaging, and multi-store maintainability, where Rust
  and Cargo are the clear fit for a single-developer, single-binary tool.
- **Windows API:** `windows-sys` for the raw spooler bindings (a near 1:1 mirror of
  `winspool.h`). All `unsafe` is confined to one small wrapper module exposing a
  single safe "send these bytes to this printer" function; the rest of the agent
  never sees `unsafe` or wide strings.
- **Server:** Axum or Actix + Tokio. The spooler calls are blocking, so run them on
  `spawn_blocking` rather than the async runtime.
- **Serialization:** serde for the request/response payloads.
- **Correctness:** clippy as the standard check.

---

## Interface (draft)

A single print endpoint is enough to start. Illustrative shape — not final:

- `POST /print` with a JSON body containing the target printer name and the
  print-ready TSPL payload (or the structured label data, depending on the boundary
  decision below). Returns a job result with status and any error detail.
- `GET /status` (later) reporting agent health and printer readiness.

---

## Open decisions

1. **Where does TSPL get built — server or agent?**
    - *Server emits ready-to-print TSPL:* keeps all label logic in one place
      (`pricetagger.mardens.com`), keeps every store identical, and makes Quill a
      dumb, stable forwarder. Recommended for a sole-dev, multi-store setup.
    - *Agent builds TSPL from structured data:* more logic per workstation, more to
      keep in sync across stores. Generally not worth it here.
2. **Tray app vs Windows service** for the run-unattended requirement.
3. **Fixed vs configurable port,** and how the Pricing App discovers it.
4. **Authentication** on the local endpoint — even on localhost, decide whether a
   shared token or origin check is warranted to keep other local processes from
   issuing print jobs.

---

## Status

Early design. Architecture and approach are settled (local Rust agent, spooler RAW
pass-through, TSPL). Implementation not yet started. The first milestone is the
end-to-end happy path: app click → `POST /print` → label out of the TX200.