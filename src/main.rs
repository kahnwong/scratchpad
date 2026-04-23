use adw::gtk::{gdk, Box, CssProvider, Orientation, ScrolledWindow, ToggleButton, WrapMode};
use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};
use serde_json::Value;
use sourceview5::prelude::*;
use sourceview5::{Buffer, LanguageManager, StyleSchemeManager, View};

const APP_ID: &str = "io.github.kahnwong.Scratchpad";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let css = CssProvider::new();
    css.load_from_string("textview { font-size: 12pt; }");
    adw::gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css,
        adw::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let buffer = Buffer::new(None);

    let scheme_manager = StyleSchemeManager::default();
    if let Some(scheme) = scheme_manager.scheme("Adwaita-dark") {
        buffer.set_style_scheme(Some(&scheme));
    }

    let source_view = View::with_buffer(&buffer);
    source_view.set_wrap_mode(WrapMode::Word);
    source_view.set_monospace(true);
    source_view.set_left_margin(16);
    source_view.set_right_margin(16);
    source_view.set_top_margin(16);
    source_view.set_bottom_margin(16);

    let scrolled = ScrolledWindow::builder()
        .child(&source_view)
        .vexpand(true)
        .hexpand(true)
        .build();

    let header = HeaderBar::new();
    header.set_title_widget(Some(&Box::new(Orientation::Horizontal, 0)));
    header.set_decoration_layout(Some(":close"));

    let json_button = ToggleButton::builder()
        .label("JSON")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    let bottom_bar = Box::new(Orientation::Horizontal, 0);
    bottom_bar.append(&json_button);

    let buffer_clone = buffer.clone();
    json_button.connect_toggled(move |btn| {
        if btn.is_active() {
            let lang_manager = LanguageManager::default();
            if let Some(lang) = lang_manager.language("json") {
                buffer_clone.set_language(Some(&lang));
            }
            let (start, end) = buffer_clone.bounds();
            let text = buffer_clone.text(&start, &end, false);
            if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                if let Ok(pretty) = serde_json::to_string_pretty(&parsed) {
                    buffer_clone.set_text(&pretty);
                }
            }
        } else {
            buffer_clone.set_language(None);
        }
    });

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.add_bottom_bar(&bottom_bar);
    toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_bottom_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_content(Some(&scrolled));

    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(640)
        .default_height(480)
        .build();

    window.present();
}
