use adw::gtk::{
    Box, Button, CssProvider, Entry, Orientation, ScrolledWindow, ToggleButton, WrapMode, gdk, gio,
};
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
    app.connect_activate(|app| {
        if let Some(window) = app.active_window() {
            window.present();
            return;
        }

        build_ui(app);
    });
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

fn find_next(buffer: &Buffer, source_view: &View, query: &str) {
    if query.is_empty() {
        return;
    }

    let (start, end) = buffer.bounds();
    let text = buffer.text(&start, &end, false).to_string();
    if text.is_empty() {
        return;
    }

    let cursor = buffer.cursor_position().max(0) as usize;
    let start_byte = text
        .char_indices()
        .nth(cursor)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len());
    let match_byte = text[start_byte..]
        .find(query)
        .map(|idx| start_byte + idx)
        .or_else(|| text[..start_byte].find(query));

    if let Some(match_byte) = match_byte {
        let match_start = text[..match_byte].chars().count() as i32;
        let match_end = match_start + query.chars().count() as i32;
        let start_iter = buffer.iter_at_offset(match_start);
        let mut scroll_iter = start_iter;
        let end_iter = buffer.iter_at_offset(match_end);
        buffer.select_range(&start_iter, &end_iter);
        source_view.scroll_to_iter(&mut scroll_iter, 0.1, false, 0.0, 0.0);
    }
}

fn replace_current(buffer: &Buffer, source_view: &View, query: &str, replacement: &str) {
    if query.is_empty() {
        return;
    }

    if let Some((selection_start, selection_end)) = buffer.selection_bounds() {
        let selected = buffer.text(&selection_start, &selection_end, false);
        if selected.as_str() == query {
            let start_offset = selection_start.offset() as usize;
            let end_offset = selection_end.offset() as usize;
            let (buffer_start, buffer_end) = buffer.bounds();
            let text = buffer.text(&buffer_start, &buffer_end, false).to_string();
            let mut chars = text.chars().collect::<Vec<_>>();
            chars.splice(start_offset..end_offset, replacement.chars());
            let updated = chars.into_iter().collect::<String>();
            buffer.set_text(&updated);

            let replacement_end = start_offset + replacement.chars().count();
            let start_iter = buffer.iter_at_offset(start_offset as i32);
            let mut scroll_iter = start_iter;
            let end_iter = buffer.iter_at_offset(replacement_end as i32);
            buffer.select_range(&start_iter, &end_iter);
            source_view.scroll_to_iter(&mut scroll_iter, 0.1, false, 0.0, 0.0);
        }
    }

    find_next(buffer, source_view, query);
}

fn replace_all(buffer: &Buffer, query: &str, replacement: &str) {
    if query.is_empty() {
        return;
    }

    let (start, end) = buffer.bounds();
    let text = buffer.text(&start, &end, false).to_string();
    buffer.set_text(&text.replace(query, replacement));
}

fn build_ui(app: &Application) {
    let quit_action = gio::SimpleAction::new("quit", None);
    let app_clone = app.clone();
    quit_action.connect_activate(move |_, _| app_clone.quit());
    app.add_action(&quit_action);
    app.set_accels_for_action("app.quit", &["<Primary>q"]);

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

    let search_entry = Entry::builder()
        .placeholder_text("Search")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .hexpand(true)
        .build();
    let replace_entry = Entry::builder()
        .placeholder_text("Replace")
        .margin_top(6)
        .margin_bottom(6)
        .hexpand(true)
        .build();
    let find_button = Button::builder()
        .label("Find")
        .margin_top(6)
        .margin_bottom(6)
        .build();
    let replace_button = Button::builder()
        .label("Replace")
        .margin_top(6)
        .margin_bottom(6)
        .build();
    let replace_all_button = Button::builder()
        .label("Replace All")
        .margin_top(6)
        .margin_bottom(6)
        .margin_end(6)
        .build();

    let search_bar = Box::new(Orientation::Horizontal, 6);
    search_bar.append(&search_entry);
    search_bar.append(&replace_entry);
    search_bar.append(&find_button);
    search_bar.append(&replace_button);
    search_bar.append(&replace_all_button);
    search_bar.set_visible(false);

    {
        let buffer = buffer.clone();
        let source_view = source_view.clone();
        let search_entry = search_entry.clone();
        find_button.connect_clicked(move |_| {
            find_next(&buffer, &source_view, &search_entry.text());
        });
    }

    {
        let buffer = buffer.clone();
        let source_view = source_view.clone();
        let search_entry = search_entry.clone();
        search_entry.connect_activate(move |entry| {
            find_next(&buffer, &source_view, &entry.text());
        });
    }

    {
        let buffer = buffer.clone();
        let source_view = source_view.clone();
        let search_entry = search_entry.clone();
        let replace_entry = replace_entry.clone();
        replace_button.connect_clicked(move |_| {
            replace_current(
                &buffer,
                &source_view,
                &search_entry.text(),
                &replace_entry.text(),
            );
        });
    }

    {
        let buffer = buffer.clone();
        let search_entry = search_entry.clone();
        let replace_entry = replace_entry.clone();
        replace_all_button.connect_clicked(move |_| {
            replace_all(&buffer, &search_entry.text(), &replace_entry.text());
        });
    }

    let header = HeaderBar::new();
    header.set_title_widget(Some(&Box::new(Orientation::Horizontal, 0)));
    header.set_decoration_layout(Some(":close"));

    let json_button = make_button("JSON");
    let yaml_button = make_button("YAML");
    let bash_button = make_button("Bash");
    let go_button = make_button("Go");
    let javascript_button = make_button("JavaScript");
    let rust_button = make_button("Rust");
    let python_button = make_button("Python");

    let bottom_bar = Box::new(Orientation::Horizontal, 0);
    for btn in [
        &json_button,
        &yaml_button,
        &bash_button,
        &go_button,
        &javascript_button,
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
        javascript_button.clone(),
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
                if text.trim().is_empty() {
                    return;
                }
                let parsed = serde_json::from_str::<JsonValue>(&text)
                    .or_else(|_| serde_yaml::from_str::<JsonValue>(&text));
                if let Ok(v) = parsed
                    && let Ok(pretty) = serde_json::to_string_pretty(&v)
                {
                    buffer.set_text(&pretty);
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
                if text.trim().is_empty() {
                    return;
                }
                let parsed = serde_yaml::from_str::<YamlValue>(&text)
                    .or_else(|_| serde_json::from_str::<YamlValue>(&text));
                if let Ok(v) = parsed
                    && let Ok(pretty) = serde_yaml::to_string(&v)
                {
                    buffer.set_text(&pretty);
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
        &javascript_button,
        "js",
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
    toolbar_view.add_top_bar(&search_bar);
    toolbar_view.add_bottom_bar(&bottom_bar);
    toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_bottom_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_content(Some(&scrolled));

    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(728)
        .default_height(450)
        .build();

    let find_action = gio::SimpleAction::new("find", None);
    {
        let search_bar = search_bar.clone();
        let search_entry = search_entry.clone();
        find_action.connect_activate(move |_, _| {
            search_bar.set_visible(true);
            search_entry.grab_focus();
        });
    }
    window.add_action(&find_action);
    app.set_accels_for_action("win.find", &["<Primary>f"]);

    window.present();
}
