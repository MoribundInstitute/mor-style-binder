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
pub fn unbind_bundle_to_modules(monolith: &str) -> Vec<CssModule> {
    let mut modules = Vec::new();

    let mut current_file: Option<String> = None;
    let mut current_content = String::new();

    for line in monolith.lines() {
        let trimmed = line.trim();

        // Strip Blogger skin wrappers if the pasted/imported file contains them.
        if trimmed.contains("<b:skin>")
            || trimmed.contains("</b:skin>")
            || trimmed.contains("<![CDATA[")
            || trimmed.contains("]]>")
        {
            continue;
        }

        if let Some(name) = parse_demarcation(trimmed) {
            // Flush the previous module before switching to the next one.
            if let Some(previous_name) = current_file.take() {
                if !current_content.trim().is_empty() {
                    modules.push(CssModule::new(previous_name, current_content.clone()));
                }

                current_content.clear();
            }

            // Ignore the global generated header.
            if name == "Auto-Generated CSS Bundle" {
                current_file = None;
                continue;
            }

            current_file = Some(name);
        } else if current_file.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Flush the final module.
    if let Some(name) = current_file {
        if !current_content.trim().is_empty() {
            modules.push(CssModule::new(name, current_content));
        }
    }

    modules
}

/// Detects markers like:
///
/// /* --- 01_tokens.css --- */
fn parse_demarcation(line: &str) -> Option<String> {
    if !line.starts_with("/* --- ") || !line.ends_with(" --- */") {
        return None;
    }

    let name = line
        .trim_start_matches("/* --- ")
        .trim_end_matches(" --- */")
        .trim();

    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}