// src/fs_engine.rs
//
// Desktop filesystem adapter.
// This is the place where std::fs, paths, and walkdir are allowed to live.

use crate::core::{self, CssModule, ExportMode};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Reads a directory of CSS files, sorts them predictably, and delegates
/// pure bundle construction to `core.rs`.
pub fn build_bundle_from_dir<P: AsRef<Path>>(dir: P, mode: ExportMode) -> String {
    let dir_ref = dir.as_ref();

    let blogger_vars = if mode == ExportMode::BloggerXml {
        let vars_path = dir_ref.join("blogger_vars.xml");
        fs::read_to_string(vars_path).ok()
    } else {
        None
    };

    let modules = read_css_modules_from_dir(dir_ref);

    core::build_bundle_from_modules(&modules, mode, blogger_vars.as_deref())
}

/// Loads all `.css` files from a directory into in-memory modules.
///
/// `blogger_vars.xml` is deliberately ignored here because it is not a CSS
/// module. It is handled separately by `build_bundle_from_dir`.
fn read_css_modules_from_dir(dir: &Path) -> Vec<CssModule> {
    let mut paths: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "css"))
        .map(|entry| entry.path().to_owned())
        .collect();

    paths.sort();

    paths
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(&path).ok()?;
            let name = path.file_name()?.to_string_lossy().to_string();

            Some(CssModule::new(name, content))
        })
        .collect()
}