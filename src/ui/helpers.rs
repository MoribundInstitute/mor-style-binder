use crate::core::{self, CssModule, ExportMode};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TARGET_DIR: &str = "./src_css";
pub const BACKUP_DIR: &str = "./src_css_backup";
pub const IMPORT_FILE: &str = "./import.css";
pub const STANDARD_EXPORT_FILE: &str = "./bundle.css";
pub const BLOGGER_EXPORT_FILE: &str = "./blogger-theme.xml";
pub const PROJECT_ROOT: &str = ".";
pub const SPLIT_MARKER_EXAMPLE: &str = "/* --- filename.css --- */";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssImportReport {
    pub imported_count: usize,
    pub ignored_count: usize,
    pub overwritten: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeImportReport {
    pub imported_path: PathBuf,
    pub detected_modules: Vec<CssModule>,
    pub overwritten: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackupReport {
    pub backed_up_count: usize,
    pub backup_dir: Option<PathBuf>,
    pub backed_up_files: Vec<String>,
}

pub fn read_css_module_names(dir: &str) -> Vec<String> {
    let mut names = match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter(|entry| is_css_file(&entry.path()))
            .filter_map(|entry| {
                entry
                    .path()
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    names.sort();
    names
}

pub fn summarize_module_names(modules: &[CssModule]) -> String {
    let mut names: Vec<String> = modules.iter().map(|module| module.name.clone()).collect();
    names.sort();

    let shown: Vec<String> = names.iter().take(5).cloned().collect();

    if names.len() > shown.len() {
        format!("{} … and {} more", shown.join(", "), names.len() - shown.len())
    } else {
        shown.join(", ")
    }
}

pub fn find_overwrites(modules: &[CssModule], output_dir: &str) -> Vec<String> {
    modules
        .iter()
        .filter(|module| Path::new(output_dir).join(&module.name).exists())
        .map(|module| module.name.clone())
        .collect()
}


pub fn backup_overwritten_modules(
    modules: &[CssModule],
    output_dir: &str,
    backup_root: &str,
) -> std::io::Result<BackupReport> {
    let mut overwritten = find_overwrites(modules, output_dir);
    overwritten.sort();

    if overwritten.is_empty() {
        return Ok(BackupReport {
            backed_up_count: 0,
            backup_dir: None,
            backed_up_files: Vec::new(),
        });
    }

    let backup_dir = Path::new(backup_root).join(timestamped_backup_folder_name());
    fs::create_dir_all(&backup_dir)?;

    for file_name in &overwritten {
        let source = Path::new(output_dir).join(file_name);
        let destination = backup_dir.join(file_name);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, destination)?;
    }

    Ok(BackupReport {
        backed_up_count: overwritten.len(),
        backup_dir: Some(backup_dir),
        backed_up_files: overwritten,
    })
}

fn timestamped_backup_folder_name() -> String {
    let unix_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    format!("backup_{unix_seconds}")
}

pub fn write_modules_to_dir(modules: &[CssModule], output_dir: &str) -> std::io::Result<usize> {
    fs::create_dir_all(output_dir)?;

    for module in modules {
        fs::write(Path::new(output_dir).join(&module.name), &module.content)?;
    }

    Ok(modules.len())
}

pub fn copy_css_files_to_target_dir<P: AsRef<Path>>(
    paths: &[P],
    target_dir: &str,
) -> std::io::Result<CssImportReport> {
    fs::create_dir_all(target_dir)?;

    let target_root = Path::new(target_dir);
    let mut imported_count = 0usize;
    let mut ignored_count = 0usize;
    let mut overwritten = Vec::new();

    for path in paths {
        let source = path.as_ref();

        if !source.is_file() || !is_css_file(source) {
            ignored_count += 1;
            continue;
        }

        let Some(file_name) = source.file_name() else {
            ignored_count += 1;
            continue;
        };

        let destination = target_root.join(file_name);

        if destination.exists() {
            overwritten.push(file_name.to_string_lossy().to_string());
        }

        fs::copy(source, destination)?;
        imported_count += 1;
    }

    overwritten.sort();

    Ok(CssImportReport {
        imported_count,
        ignored_count,
        overwritten,
    })
}

pub fn copy_theme_file_to_import_css<P: AsRef<Path>>(
    source_path: P,
    import_file: &str,
    target_dir: &str,
) -> std::io::Result<ThemeImportReport> {
    let source = source_path.as_ref();

    if !source.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Dropped theme path is not a file.",
        ));
    }

    if !is_theme_import_file(source) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Theme import must be a .css, .xml, or .txt file.",
        ));
    }

    fs::copy(source, import_file)?;

    let monolith = fs::read_to_string(import_file)?;
    let detected_modules = core::unbind_bundle_to_modules(&monolith);
    let overwritten = find_overwrites(&detected_modules, target_dir);

    Ok(ThemeImportReport {
        imported_path: source.to_path_buf(),
        detected_modules,
        overwritten,
    })
}

pub fn inspect_import_file(
    import_file: &str,
    target_dir: &str,
) -> std::io::Result<ThemeImportReport> {
    let monolith = fs::read_to_string(import_file)?;
    let detected_modules = core::unbind_bundle_to_modules(&monolith);
    let overwritten = find_overwrites(&detected_modules, target_dir);

    Ok(ThemeImportReport {
        imported_path: PathBuf::from(import_file),
        detected_modules,
        overwritten,
    })
}

pub fn ensure_import_file_exists(import_file: &str) -> std::io::Result<bool> {
    let path = Path::new(import_file);

    if path.exists() {
        return Ok(false);
    }

    let starter = "/* --- 01_base.css --- */\nbody {\n  margin: 0;\n}\n";
    fs::write(path, starter)?;

    Ok(true)
}

pub fn clear_import_file(import_file: &str) -> std::io::Result<()> {
    fs::write(import_file, "")
}

pub fn save_bundle_to_file(output_file: &str, mode: ExportMode) -> std::io::Result<()> {
    let bundle = crate::fs_engine::build_bundle_from_dir(TARGET_DIR, mode);
    fs::write(output_file, bundle)
}

pub fn open_existing_path(path: &str) -> std::io::Result<()> {
    if !Path::new(path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{path} does not exist yet."),
        ));
    }

    open_path(path)
}

pub fn is_css_file(path: &Path) -> bool {
    has_extension(path, "css")
}

pub fn is_theme_import_file(path: &Path) -> bool {
    has_extension(path, "css") || has_extension(path, "xml") || has_extension(path, "txt")
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

pub fn open_path(path: &str) -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", "", path]).spawn()?;
    }

    Ok(())
}