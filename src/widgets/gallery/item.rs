use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{Box, Image, Label, Picture, prelude::*};
use std::path::Path;

pub struct Item {
    pub inner: WidgetWrapper<Box>,
}

impl Item {
    pub fn new() -> Self {
        let image_path = "example.png";
        let bx = Box::new(gtk::Orientation::Vertical, 0);
        let title = Label::new(Some("void-heart"));
        // Verify file exists first
        if !Path::new(image_path).exists() {
            //eprintln!("Image file not found: {}", image_path);
            std::process::exit(1);
        }

        // Create image widget
        let image = Picture::for_filename(image_path);
        image.set_can_shrink(true); // Allow shrinking if needed
        image.set_size_request(250, -1);
        bx.append(&image);
        bx.append(&title);

        return Self {
            inner: WidgetWrapper { inner: bx },
        };
        //Self { inner: bx };
    }
}

