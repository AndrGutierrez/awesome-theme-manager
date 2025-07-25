mod utils;
mod widgets;
use gtk::{Application, ApplicationWindow, Box};
use gtk::{Label, prelude::*};
use widgets::footer::Footer;
use widgets::header::Header;

use crate::widgets::gallery::gallery::Gallery;

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

    app.run();
}
