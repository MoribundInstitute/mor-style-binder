use floem::event::EventListener;
use floem::prelude::*;

/// Shows static hover-help in the existing status strip.
///
/// Floem's popup tooltip wrapper can be hard to see in this desktop layout,
/// so MorStyleBinder uses the status bar as the tooltip surface instead.
pub fn status_tip<V>(
    child: V,
    status_message: RwSignal<String>,
    tip_text: &'static str,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    status_tip_fn(child, status_message, move || tip_text)
}

/// Shows dynamic hover-help in the status strip.
///
/// This is useful for menu command rows, where the visible command changes
/// when File/Edit/View/Tools/Help changes.
pub fn status_tip_fn<V, F>(
    child: V,
    status_message: RwSignal<String>,
    tip_text: F,
) -> impl IntoView
where
    V: IntoView + 'static,
    F: Fn() -> &'static str + Copy + 'static,
{
    let previous_status = RwSignal::new(None::<String>);

    child
        .into_view()
        .on_event_stop(EventListener::PointerEnter, move |_| {
            let text = tip_text();

            if text.is_empty() {
                return;
            }

            previous_status.set(Some(status_message.get()));
            status_message.set(format!("Tip: {text}"));
        })
        .on_event_stop(EventListener::PointerLeave, move |_| {
            if let Some(previous) = previous_status.get() {
                if status_message.get().starts_with("Tip: ") {
                    status_message.set(previous);
                }
            }

            previous_status.set(None);
        })
}
