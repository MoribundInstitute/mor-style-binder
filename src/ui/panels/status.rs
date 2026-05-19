use floem::peniko::Color;
use floem::prelude::*;

pub fn status_strip(status_message: RwSignal<String>) -> impl IntoView {
    label(move || format!("Status: {}", status_message.get())).style(|s| {
        s.padding(10.0)
            .margin_top(12.0)
            .border(1.0)
            .border_radius(4.0)
            .border_color(Color::rgb8(45, 45, 45))
            .background(Color::rgb8(10, 10, 10))
            .font_size(13.0)
            .color(Color::rgb8(190, 190, 190))
    })
}
