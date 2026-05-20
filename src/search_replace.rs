use adw::gtk::{Box, Button, Entry, Orientation};
use adw::prelude::*;
use sourceview5::{Buffer, View};

pub struct SearchReplaceBar {
    pub widget: Box,
    search_entry: Entry,
}

impl SearchReplaceBar {
    pub fn new(buffer: &Buffer, source_view: &View) -> Self {
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

        let widget = Box::new(Orientation::Horizontal, 6);
        widget.append(&search_entry);
        widget.append(&replace_entry);
        widget.append(&find_button);
        widget.append(&replace_button);
        widget.append(&replace_all_button);
        widget.set_visible(false);

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

        Self {
            widget,
            search_entry,
        }
    }

    pub fn show_and_focus(&self) {
        self.widget.set_visible(true);
        self.search_entry.grab_focus();
    }
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
