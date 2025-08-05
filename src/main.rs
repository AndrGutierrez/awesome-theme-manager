mod utils;
mod widgets;

use gtk::gdk::Display;
use gtk::{Application, ApplicationWindow, Box, CssProvider};
use gtk::{Label, prelude::*};
use widgets::footer::Footer;
use widgets::header::Header;

use crate::widgets::gallery::gallery::Gallery;

use std::cell::RefCell;
use std::rc::Rc;
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("src/assets/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    // Create the shared state
    let selected_theme = Rc::new(RefCell::new(String::new()));

    // Clone the Rc for the activate closure
    let selected_theme_clone = Rc::clone(&selected_theme);

    app.connect_activate(move |app| {
        let win = ApplicationWindow::new(app);
        let header = Header::new();
        let main_box = Box::new(gtk::Orientation::Vertical, 50);

        // Create components with cloned Rcs
        let action_bar = Footer::new(Rc::clone(&selected_theme_clone));
        let label = Label::builder().label("Lorem ipsum").build();
        let gallerybox = Gallery::new(Rc::clone(&selected_theme_clone));

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
