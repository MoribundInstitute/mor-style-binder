# Split Markers

MorStyleBinder can refactor a monolithic CSS or theme file into separate CSS sheets by reading **split markers**.

A split marker is a CSS comment that names the output file for the section that follows it.

```css
/* --- filename.css --- */
```

When MorStyleBinder sees that marker in `import.css`, it treats everything after the marker as belonging to `filename.css` until it reaches the next marker.

---

## Basic example

Input file:

```text
import.css
```

Contents:

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

After clicking:

```text
Inspect Import
Refactor into Sheets
```

MorStyleBinder writes:

```text
src_css/01_base.css
src_css/02_links.css
```

The generated files contain only their own section content.

`src_css/01_base.css`:

```css
body {
  margin: 0;
}
```

`src_css/02_links.css`:

```css
a {
  text-decoration: underline;
}
```

---

## Marker format

Use this exact shape:

```css
/* --- filename.css --- */
```

Recommended:

```css
/* --- 01_base.css --- */
```

```css
/* --- 02_header.css --- */
```

```css
/* --- 03_links.css --- */
```

Avoid unusual marker formats like:

```css
/* filename.css */
```

```css
/* === filename.css === */
```

```css
/*---filename.css---*/
```

MorStyleBinder is designed around the readable marker style:

```css
/* --- filename.css --- */
```

---

## Recommended file naming

Use numbered filenames when order matters.

Good:

```text
00_tokens.css
01_base.css
02_layout.css
03_header.css
04_navigation.css
05_sidebar.css
06_posts.css
07_footer.css
```

Numbered names make the bundle order predictable when files are sorted alphabetically.

Less ideal:

```text
base.css
footer.css
header.css
layout.css
posts.css
tokens.css
```

These are readable, but the bundle order may not be what you expect.

---

## Blogger theme example

A Blogger theme often stores CSS inside a large XML file.

You can copy the relevant CSS into `import.css` and add markers:

```css
/* --- 00_tokens.css --- */
:root {
  --page-bg: #111;
  --text-main: #eee;
}

/* --- 01_base.css --- */
body {
  margin: 0;
  background: var(--page-bg);
  color: var(--text-main);
}

/* --- 04_header.css --- */
.terminal-main-header {
  display: block;
}

/* --- 18_footer.css --- */
.terminal-footer {
  margin-top: 2rem;
}
```

After refactoring, the files appear in:

```text
src_css/
├── 00_tokens.css
├── 01_base.css
├── 04_header.css
└── 18_footer.css
```

---

## Full workflow

### 1. Open or create `import.css`

Use the app button:

```text
Open import.css
```

or create the file manually:

```bash
touch import.css
```

### 2. Paste or drop a monolithic theme

You can paste CSS into `import.css`, or drag a `.css`, `.xml`, or `.txt` theme file into the Import Theme area.

Dropped theme files are copied into:

```text
import.css
```

### 3. Add split markers

Place a marker before each section:

```css
/* --- 01_base.css --- */
```

### 4. Inspect the import

Click:

```text
Inspect Import
```

MorStyleBinder previews the files it will write.

Example preview:

```text
Preview Files to Be Written:
▰ 01_base.css
▰ 02_links.css
```

### 5. Review overwrites

If a target file already exists in `src_css/`, the preview shows it.

```text
Will overwrite existing sheets:
▰ 01_base.css
```

### 6. Refactor into sheets

Click:

```text
Refactor into Sheets
```

If overwrites exist, the button changes to:

```text
Overwrite Existing Sheets
```

Click it again only after reviewing the overwrite list.

---

## Backup behavior

Before overwriting existing files, MorStyleBinder creates a backup in:

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

This means the app can safely replace sheets while preserving the previous versions.

---

## What happens to text before the first marker?

For clean results, put a split marker before the first section you want to keep.

Recommended:

```css
/* --- 01_base.css --- */
body {
  margin: 0;
}
```

Avoid leaving important CSS before the first marker:

```css
body {
  margin: 0;
}

/* --- 02_links.css --- */
a {
  text-decoration: underline;
}
```

If you have comments, metadata, or wrapper text before the first real section, decide whether it belongs in its own file.

Example:

```css
/* --- 00_notes.css --- */
/*
Theme notes:
Copied from live Blogger theme.
*/

/* --- 01_base.css --- */
body {
  margin: 0;
}
```

---

## Good marker habits

Use markers that are:

```text
clear
ordered
stable
specific
```

Good:

```css
/* --- 00_tokens.css --- */
/* --- 01_base.css --- */
/* --- 02_layout.css --- */
/* --- 03_header.css --- */
/* --- 04_navigation.css --- */
/* --- 18_footer.css --- */
```

Less useful:

```css
/* --- stuff.css --- */
/* --- more.css --- */
/* --- old.css --- */
/* --- misc.css --- */
```

A good filename should tell you what kind of theme part lives inside the file.

---

## Suggested section names

For Blogger themes, these names are often useful:

```text
00_tokens.css
01_base.css
02_typography.css
03_layout.css
04_header.css
05_navigation.css
06_sidebar.css
07_catalog-dropdown.css
08_posts.css
09_pages.css
10_widgets.css
11_comments.css
12_archive.css
13_search.css
14_mobile.css
15_utilities.css
18_footer.css
```

Use only the sections your theme actually needs.

---

## Refactoring existing large CSS

When splitting a large file, do it gradually.

Recommended approach:

```text
1. Add markers around a few obvious sections.
2. Inspect Import.
3. Check the preview.
4. Refactor into sheets.
5. Open src_css/ and review the files.
6. Repeat with more sections.
```

This is safer than trying to split a huge theme perfectly in one attempt.

---

## Common mistakes

### Missing `.css` extension

Bad:

```css
/* --- 01_base --- */
```

Good:

```css
/* --- 01_base.css --- */
```

### Duplicate filenames

Avoid using the same filename twice:

```css
/* --- 01_base.css --- */
body {
  margin: 0;
}

/* --- 01_base.css --- */
html {
  font-size: 100%;
}
```

Use one marker per output file.

### Vague filenames

Bad:

```css
/* --- section.css --- */
```

Better:

```css
/* --- 04_header.css --- */
```

### Forgetting to inspect before overwriting

Always review the preview before confirming overwrites.

---

## Quick test file

Use this sample to test the refactor workflow:

```bash
cat > import.css <<'EOF'
/* --- 01_base.css --- */
body {
  margin: 0;
}

/* --- 02_links.css --- */
a {
  text-decoration: underline;
}
EOF
```

Then in the app:

```text
Inspect Import
Refactor into Sheets
Open Refactored Folder
```

Expected files:

```text
src_css/01_base.css
src_css/02_links.css
```

---

## Relationship to exports

Split markers are only used for the import/refactor workflow.

They help turn this:

```text
import.css
```

into this:

```text
src_css/*.css
```

After files are in `src_css/`, MorStyleBinder can bind them back into:

```text
bundle.css
blogger-theme.xml
```

So the full round trip is:

```text
monolithic theme CSS
→ import.css with split markers
→ src_css/*.css
→ bundle.css
→ blogger-theme.xml
```
