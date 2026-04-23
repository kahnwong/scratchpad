use adw::gtk::{gdk, Box, CssProvider, Orientation, ScrolledWindow, TextView, WrapMode};
use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};

const APP_ID: &str = "io.github.kahnwong.Scratchpad";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let css = CssProvider::new();
    css.load_from_string("headerbar, textview, textview text { background-color: #1d1d20; } textview { font-size: 12pt; }");
    adw::gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css,
        adw::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let text_view = TextView::builder()
        .wrap_mode(WrapMode::Word)
        .monospace(true)
        .left_margin(16)
        .right_margin(16)
        .top_margin(16)
        .bottom_margin(16)
        .build();

    let scrolled = ScrolledWindow::builder()
        .child(&text_view)
        .vexpand(true)
        .hexpand(true)
        .build();

    let header = HeaderBar::new();
    header.set_title_widget(Some(&Box::new(Orientation::Horizontal, 0)));
    header.set_decoration_layout(Some(":close"));

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
    toolbar_view.set_content(Some(&scrolled));

    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(640)
        .default_height(480)
        .build();

    window.present();
}
