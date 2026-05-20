// src/core.rs
//
// Pure domain logic only.
// No std::fs.
// No walkdir.
// No platform-specific filesystem assumptions.
//
// This file is intended to stay friendly to future Wasm/Web Component reuse.

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExportMode {
    Standard,
    BloggerXml,
}

/// One named CSS module held in memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssModule {
    pub name: String,
    pub content: String,
}

impl CssModule {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }
}

/// Builds a bundle from already-loaded CSS modules.
///
/// Filesystem loading belongs in `fs_engine.rs`.
/// This function only receives names + strings and returns a string.
pub fn build_bundle_from_modules(
    modules: &[CssModule],
    mode: ExportMode,
    blogger_vars: Option<&str>,
) -> String {
    let mut bundle = String::new();

    if mode == ExportMode::BloggerXml {
        if let Some(vars) = blogger_vars {
            if !vars.trim().is_empty() {
                bundle.push('\n');
                bundle.push_str(vars);
                bundle.push_str("\n\n");
            }
        }

        bundle.push_str("<b:skin>\n  <![CDATA[\n");
    }

    bundle.push_str("/* --- Auto-Generated CSS Bundle --- */\n\n");

    for module in modules {
        bundle.push_str(&format!("/* --- {} --- */\n", module.name));
        bundle.push_str(&module.content);

        if !module.content.ends_with('\n') {
            bundle.push('\n');
        }

        bundle.push('\n');
    }

    if mode == ExportMode::BloggerXml {
        bundle.push_str("  ]]>\n</b:skin>\n");
    }

    bundle
}

