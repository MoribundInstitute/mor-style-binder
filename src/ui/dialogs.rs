use floem::peniko::Color;
use floem::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogKind {
    About,
    SplitMarkerHelp,
    UsageGuide,
}

pub fn dialog_layer(active_dialog: RwSignal<Option<DialogKind>>) -> impl IntoView {
    v_stack((
        h_stack((
            label(move || dialog_title(active_dialog.get()).to_string()).style(|s| {
                s.font_size(16.0)
                    .color(Color::rgb8(230, 230, 230))
                    .margin_right(12.0)
            }),

            label(|| "Close".to_string())
                .on_click_stop(move |_| active_dialog.set(None))
                .style(move |s| {
                    let hidden = active_dialog.get().is_none();

                    s.padding_horiz(10.0)
                        .padding_vert(5.0)
                        .border(1.0)
                        .border_radius(3.0)
                        .apply_if(hidden, |s| {
                            s.width(0.0)
                                .height(0.0)
                                .padding(0.0)
                                .border(0.0)
                                .color(Color::rgb8(15, 15, 15))
                        })
                        .apply_if(!hidden, |s| {
                            s.border_color(Color::rgb8(100, 100, 100))
                                .background(Color::rgb8(25, 25, 25))
                                .color(Color::rgb8(220, 220, 220))
                        })
                }),
        ))
        .style(|s| s.items_center().margin_bottom(8.0)),

        label(move || dialog_body(active_dialog.get()).to_string()).style(move |s| {
            let hidden = active_dialog.get().is_none();

            s.font_size(13.0)
                .line_height(1.45)
                .apply_if(hidden, |s| s.color(Color::rgb8(15, 15, 15)))
                .apply_if(!hidden, |s| s.color(Color::rgb8(180, 180, 180)))
        }),
    ))
    .style(move |s| {
        let hidden = active_dialog.get().is_none();

        s.width_full()
            .apply_if(hidden, |s| {
                s.height(0.0)
                    .padding(0.0)
                    .margin_bottom(0.0)
                    .border(0.0)
                    .background(Color::rgb8(15, 15, 15))
            })
            .apply_if(!hidden, |s| {
                s.padding(12.0)
                    .margin_bottom(10.0)
                    .border(1.0)
                    .border_radius(4.0)
                    .border_color(Color::rgb8(70, 70, 70))
                    .background(Color::rgb8(10, 10, 10))
            })
    })
}

fn dialog_title(dialog: Option<DialogKind>) -> &'static str {
    match dialog {
        Some(DialogKind::About) => "About MorStyleBinder",
        Some(DialogKind::SplitMarkerHelp) => "Split Marker Help",
        Some(DialogKind::UsageGuide) => "Usage Guide",
        None => "",
    }
}

fn dialog_body(dialog: Option<DialogKind>) -> &'static str {
    match dialog {
        Some(DialogKind::About) => {
            "MorStyleBinder is a lightweight desktop workbench for binding modular CSS sheets into one exportable theme, and refactoring monolithic Blogger/Tumblr/SpaceHey-style themes back into manageable files.\n\nThe app is built around a small CSS bundle/refactor engine: src_css/ is the modular source folder, import.css is the monolithic-theme inbox, and bundle.css or blogger-theme.xml are the export targets."
        }
        Some(DialogKind::SplitMarkerHelp) => {
            "To split a monolithic theme into separate sheets, place a marker before each section:\n\n/* --- filename.css --- */\n\nExample:\n\n/* --- 01_base.css --- */\nbody { margin: 0; }\n\n/* --- 02_links.css --- */\na { text-decoration: underline; }\n\nThen use Inspect Import to preview the detected files. Use Refactor into Sheets to write them into src_css/."
        }
        Some(DialogKind::UsageGuide) => {
            "Basic workflow:\n\n1. Drop modular .css sheets into Modular Sheets to bundle them.\n2. Use Plain CSS Bundle or Blogger Skin Wrapper to choose the output format.\n3. Drop a monolithic theme file into Import Theme to inspect split markers.\n4. Review Preview Files to Be Written before writing anything.\n5. Use Refactor into Sheets to write detected sections into src_css/.\n6. Use Export to copy or save bundle.css or blogger-theme.xml.\n\nSafety note:\nIf imported sections would overwrite existing files, MorStyleBinder previews the filenames first. Choose Overwrite Existing Sheets only after reviewing the list, or use Cancel Pending Import to back out."
        }
        None => "",
    }
}
