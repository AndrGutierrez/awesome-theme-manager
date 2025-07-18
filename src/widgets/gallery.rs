use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{ApplicationWindow, Box, Image, Label, prelude::*};
use std::path::Path;

pub type Gallery = WidgetWrapper<gtk::Box>;

impl Gallery {
    pub fn new(window: &ApplicationWindow) -> Self {
        let image_path = "example.png";
        let bx = Box::new(gtk::Orientation::Vertical, 0);
        let title = Label::new(Some("void-heart"));

        // Verify file exists first
        if !Path::new(image_path).exists() {
            //eprintln!("Image file not found: {}", image_path);
            std::process::exit(1);
        }

        // Create image widget
        let image = Image::from_file(image_path);
        image.set_size_request(800, 400);
        bx.append(&image);
        bx.append(&title);

        Self { inner: bx }
    }
}