/// Parses a monolithic CSS/Blogger skin bundle into named in-memory modules.
///
/// File writing belongs in `fs_engine.rs`.
/// This function only receives a string and returns split modules.
///
/// Tolerant marker examples:
///
/// ```css
/// /* --- 01_tokens.css --- */
/// /* 01_tokens.css */
/// /*Root-Section.css */
/// /* === 01-reset.css === */
/// /* SECTION: 04-Main-Header.css */
/// /* FILE: layout/header.css */
/// /* =========================================================
///    01-Reset-Base.css
///    ========================================================= */
/// ```
pub fn unbind_bundle_to_modules(monolith: &str) -> Vec<CssModule> {
    let mut modules = Vec::new();

    let mut current_file: Option<String> = None;
    let mut current_content = String::new();

    let mut pending_comment = String::new();
    let mut inside_comment = false;

    for line in monolith.lines() {
        let trimmed = line.trim();

        // Strip Blogger skin wrappers if the pasted/imported file contains them.
        if is_blogger_wrapper_line(trimmed) {
            continue;
        }

        // Continue collecting a multi-line CSS block comment.
        if inside_comment {
            pending_comment.push('\n');
            pending_comment.push_str(line);

            if trimmed.contains("*/") {
                inside_comment = false;

                if let Some(name) = parse_demarcation(&pending_comment) {
                    flush_module(&mut modules, &mut current_file, &mut current_content);

                    if is_generated_bundle_header(&name) {
                        current_file = None;
                    } else {
                        current_file = Some(name);
                    }

                    pending_comment.clear();
                    continue;
                }

                // Not a marker, so preserve the ordinary comment inside the current sheet.
                if current_file.is_some() {
                    current_content.push_str(&pending_comment);
                    current_content.push('\n');
                }

                pending_comment.clear();
            }

            continue;
        }

        // Start collecting a CSS block comment.
        if trimmed.starts_with("/*") {
            pending_comment.clear();
            pending_comment.push_str(line);

            // One-line comment.
            if trimmed.contains("*/") {
                if let Some(name) = parse_demarcation(&pending_comment) {
                    flush_module(&mut modules, &mut current_file, &mut current_content);

                    if is_generated_bundle_header(&name) {
                        current_file = None;
                    } else {
                        current_file = Some(name);
                    }

                    pending_comment.clear();
                    continue;
                }

                // Not a marker, so preserve the ordinary comment inside the current sheet.
                if current_file.is_some() {
                    current_content.push_str(line);
                    current_content.push('\n');
                }

                pending_comment.clear();
                continue;
            }

            // Multi-line comment begins. Wait until we see the closing marker.
            inside_comment = true;
            continue;
        }

        if current_file.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // If the file ended while a non-marker comment was still open, preserve it.
    if inside_comment && current_file.is_some() && !pending_comment.trim().is_empty() {
        current_content.push_str(&pending_comment);
        current_content.push('\n');
    }

    flush_module(&mut modules, &mut current_file, &mut current_content);

    modules
}

fn flush_module(
    modules: &mut Vec<CssModule>,
    current_file: &mut Option<String>,
    current_content: &mut String,
) {
    if let Some(previous_name) = current_file.take() {
        if !current_content.trim().is_empty() {
            modules.push(CssModule::new(previous_name, current_content.clone()));
        }

        current_content.clear();
    }
}

fn is_blogger_wrapper_line(line: &str) -> bool {
    line.contains("<b:skin>")
        || line.contains("</b:skin>")
        || line.contains("<![CDATA[")
        || line.contains("]]>")
}

fn is_generated_bundle_header(name: &str) -> bool {
    name.eq_ignore_ascii_case("Auto-Generated CSS Bundle")
}

/// Detects a CSS filename inside almost any CSS block comment.
///
/// Examples accepted:
///
/// ```css
/// /* --- 01_tokens.css --- */
/// /* 01_tokens.css */
/// /*Root-Section.css */
/// /* === 01-reset.css === */
/// /* SECTION: 04-Main-Header.css */
/// /* FILE: layout/header.css */
/// /* =========================================================
///    01-Reset-Base.css
///    ========================================================= */
/// ```
fn parse_demarcation(comment: &str) -> Option<String> {
    let cleaned = comment
        .replace("/*", " ")
        .replace("*/", " ")
        .replace("---", " ")
        .replace("===", " ")
        .replace("***", " ")
        .replace("___", " ");

    for raw_token in cleaned.split_whitespace() {
        let token = clean_marker_token(raw_token);

        if let Some(name) = extract_css_filename(token) {
            return Some(name);
        }
    }

    None
}

fn clean_marker_token(token: &str) -> &str {
    token
        .trim()
        .trim_matches('*')
        .trim_matches('-')
        .trim_matches('=')
        .trim_matches('_')
        .trim_matches(':')
        .trim_matches(';')
        .trim_matches(',')
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches('`')
        .trim_matches('[')
        .trim_matches(']')
        .trim_matches('(')
        .trim_matches(')')
        .trim_matches('{')
        .trim_matches('}')
        .trim_matches('<')
        .trim_matches('>')
}

fn extract_css_filename(token: &str) -> Option<String> {
    let lower = token.to_ascii_lowercase();
    let css_pos = lower.find(".css")?;
    let end = css_pos + ".css".len();

    let mut candidate = &token[..end];

    // Allow paths in comments, but keep only the final file name.
    if let Some(last_slash) = candidate.rfind('/') {
        candidate = &candidate[last_slash + 1..];
    }

    if let Some(last_slash) = candidate.rfind('\\') {
        candidate = &candidate[last_slash + 1..];
    }

    if is_valid_css_module_name(candidate) {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn is_valid_css_module_name(name: &str) -> bool {
    !name.is_empty()
        && name.to_ascii_lowercase().ends_with(".css")
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains(':')
        && !name.contains('\0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_generated_markers() {
        let input = r#"
/* --- 01_base.css --- */
body {
  margin: 0;
}

/* --- 02_buttons.css --- */
button {
  cursor: pointer;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "01_base.css");
        assert!(modules[0].content.contains("margin: 0"));
        assert_eq!(modules[1].name, "02_buttons.css");
        assert!(modules[1].content.contains("cursor: pointer"));
    }

    #[test]
    fn parses_compact_marker_comments() {
        let input = r#"
/*Root-Section.css */
:root {
  --bg-base: #0a0a0a;
}

/*Buttons.css*/
button {
  border: 1px solid currentColor;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "Root-Section.css");
        assert_eq!(modules[1].name, "Buttons.css");
    }

    #[test]
    fn parses_big_banner_markers() {
        let input = r#"
/* =========================================================
   01-Reset-Base.css
   ========================================================= */

* {
  box-sizing: border-box;
}

/* =========================================================
   02-Typography-Links-Glow.css
   ========================================================= */

a {
  text-decoration: none;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "01-Reset-Base.css");
        assert!(modules[0].content.contains("box-sizing"));
        assert_eq!(modules[1].name, "02-Typography-Links-Glow.css");
        assert!(modules[1].content.contains("text-decoration"));
    }

    #[test]
    fn parses_noisy_comment_markers() {
        let input = r#"
/* SECTION: 04-Main-Header.css */
.terminal-main-header {
  display: flex;
}

/* lol whatever goblin note — 10-Side-Panels.css */
.runelite-panel {
  width: 300px;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "04-Main-Header.css");
        assert_eq!(modules[1].name, "10-Side-Panels.css");
    }

    #[test]
    fn keeps_only_final_filename_from_paths() {
        let input = r#"
/* FILE: layout/header.css */
header {
  display: block;
}

/* FILE: C:\themes\footer.css */
footer {
  display: block;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "header.css");
        assert_eq!(modules[1].name, "footer.css");
    }

    #[test]
    fn strips_blogger_wrappers() {
        let input = r#"
<b:skin>
  <![CDATA[
/* wrapped.css */
body {
  color: black;
}
  ]]>
</b:skin>
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "wrapped.css");
        assert!(modules[0].content.contains("color: black"));
        assert!(!modules[0].content.contains("<b:skin>"));
        assert!(!modules[0].content.contains("<![CDATA["));
    }

    #[test]
    fn preserves_non_marker_comments_inside_modules() {
        let input = r#"
/* base.css */
body {
  margin: 0;
}

/* This ordinary comment should stay. */
html {
  min-height: 100%;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "base.css");
        assert!(modules[0]
            .content
            .contains("/* This ordinary comment should stay. */"));
    }

    #[test]
    fn ignores_auto_generated_bundle_header() {
        let input = r#"
/* --- Auto-Generated CSS Bundle --- */

/* --- base.css --- */
body {
  margin: 0;
}
"#;

        let modules = unbind_bundle_to_modules(input);

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "base.css");
    }
}
