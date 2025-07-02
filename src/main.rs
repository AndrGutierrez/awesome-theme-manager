mod utils;
mod widgets;
mod window;
use gtk::prelude::*;
use gtk::{Application, Box};
use widgets::apply_button::ApplyButton;
use widgets::header::Header;
use window::MainWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let win = MainWindow::new(app);
        let header = Header::new();
        let apply_button = ApplyButton::new();
        let main_box = Box::new(gtk::Orientation::Vertical, 50);
        main_box.append(apply_button.as_ref());

        apply_button.set_halign(gtk::Align::End); // Or Start/End based on your needs
        win.set_titlebar(Some(header.as_ref()));
        win.set_child(Some(&main_box));
        win.show();
    });

    app.run();
}
