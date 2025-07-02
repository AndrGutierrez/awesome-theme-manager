mod utils;
mod widgets;
mod window;
use gtk::Application;
use gtk::prelude::*;
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
        win.set_titlebar(Some(header.as_ref()));
        win.set_child(Some(apply_button.as_ref()));
        win.show();
    });

    app.run();
}
