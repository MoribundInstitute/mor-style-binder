mod core;
mod fs_engine;
mod ui;

use crate::core::{CssModule, ExportMode};
use crate::ui::helpers::{read_css_module_names, TARGET_DIR};
use crate::ui::{
    app_header,
    app_menu_bar,
    bundle_preview_panel,
    dialog_layer,
    import_export_panel,
    module_tray_panel,
    status_strip,
};
use floem::peniko::Color;
use floem::prelude::*;
use floem::reactive::create_effect;
use floem::window::WindowConfig;
use floem::Application;
use notify::{RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::thread;

fn app_view() -> impl IntoView {
    let _ = fs::create_dir_all(TARGET_DIR);

    let css_content = RwSignal::new(String::new());
    let export_mode = RwSignal::new(ExportMode::Standard);
    let file_tick = RwSignal::new(0);

    let status_message = RwSignal::new("Ready.".to_string());
    let module_count = RwSignal::new(0usize);
    let module_names = RwSignal::new(Vec::<String>::new());
    let blogger_vars_detected = RwSignal::new(false);
    let pending_modules = RwSignal::new(Vec::<CssModule>::new());
    let active_dialog = RwSignal::new(None);

    let thread_tick = file_tick;

    thread::spawn(move || {
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                    thread_tick.update(|tick| *tick += 1);
                }
            }
        })
        .expect("Failed to initialize watcher");

        watcher
            .watch(Path::new(TARGET_DIR), RecursiveMode::Recursive)
            .expect("Failed to watch src_css directory");

        loop {
            thread::park();
        }
    });

    create_effect(move |_| {
        let _ = file_tick.get();
        let mode = export_mode.get();

        let rebuilt_css = crate::fs_engine::build_bundle_from_dir(TARGET_DIR, mode);
        css_content.set(rebuilt_css);

        let names = read_css_module_names(TARGET_DIR);
        module_count.set(names.len());
        module_names.set(names);

        blogger_vars_detected.set(Path::new(TARGET_DIR).join("blogger_vars.xml").is_file());
    });

    v_stack((
        app_menu_bar(
            css_content,
            export_mode,
            file_tick,
            status_message,
            pending_modules,
            active_dialog,
        ),

        app_header(export_mode, status_message),

        status_strip(status_message),

        h_stack((
            module_tray_panel(
                module_count,
                module_names,
                blogger_vars_detected,
                status_message,
                file_tick,
            ),

            bundle_preview_panel(css_content),

            import_export_panel(
                css_content,
                export_mode,
                file_tick,
                status_message,
                pending_modules,
            ),
        ))
        .style(|s| s.items_start()),

        dialog_layer(active_dialog),
    ))
    .style(|s| {
        s.size_full()
            .padding(18.0)
            .background(Color::rgb8(15, 15, 15))
    })
}

fn main() {
    Application::new()
        .window(
            |_| app_view(),
            Some(
                WindowConfig::default()
                    .title("Style Binder")
                    .size((1220.0, 760.0)),
            ),
        )
        .run();
}