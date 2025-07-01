mod widgets;
mod window;
use gtk::prelude::*;
use gtk::Application;
use widgets::header::Header;
use window::MainWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let win = MainWindow::new(app);
        let header = Header::new();
        win.set_titlebar(Some(header.as_ref()));
        win.show();
    });

    app.run();
}
