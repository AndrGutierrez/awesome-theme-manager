use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{Box, Label, Picture, prelude::*};
use std::path::Path;

pub struct Item {
    pub inner: WidgetWrapper<Box>,
}

impl Item {
    // TODO add support for more file formats
    pub fn new(image_path: &str, title: &Label) -> Self {
        let bx = Box::new(gtk::Orientation::Vertical, 20);
        bx.add_css_class("gallery-item");
        if !Path::new(image_path).exists() {
            std::process::exit(1);
        }

        let image = Picture::for_filename(image_path);
        bx.append(&image);
        bx.append(title);

        let click_controller = gtk::GestureClick::new();
        click_controller.connect_pressed(|controller, _, _, _| {
            if let Some(widget) = controller.widget() {
                if widget.has_css_class("selected") {
                    widget.remove_css_class("selected");
                } else {
                    widget.add_css_class("selected");
                }
            }
        });
        bx.add_controller(click_controller);
        return Self {
            inner: WidgetWrapper { inner: bx },
        };
    }
}
