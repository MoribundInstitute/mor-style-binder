// src/lib.rs
//
// Library entry point for style_binder.
//
// This file exposes the pure domain layer for future Wasm/Web Component use.
// Keep this file free of desktop-only modules such as fs_engine, notify, Floem,
// clipboard handling, or thread-based file watching.

pub mod core;

#[cfg(feature = "desktop")]
pub mod settings;

#[cfg(feature = "wasm")]
pub mod wasm_api;

pub use crate::core::{build_bundle_from_modules, unbind_bundle_to_modules, CssModule, ExportMode};

/// Convenience wrapper for Standard CSS output.
///
/// Useful for future Wasm bindings because callers can pass plain vectors of
/// in-memory modules without touching the filesystem.
pub fn build_standard_bundle(modules: &[CssModule]) -> String {
    build_bundle_from_modules(modules, ExportMode::Standard, None)
}

/// Convenience wrapper for Blogger XML output.
///
/// `blogger_vars` may contain the contents of `blogger_vars.xml`.
pub fn build_blogger_xml_bundle(modules: &[CssModule], blogger_vars: Option<&str>) -> String {
    build_bundle_from_modules(modules, ExportMode::BloggerXml, blogger_vars)
}

/// Convenience wrapper for splitting a monolithic theme string into modules.
///
/// This does not write files. Desktop file writing belongs in `fs_engine.rs`.
pub fn unbind_theme(monolith: &str) -> Vec<CssModule> {
    unbind_bundle_to_modules(monolith)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_standard_bundle_from_modules() {
        let modules = vec![
            CssModule::new("01_base.css", "body { margin: 0; }"),
            CssModule::new("02_links.css", "a { text-decoration: underline; }"),
        ];

        let bundle = build_standard_bundle(&modules);

        assert!(bundle.contains("/* --- Auto-Generated CSS Bundle --- */"));
        assert!(bundle.contains("/* --- 01_base.css --- */"));
        assert!(bundle.contains("body { margin: 0; }"));
        assert!(bundle.contains("/* --- 02_links.css --- */"));
        assert!(bundle.contains("a { text-decoration: underline; }"));
        assert!(!bundle.contains("<b:skin>"));
    }

    #[test]
    fn builds_blogger_xml_bundle_from_modules() {
        let modules = vec![CssModule::new("01_base.css", "body { margin: 0; }")];

        let bundle =
            build_blogger_xml_bundle(&modules, Some("<Variable name=\"body.background\"/>"));

        assert!(bundle.contains("<b:skin>"));
        assert!(bundle.contains("<![CDATA["));
        assert!(bundle.contains("<Variable name=\"body.background\"/>"));
        assert!(bundle.contains("/* --- 01_base.css --- */"));
        assert!(bundle.contains("body { margin: 0; }"));
        assert!(bundle.contains("]]>"));
        assert!(bundle.contains("</b:skin>"));
    }

    #[test]
    fn unbinds_theme_into_modules() {
        let monolith = r#"
/* --- Auto-Generated CSS Bundle --- */

/* --- 01_base.css --- */
body {
  margin: 0;
}

/* --- 02_links.css --- */
a {
  text-decoration: underline;
}
"#;

        let modules = unbind_theme(monolith);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "01_base.css");
        assert!(modules[0].content.contains("body"));
        assert_eq!(modules[1].name, "02_links.css");
        assert!(modules[1].content.contains("text-decoration"));
    }
}
