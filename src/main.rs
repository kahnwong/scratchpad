use adw::gtk::{gdk, Box, CssProvider, Orientation, ScrolledWindow, ToggleButton, WrapMode};
use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use sourceview5::prelude::*;
use sourceview5::{Buffer, LanguageManager, StyleSchemeManager, View};
use std::cell::Cell;
use std::rc::Rc;

const APP_ID: &str = "io.github.kahnwong.Scratchpad";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn make_button(label: &str) -> ToggleButton {
    ToggleButton::builder()
        .label(label)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build()
}

fn deactivate_others(current: &ToggleButton, all: &[ToggleButton], guard: &Rc<Cell<bool>>) {
    guard.set(true);
    for btn in all {
        if btn != current {
            btn.set_active(false);
        }
    }
    guard.set(false);
}

fn connect_lang_button(
    button: &ToggleButton,
    lang_id: &'static str,
    buffer: Buffer,
    all_buttons: Rc<Vec<ToggleButton>>,
    guard: Rc<Cell<bool>>,
) {
    button.connect_toggled(move |btn| {
        if guard.get() {
            return;
        }
        if btn.is_active() {
            deactivate_others(btn, &all_buttons, &guard);
            let lang_manager = LanguageManager::default();
            if let Some(lang) = lang_manager.language(lang_id) {
                buffer.set_language(Some(&lang));
            }
        } else {
            buffer.set_language(None);
        }
    });
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
    source_view.set_insert_spaces_instead_of_tabs(true);
    source_view.set_tab_width(2);
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

    let json_button = make_button("JSON");
    let yaml_button = make_button("YAML");
    let bash_button = make_button("Bash");
    let go_button = make_button("Go");
    let rust_button = make_button("Rust");
    let python_button = make_button("Python");

    let bottom_bar = Box::new(Orientation::Horizontal, 0);
    for btn in [
        &json_button,
        &yaml_button,
        &bash_button,
        &go_button,
        &rust_button,
        &python_button,
    ] {
        bottom_bar.append(btn);
    }

    let guard = Rc::new(Cell::new(false));
    let all_buttons = Rc::new(vec![
        json_button.clone(),
        yaml_button.clone(),
        bash_button.clone(),
        go_button.clone(),
        rust_button.clone(),
        python_button.clone(),
    ]);

    {
        let buffer = buffer.clone();
        let all = all_buttons.clone();
        let guard = guard.clone();
        json_button.connect_toggled(move |btn| {
            if guard.get() {
                return;
            }
            if btn.is_active() {
                deactivate_others(btn, &all, &guard);
                let lang_manager = LanguageManager::default();
                if let Some(lang) = lang_manager.language("json") {
                    buffer.set_language(Some(&lang));
                }
                let (start, end) = buffer.bounds();
                let text = buffer.text(&start, &end, false);
                let parsed = serde_json::from_str::<JsonValue>(&text)
                    .or_else(|_| serde_yaml::from_str::<JsonValue>(&text));
                if let Ok(v) = parsed {
                    if let Ok(pretty) = serde_json::to_string_pretty(&v) {
                        buffer.set_text(&pretty);
                    }
                }
            } else {
                buffer.set_language(None);
            }
        });
    }

    {
        let buffer = buffer.clone();
        let all = all_buttons.clone();
        let guard = guard.clone();
        yaml_button.connect_toggled(move |btn| {
            if guard.get() {
                return;
            }
            if btn.is_active() {
                deactivate_others(btn, &all, &guard);
                let lang_manager = LanguageManager::default();
                if let Some(lang) = lang_manager.language("yaml") {
                    buffer.set_language(Some(&lang));
                }
                let (start, end) = buffer.bounds();
                let text = buffer.text(&start, &end, false);
                let parsed = serde_yaml::from_str::<YamlValue>(&text)
                    .or_else(|_| serde_json::from_str::<YamlValue>(&text));
                if let Ok(v) = parsed {
                    if let Ok(pretty) = serde_yaml::to_string(&v) {
                        buffer.set_text(&pretty);
                    }
                }
            } else {
                buffer.set_language(None);
            }
        });
    }

    connect_lang_button(
        &bash_button,
        "sh",
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_lang_button(
        &go_button,
        "go",
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_lang_button(
        &rust_button,
        "rust",
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_lang_button(
        &python_button,
        "python3",
        buffer.clone(),
        all_buttons,
        guard,
    );

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.add_bottom_bar(&bottom_bar);
    toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_bottom_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_content(Some(&scrolled));

    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(1400)
        .default_height(900)
        .build();

    window.present();
}
