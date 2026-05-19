# MorStyleBinder Usage Guide

MorStyleBinder is a desktop workbench for managing theme CSS as smaller, safer files. It helps with two opposite workflows:

```text
many modular CSS sheets → one exportable theme bundle
```

and:

```text
one monolithic theme file → many modular CSS sheets
```

The app is especially useful for Blogger themes, where CSS often ends up inside one large XML theme file.

---

## Main layout

MorStyleBinder uses a three-panel layout:

```text
Modular Sheets | Bundle Preview | Import Theme / Export
```

### Modular Sheets

The left panel shows the CSS files currently loaded from:

```text
src_css/
```

You can drop `.css` files anywhere in the Modular Sheets panel. MorStyleBinder copies them into `src_css/` and refreshes the bundle preview.

### Bundle Preview

The center panel shows the generated bundle output.

The preview changes depending on the selected mode:

```text
Plain CSS Bundle
Blogger Skin Wrapper
```

### Import Theme / Export

The right panel handles two workflows:

```text
Import Theme
```

for inspecting and splitting monolithic theme files.

```text
Export
```

for copying or saving generated output.

---

## Basic modular CSS workflow

Use this workflow when you already have separate CSS files.

### 1. Add CSS files

Place `.css` files in:

```text
src_css/
```

or drag `.css` files into the Modular Sheets panel.

Example:

```text
src_css/
├── 01_base.css
├── 02_header.css
├── 03_sidebar.css
└── 04_footer.css
```

### 2. Review the bundle preview

The center Bundle Preview panel updates from the contents of `src_css/`.

### 3. Choose an output mode

Use the mode buttons near the top of the app.

```text
Plain CSS Bundle
```

shows normal bundled CSS.

```text
Blogger Skin Wrapper
```

shows Blogger-safe XML output wrapped as:

```xml
<b:skin><![CDATA[
  /* CSS goes here */
]]></b:skin>
```

### 4. Export the bundle

Use the Export section.

Available actions:

```text
Copy Bundle CSS
Save bundle.css
Open bundle.css
Export Blogger Skin
Open blogger-theme.xml
Open Export Folder
```

`Save bundle.css` writes:

```text
bundle.css
```

`Export Blogger Skin` writes:

```text
blogger-theme.xml
```

---

## Blogger export workflow

Use this workflow when preparing CSS for a Blogger theme.

### 1. Put CSS files in `src_css/`

Use one file per theme section.

Example:

```text
src_css/
├── 00_tokens.css
├── 01_base.css
├── 04_header.css
├── 07_catalog-dropdown.css
└── 18_footer.css
```

### 2. Select Blogger Skin Wrapper

Click:

```text
Blogger Skin Wrapper
```

The preview should switch to a Blogger-safe wrapper.

### 3. Export Blogger Skin

Click:

```text
Export Blogger Skin
```

MorStyleBinder writes:

```text
blogger-theme.xml
```

### 4. Open the export folder

Click:

```text
Open Export Folder
```

This opens the project folder so you can find:

```text
bundle.css
blogger-theme.xml
```

This is useful if your operating system does not know how to open `.xml` files directly.

---

## Import and refactor workflow

Use this workflow when you have one large theme file and want to split it into smaller CSS files.

### 1. Prepare `import.css`

You can either:

```text
Open import.css
```

and paste a monolithic theme into it, or drag a `.css`, `.xml`, or `.txt` theme file into the Import Theme drop area.

MorStyleBinder copies dropped theme files to:

```text
import.css
```

### 2. Add split markers

MorStyleBinder splits files using marker comments.

```css
/* --- filename.css --- */
```

Example:

```css
/* --- 01_base.css --- */
body {
  margin: 0;
}

/* --- 02_links.css --- */
a {
  text-decoration: underline;
}
```

Each marker becomes a file name in `src_css/`.

The example above writes:

```text
src_css/01_base.css
src_css/02_links.css
```

