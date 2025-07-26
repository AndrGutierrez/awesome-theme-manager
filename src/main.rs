mod utils;
mod widgets;

use gtk::{Application, ApplicationWindow, Box, CssProvider};
use gtk::{Label, prelude::*};
use widgets::footer::Footer;
use widgets::header::Header;

use crate::widgets::gallery::gallery::Gallery;
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("src/assets/style.css");

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::new(app);
        let header = Header::new();
        let main_box = Box::new(gtk::Orientation::Vertical, 50);
        let action_bar = Footer::new();
        let label = Label::builder().label("Lorem ipsum").build();
        let gallerybox = Gallery::new();
        main_box.append(&label);
        main_box.append(gallerybox.inner.as_ref());
        main_box.append(action_bar.as_ref());

        win.set_default_size(1080, 700);
        win.set_titlebar(Some(header.as_ref()));
        win.set_child(Some(&main_box));

        win.show();
    });
    app.connect_startup(|_| load_css());
    app.run();
}
