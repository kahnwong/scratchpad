use adw::gtk::{Box, Orientation, ToggleButton};
use adw::prelude::*;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use sourceview5::prelude::*;
use sourceview5::{Buffer, LanguageManager};
use sqlformat::{FormatOptions, QueryParams, format as format_sql_query};
use std::cell::Cell;
use std::rc::Rc;

pub fn build_syntax_bar(buffer: &Buffer) -> Box {
    let json_button = make_button("JSON");
    let yaml_button = make_button("YAML");
    let bash_button = make_button("Bash");
    let go_button = make_button("Go");
    let javascript_button = make_button("JavaScript");
    let rust_button = make_button("Rust");
    let python_button = make_button("Python");
    let sql_button = make_button("SQL");

    let bottom_bar = Box::new(Orientation::Horizontal, 0);
    for btn in [
        &json_button,
        &yaml_button,
        &bash_button,
        &go_button,
        &javascript_button,
        &rust_button,
        &python_button,
        &sql_button,
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
        sql_button.clone(),
    ]);

    connect_json_button(
        &json_button,
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_yaml_button(
        &yaml_button,
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
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
        all_buttons.clone(),
        guard.clone(),
    );
    connect_sql_button(&sql_button, buffer.clone(), all_buttons, guard);

    bottom_bar
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
            set_language(&buffer, lang_id);
        } else {
            buffer.set_language(None);
        }
    });
}

fn connect_sql_button(
    button: &ToggleButton,
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
            set_language(&buffer, "sql");
            format_sql(&buffer);
        } else {
            buffer.set_language(None);
        }
    });
}

fn connect_json_button(
    button: &ToggleButton,
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
            set_language(&buffer, "json");
            format_json(&buffer);
        } else {
            buffer.set_language(None);
        }
    });
}

fn connect_yaml_button(
    button: &ToggleButton,
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
            set_language(&buffer, "yaml");
            format_yaml(&buffer);
        } else {
            buffer.set_language(None);
        }
    });
}

fn set_language(buffer: &Buffer, lang_id: &str) {
    let lang_manager = LanguageManager::default();
    if let Some(lang) = lang_manager.language(lang_id) {
        buffer.set_language(Some(&lang));
    }
}

fn format_json(buffer: &Buffer) {
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
}

fn format_sql(buffer: &Buffer) {
    let (start, end) = buffer.bounds();
    let text = buffer.text(&start, &end, false);
    if text.trim().is_empty() {
        return;
    }

    buffer.set_text(&format_sql_query(
        &text,
        &QueryParams::None,
        &FormatOptions::default(),
    ));
}

fn format_yaml(buffer: &Buffer) {
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
}
