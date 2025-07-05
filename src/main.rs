mod utils;
mod widgets;
mod window;
use gtk::{Application, Box};
use gtk::{Label, prelude::*};
use widgets::footer::Footer;
use widgets::header::Header;
use window::MainWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let win = MainWindow::new(app);
        let header = Header::new();
        let main_box = Box::new(gtk::Orientation::Vertical, 50);
        let action_bar = Footer::new();
        let label = Label::builder().label("Lorem ipsum").build();
        main_box.append(&label);
        main_box.append(action_bar.as_ref());
        //main_box.append(apply_button.as_ref());
        //main_box.append(action_bar.as_ref());

        win.set_titlebar(Some(header.as_ref()));
        win.set_child(Some(&main_box));
        win.show();
    });

    app.run();
}
