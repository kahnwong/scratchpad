use adw::gtk::{Box, Orientation, ToggleButton};
use adw::prelude::*;
use serde_json::Value as JsonValue;
use sourceview5::prelude::*;
use sourceview5::{Buffer, LanguageManager};
use sqlformat::{FormatOptions, QueryParams, format as format_sql_query};
use std::cell::Cell;
use std::rc::Rc;

pub fn build_syntax_bar(buffer: &Buffer) -> Box {
    let json_button = make_button("JSON");
    let yaml_button = make_button("YAML");
    let toml_button = make_button("TOML");
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
        &toml_button,
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
        toml_button.clone(),
        bash_button.clone(),
        go_button.clone(),
        javascript_button.clone(),
        rust_button.clone(),
        python_button.clone(),
        sql_button.clone(),
    ]);

    connect_format_button(
        &json_button,
        "json",
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_format_button(
        &yaml_button,
        "yaml",
        buffer.clone(),
        all_buttons.clone(),
        guard.clone(),
    );
    connect_format_button(
        &toml_button,
        "toml",
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

fn connect_format_button(
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
            let (start, end) = buffer.bounds();
            let text = buffer.text(&start, &end, false);
            if let Some(converted) = convert_format(&text, lang_id) {
                buffer.set_text(&converted);
            }
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

fn convert_format(text: &str, lang_id: &str) -> Option<String> {
    if text.trim().is_empty() {
        return None;
    }

    // Try TOML before YAML: YAML accepts some TOML documents as plain strings.
    let value = serde_json::from_str::<JsonValue>(text)
        .or_else(|_| toml::from_str::<JsonValue>(text))
        .or_else(|_| serde_yaml::from_str::<JsonValue>(text))
        .ok()?;

    match lang_id {
        "json" => serde_json::to_string_pretty(&value).ok(),
        "yaml" => serde_yaml::to_string(&value).ok(),
        "toml" => toml::to_string_pretty(&value).ok(),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::convert_format;
    use serde_json::{Value, json};

    #[test]
    fn converts_between_all_formats() {
        let inputs = [
            r#"{"name":"café","enabled":true,"count":3,"ratio":1.5,"tags":["a","b"],"server":{"port":8080}}"#,
            "name: café\nenabled: true\ncount: 3\nratio: 1.5\ntags: [a, b]\nserver:\n  port: 8080\n",
            "name = \"café\"\nenabled = true\ncount = 3\nratio = 1.5\ntags = [\"a\", \"b\"]\n[server]\nport = 8080\n",
        ];
        let expected = json!({
            "name": "café", "enabled": true, "count": 3, "ratio": 1.5,
            "tags": ["a", "b"], "server": {"port": 8080}
        });

        for input in inputs {
            for target in ["json", "yaml", "toml"] {
                let output = convert_format(input, target).expect("conversion should succeed");
                let actual: Value = match target {
                    "json" => serde_json::from_str(&output).unwrap(),
                    "yaml" => serde_yaml::from_str(&output).unwrap(),
                    "toml" => toml::from_str(&output).unwrap(),
                    _ => unreachable!(),
                };
                assert_eq!(actual, expected, "input: {input}, target: {target}");
            }
        }
    }

    #[test]
    fn rejects_values_toml_cannot_represent() {
        for input in [r#"{"value":null}"#, "value: null", "[1, 2]", "42"] {
            assert!(convert_format(input, "toml").is_none(), "input: {input}");
        }
    }

    #[test]
    fn leaves_empty_and_invalid_input_unchanged() {
        for input in ["", " \n", "{\"broken\": ["] {
            for target in ["json", "yaml", "toml"] {
                assert!(convert_format(input, target).is_none());
            }
        }
    }
}
