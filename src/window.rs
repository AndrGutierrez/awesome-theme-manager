use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::ops::Deref;

pub struct MainWindow {
    pub inner: ApplicationWindow,
}

impl Deref for MainWindow {
    type Target = ApplicationWindow;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(860)
            .default_height(600)
            .title("Awesome Theme Manager")
            .build();

        // Don't forget to make all widgets visible.
        Self { inner: window }
    }
    pub fn show(&self) {
        self.inner.show();
    }
}
