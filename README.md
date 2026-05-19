# MorStyleBinder

MorStyleBinder is a lightweight desktop workbench for binding modular CSS sheets into one theme, or refactoring a monolithic Blogger/Tumblr/SpaceHey-style theme into modular files.

It is built for theme workflows where one huge stylesheet becomes hard to edit, review, reuse, or safely paste back into a platform like Blogger.

## Screenshot

![MorStyleBinder main workbench](screenshots/morstylebinder-main.png)

## What it does

MorStyleBinder helps with both directions of a theme workflow:

```text
many readable CSS files → one exportable theme bundle
```

and:

```text
one large theme file → many readable CSS files
```

The app can:

- read modular `.css` sheets from `src_css/`
- generate a live bundle preview
- export a normal `bundle.css`
- export a Blogger-safe `blogger-theme.xml` using a `<b:skin><![CDATA[ ... ]]></b:skin>` wrapper
- copy the generated bundle CSS to the clipboard
- open the export folder so generated files are easy to find
- inspect a monolithic theme file through `import.css`
- split a monolithic theme into separate sheets using split markers
- preview files before writing them
- warn when refactoring would overwrite existing sheets
- back up overwritten sheets into `src_css_backup/`

## Current app layout

The desktop UI is arranged as a three-panel workbench:

```text
Modular Sheets | Bundle Preview | Import Theme / Export
```

### Modular Sheets

The left panel shows CSS files currently loaded from `src_css/`.

You can drop `.css` files anywhere on this panel. They are copied into `src_css/` and the bundle preview is refreshed.

### Bundle Preview

The center panel shows the generated output.

The preview changes depending on the current binding mode:

- `Plain CSS Bundle`
- `Blogger Skin Wrapper`

### Import Theme / Export

The right panel handles two related workflows:

- importing/refactoring a monolithic theme into modular sheets
- exporting the generated bundle as plain CSS or Blogger XML

## Folder structure

MorStyleBinder expects this basic project layout:

```text
.
├── src_css/
│   ├── 01_base.css
│   ├── 02_links.css
│   └── ...
├── import.css
├── bundle.css
├── blogger-theme.xml
└── src_css_backup/
```

### `src_css/`

This is the main source folder for modular CSS sheets.

All `.css` files in this folder are bundled into the generated preview and exports.

### `import.css`

This is the import inbox for monolithic theme CSS.

Paste or drop a large theme into this file, then use split markers to tell MorStyleBinder where each output file should begin.

### `bundle.css`

This is the plain CSS export.

### `blogger-theme.xml`

This is the Blogger-safe export wrapped like this:

```xml
<b:skin><![CDATA[
/* generated CSS goes here */
]]></b:skin>
```

### `src_css_backup/`

When refactoring would overwrite existing files, MorStyleBinder backs up the old files here before writing the new versions.

Example:

```text
src_css_backup/
└── backup_1779208539/
    ├── 01_base.css
    └── 02_links.css
```

## Split markers

A split marker is a CSS comment that tells MorStyleBinder where a new output file begins.

Use this format:

```css
/* --- filename.css --- */
```

Example `import.css`:

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

When you inspect and refactor that import, MorStyleBinder writes:

```text
src_css/01_base.css
src_css/02_links.css
```

## Refactor safety

MorStyleBinder is designed to avoid silent destructive writes.

When `import.css` contains split markers that would overwrite existing files, the app:

1. previews the files that will be written
2. shows the exact files that would be overwritten
3. changes the action button to `Overwrite Existing Sheets`
4. waits for a second confirmation click
5. backs up overwritten files into `src_css_backup/`
6. writes the new refactored sheets into `src_css/`

This keeps the workflow fast while still giving you a recovery path.

## Export modes

### Plain CSS Bundle

This mode exports regular CSS to:

```text
bundle.css
```

Use this when you want a normal stylesheet.

### Blogger Skin Wrapper

This mode exports Blogger-safe XML to:

```text
blogger-theme.xml
```

Use this when you need to paste CSS into a Blogger theme skin block.

## Basic workflow

### Binding modular sheets into one bundle

```text
1. Put `.css` files into `src_css/`.
2. Open MorStyleBinder.
3. Review the Bundle Preview panel.
4. Choose Plain CSS Bundle or Blogger Skin Wrapper.
5. Click Save bundle.css or Export Blogger Skin.
6. Use Open Export Folder to find the generated files.
```

### Refactoring a monolithic theme into sheets

```text
1. Put a monolithic theme into `import.css`, or drop a `.css`, `.xml`, or `.txt` theme into Import Theme.
2. Add split markers before each section.
3. Click Inspect Import.
4. Review the files that will be written.
5. If overwrites are detected, review the warning.
6. Click Refactor into Sheets or Overwrite Existing Sheets.
7. Use Open Refactored Folder to view the output in `src_css/`.
```

## Status-bar help

MorStyleBinder uses the status bar for hover help.

Hover over buttons and controls to see a short explanation in the status strip. This is used instead of floating tooltips because it is more reliable in the current desktop layout.

## Development

### Requirements

Current stack:

```text
Rust 2021
Floem 0.2.0 GUI
notify 6.1.1 file watching
walkdir 2.4.0 directory traversal
copypasta 0.10.2 clipboard
wasm-bindgen optional for future Wasm
```

### Run locally

```bash
cargo check
cargo test
cargo run
```

### Release build

```bash
cargo build --release
```

The release binary will be created under:

```text
target/release/
```

## Current source organization

The UI is split into focused modules:

```text
src/ui/
├── buttons.rs
├── dialogs.rs
├── helpers.rs
├── menu.rs
├── mod.rs
├── tooltips.rs
└── panels/
    ├── mod.rs
    ├── header.rs
    ├── status.rs
    ├── modular_sheets.rs
    ├── bundle_preview.rs
    ├── import_theme.rs
    └── export.rs
```

The core logic is kept separate from the desktop UI where possible:

```text
src/core.rs       # pure domain/string logic
src/fs_engine.rs  # desktop filesystem adapter
src/wasm_api.rs   # wasm-bindgen wrappers for future Wasm usage
```

## Future ideas

Planned or possible polish items include:

- Save As for `bundle.css`
- Save As for `blogger-theme.xml`
- settings saved to `config.toml`
- app icon assets
- Linux desktop packaging
- Arch and Debian package notes
- Windows build notes
- more complete docs and screenshots

## Related workflow

MorStyleBinder is especially useful for Blogger theme repositories that keep CSS in readable modules, such as:

```text
css/sections/
css/full-theme.css
xml/live/
theme-parts/
```

It helps move between human-editable files and Blogger-ready exports without manually juggling one giant theme block.
