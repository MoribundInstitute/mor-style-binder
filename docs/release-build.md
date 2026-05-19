# Release Build Notes

This document describes how to check, test, build, and prepare MorStyleBinder for a simple release.

MorStyleBinder is a Rust desktop app built with Floem. It also contains pure core logic that can be compiled for Wasm.

---

## Development check

Before creating a release build, make sure the project compiles.

```bash
cargo check
```

Run the tests:

```bash
cargo test
```

Run the app locally:

```bash
cargo run
```

A clean pre-release pass should look like:

```text
cargo check
cargo test
cargo run
```

with no compile errors.

---

## Desktop release build

Build the optimized desktop binary:

```bash
cargo build --release
```

The release binary should be created at:

```text
target/release/style_binder
```

Run it directly:

```bash
./target/release/style_binder
```

---

## Suggested release test checklist

Before publishing a release, manually test the core workflows.

### Modular CSS workflow

```text
[ ] Drop a .css file into the Modular Sheets panel.
[ ] Confirm the file appears in src_css/.
[ ] Confirm the Bundle Preview refreshes.
[ ] Confirm non-CSS drops show a warning.
```

### Export workflow

```text
[ ] Click Save bundle.css.
[ ] Confirm bundle.css is created.
[ ] Click Export Blogger Skin.
[ ] Confirm blogger-theme.xml is created.
[ ] Click Open Export Folder.
[ ] Confirm the export folder opens.
```

### Import/refactor workflow

```text
[ ] Add split markers to import.css.
[ ] Click Inspect Import.
[ ] Confirm Preview Files to Be Written appears.
[ ] Click Refactor into Sheets.
[ ] Confirm files appear in src_css/.
[ ] Click Open Refactored Folder.
```

### Overwrite safety workflow

```text
[ ] Use import.css to target an existing file in src_css/.
[ ] Click Inspect Import.
[ ] Confirm the overwrite warning lists exact filenames.
[ ] Click Overwrite Existing Sheets.
[ ] Confirm src_css_backup/ is created.
[ ] Confirm overwritten files are backed up.
```

### UI workflow

```text
[ ] Hover over major buttons.
[ ] Confirm status-bar tips appear.
[ ] Open Help → Usage Guide.
[ ] Open Help → Split Marker Help.
[ ] Open Help → About.
```

---

## Clean local test files before release

During development, the following files may be temporary:

```text
src_css/test_sheet.css
src_css/01_drag_base.css
src_css/02_drag_links.css
src_css/98_refactor_status_test.css
src_css/99_new_test.css
src_css/boobootest.css
src_css_backup/
bundle.css
blogger-theme.xml
import.css
```

Decide which example files should remain in the repository.

For a cleaner release tree, remove temporary files:

```bash
rm -f src_css/test_sheet.css
rm -f src_css/01_drag_base.css
rm -f src_css/02_drag_links.css
rm -f src_css/98_refactor_status_test.css
rm -f src_css/99_new_test.css
rm -f src_css/boobootest.css
rm -rf src_css_backup
rm -f bundle.css
rm -f blogger-theme.xml
rm -f import.css
```

Only do this if those files are not intended to be committed as examples.

---

## Suggested `.gitignore`

Use a `.gitignore` so generated files do not get committed accidentally.

```gitignore
/target/
/pkg/
/src_css_backup/
/bundle.css
/blogger-theme.xml
/import.css

.DS_Store
*.swp
*.swo
```

Do not ignore `src_css/` if you want to ship example CSS files or keep real project sheets in version control.

---

## Wasm build

The app has pure core logic exposed for future Wasm use.

Build the Wasm package with:

```bash
wasm-pack build . --target web --no-default-features --features wasm
```

This creates or updates:

```text
pkg/
```

The Wasm package is useful for future web-based tooling, but it is not required for normal desktop releases.

---

## Linux notes

For a simple local Linux build:

```bash
cargo build --release
```

Then run:

```bash
./target/release/style_binder
```

A future packaged Linux release should include:

```text
style_binder binary
.desktop file
app icon
README or docs
```

Possible install paths:

```text
/usr/bin/style_binder
/usr/share/applications/mor-style-binder.desktop
/usr/share/icons/hicolor/512x512/apps/mor-style-binder.png
```

Packaging can be added later for:

```text
Arch package
Debian package
AppImage
```

---

## Windows notes

For the simplest Windows release, build on Windows.

Recommended approach:

```text
1. Clone the repository on Windows.
2. Install Rust.
3. Run cargo build --release.
4. Test target/release/style_binder.exe.
```

The output should be:

```text
target/release/style_binder.exe
```

Cross-compiling Windows builds from Linux is possible, but building on Windows is usually easier for desktop GUI apps.

---

## Release folder idea

A simple manual release folder could look like:

```text
release/
├── style_binder
├── README.md
├── docs/
│   ├── usage-guide.md
│   ├── split-markers.md
│   └── release-build.md
└── screenshots/
```

For Windows:

```text
release/
├── style_binder.exe
├── README.md
├── docs/
└── screenshots/
```

---

## GitHub release checklist

Before creating a GitHub release:

```text
[ ] cargo check passes.
[ ] cargo test passes.
[ ] cargo build --release passes.
[ ] App opens successfully.
[ ] Modular CSS drop workflow works.
[ ] Import/refactor workflow works.
[ ] Backup-before-overwrite works.
[ ] Export bundle.css works.
[ ] Export blogger-theme.xml works.
[ ] README.md is current.
[ ] docs/ are current.
[ ] screenshots/ contains current screenshots if needed.
[ ] Temporary test files are removed or intentionally kept.
[ ] Version number is updated if applicable.
```

---

## Current release maturity

MorStyleBinder is currently best treated as a development/pre-release tool.

It is already useful for:

```text
binding CSS sheets
exporting Blogger-safe CSS
inspecting import.css
splitting monolithic themes with split markers
backing up overwritten sheets
```

Future release polish may include:

```text
settings dialog
app icon
Linux desktop file
Arch package
Debian package
Windows build notes
Save As dialogs
```
