// src/wasm_api.rs
//
// Wasm-facing API for style_binder.
//
// This file is only compiled when the `wasm` feature is enabled.
// It wraps the pure core.rs logic in wasm-bindgen-friendly functions.

#![cfg(feature = "wasm")]

use crate::core::{build_bundle_from_modules, unbind_bundle_to_modules, CssModule, ExportMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmCssModule {
    pub name: String,
    pub content: String,
}

impl From<WasmCssModule> for CssModule {
    fn from(value: WasmCssModule) -> Self {
        CssModule::new(value.name, value.content)
    }
}

impl From<CssModule> for WasmCssModule {
    fn from(value: CssModule) -> Self {
        Self {
            name: value.name,
            content: value.content,
        }
    }
}

/// Build a Standard CSS bundle from a JavaScript array of modules.
///
/// Expected JS input:
///
/// [
///   { name: "01_base.css", content: "body { margin: 0; }" },
///   { name: "02_links.css", content: "a { text-decoration: underline; }" }
/// ]
#[wasm_bindgen]
pub fn wasm_build_standard_bundle(modules: JsValue) -> Result<String, JsValue> {
    let wasm_modules: Vec<WasmCssModule> = serde_wasm_bindgen::from_value(modules)?;

    let core_modules: Vec<CssModule> = wasm_modules.into_iter().map(CssModule::from).collect();

    Ok(build_bundle_from_modules(
        &core_modules,
        ExportMode::Standard,
        None,
    ))
}

/// Build a Blogger XML bundle from a JavaScript array of modules.
///
/// `blogger_vars` may be empty, null-ish from the caller's side, or a string
/// containing Blogger `<Variable ... />` declarations.
#[wasm_bindgen]
pub fn wasm_build_blogger_xml_bundle(
    modules: JsValue,
    blogger_vars: Option<String>,
) -> Result<String, JsValue> {
    let wasm_modules: Vec<WasmCssModule> = serde_wasm_bindgen::from_value(modules)?;

    let core_modules: Vec<CssModule> = wasm_modules.into_iter().map(CssModule::from).collect();

    Ok(build_bundle_from_modules(
        &core_modules,
        ExportMode::BloggerXml,
        blogger_vars.as_deref(),
    ))
}

/// Split a monolithic CSS/Blogger skin string into a JavaScript array of modules.
///
/// Returned JS shape:
///
/// [
///   { name: "01_base.css", content: "body { margin: 0; }\n" },
///   { name: "02_links.css", content: "a { text-decoration: underline; }\n" }
/// ]
#[wasm_bindgen]
pub fn wasm_unbind_theme(monolith: &str) -> Result<JsValue, JsValue> {
    let modules: Vec<WasmCssModule> = unbind_bundle_to_modules(monolith)
        .into_iter()
        .map(WasmCssModule::from)
        .collect();

    serde_wasm_bindgen::to_value(&modules).map_err(Into::into)
}
