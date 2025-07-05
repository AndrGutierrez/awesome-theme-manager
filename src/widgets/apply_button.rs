use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{
    Button,
    prelude::{ButtonExt, WidgetExt},
};
use std::thread;
use std::{process::Command, time::Duration};

pub type ApplyButton = WidgetWrapper<gtk::Button>;

impl ApplyButton {
    pub fn new() -> Self {
        let button = Button::builder().label("Apply").build();
        button.connect_clicked(move |button| {
            thread::spawn(move || {
                Command::new("awesome-client")
                    .arg("awesome.restart()")
                    .status()
                    .expect("Failed to restart Awesome WM");
            })
            .join()
            .unwrap();
        });
        button.set_size_request(100, 40);
        button.add_css_class("suggested-action");
        Self { inner: button }
    }
}
