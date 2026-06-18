mod search_replace;
mod syntax_tools;

use adw::gtk::{Box, CssProvider, Orientation, ScrolledWindow, WrapMode, gdk, gio};
use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};
use search_replace::SearchReplaceBar;
use sourceview5::prelude::*;
use sourceview5::{Buffer, StyleSchemeManager, View};
use syntax_tools::build_syntax_bar;

const APP_ID: &str = "me.karnwong.Scratchpad";

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

    let search_bar = SearchReplaceBar::new(&buffer, &source_view);

    let header = HeaderBar::new();
    header.set_title_widget(Some(&Box::new(Orientation::Horizontal, 0)));
    header.set_decoration_layout(Some(":close"));

    let bottom_bar = build_syntax_bar(&buffer);

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.add_top_bar(&search_bar.widget);
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
        let search_bar = search_bar;
        find_action.connect_activate(move |_, _| {
            search_bar.show_and_focus();
        });
    }
    window.add_action(&find_action);
    app.set_accels_for_action("win.find", &["<Primary>f"]);

    window.present();
}