### 3. Inspect the import

Click:

```text
Inspect Import
```

MorStyleBinder scans `import.css` and previews the files it found.

The preview can show:

```text
Preview Files to Be Written:
▰ 01_base.css
▰ 02_links.css
```

If any of those files already exist, it also shows:

```text
Will overwrite existing sheets:
▰ 01_base.css
```

### 4. Refactor into sheets

Click:

```text
Refactor into Sheets
```

If no existing files would be replaced, MorStyleBinder writes the detected sections into:

```text
src_css/
```

If existing files would be replaced, the button changes to:

```text
Overwrite Existing Sheets
```

Click it again only after reviewing the overwrite list.

---

## Backup behavior

MorStyleBinder backs up overwritten sheets before replacing them.

Backups are written to:

```text
src_css_backup/
```

Example:

```text
src_css_backup/
└── backup_1779208539/
    ├── 01_base.css
    └── 02_links.css
```

This protects existing files when refactoring a monolithic theme into modular sheets.

---

## Useful buttons

### Open Modular Sheets Folder

Opens:

```text
src_css/
```

Use this to inspect or edit modular sheets directly.

### Open import.css

Opens:

```text
import.css
```

Use this as the inbox for monolithic theme files.

### Inspect Import

Scans `import.css` for split markers and previews the output files before writing anything.

### Refactor into Sheets

Writes detected sections from `import.css` into `src_css/`.

### Cancel Pending Import

Clears the in-memory import preview without changing `import.css` or `src_css/`.

### Open Refactored Folder

Opens:

```text
src_css/
```

This is useful after a refactor.

### Clear import.css

Empties:

```text
import.css
```

and clears the pending import preview.

### Copy Bundle CSS

Copies the generated bundle preview to the clipboard.

### Save bundle.css

Writes the plain CSS bundle to:

```text
bundle.css
```

### Export Blogger Skin

Writes Blogger-safe XML to:

```text
blogger-theme.xml
```

### Open Export Folder

Opens the project folder containing exported files.

---

## Status bar tips

MorStyleBinder uses the status bar for hover help.

Hover over buttons and controls to see a short explanation in the status strip.

Example:

```text
Status: Tip: Save a Blogger-compatible <b:skin> XML wrapper to ./blogger-theme.xml.
```

---

## Suggested workflow for Blogger theme editing

A practical Blogger workflow looks like this:

```text
1. Edit modular CSS files in src_css/.
2. Review the Bundle Preview.
3. Select Blogger Skin Wrapper.
4. Export Blogger Skin.
5. Open blogger-theme.xml.
6. Copy the generated <b:skin> block into Blogger XML.
7. Export the full Blogger XML from Blogger.
8. Commit both the readable source files and the exported theme file.
```

Example commit:

```bash
git status
git add src_css/ bundle.css blogger-theme.xml
git commit -m "Update Blogger theme CSS bundle"
git push
```

---

## Safety checklist before overwriting

Before clicking `Overwrite Existing Sheets`, check:

```text
[ ] The preview lists the files you expect.
[ ] The overwrite list only contains files you are willing to replace.
[ ] You understand backups will be written to src_css_backup/.
[ ] You can open src_css/ afterward to inspect the result.
```

---

## Development commands

Run the app during development:

```bash
cargo run
```

Check for compile errors:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Build a release binary:

```bash
cargo build --release
```

---

## File map

Common project files:

```text
src_css/              Modular CSS sheets
src_css_backup/       Backups created before overwrites
import.css            Monolithic theme import inbox
bundle.css            Plain CSS export
blogger-theme.xml     Blogger-safe XML export
```

Common source files:

```text
src/ui/panels/        Main panel views
src/ui/menu.rs        Top command menu
src/ui/tooltips.rs    Status-bar hover help
src/ui/helpers.rs     File operations and workflow helpers
src/core.rs           Pure binding/refactor logic
```
