use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{Box, Label, Picture, prelude::*};
use std::{cell::RefCell, path::Path, rc::Rc};

pub struct Item {
    pub inner: WidgetWrapper<Box>,
}

impl Item {
    // TODO support for more image formats
    pub fn new(image_path: &str, title: String, theme_state: &Rc<RefCell<String>>) -> Self {
        let title_label = Label::new(Some(title.clone().as_str()));
        let bx = Box::new(gtk::Orientation::Vertical, 20);
        bx.add_css_class("gallery-item");

        if !Path::new(image_path).exists() {
            std::process::exit(1);
        }

        let image = Picture::for_filename(image_path);
        bx.append(&image);
        bx.append(&title_label);

        let theme_state_clone = Rc::clone(&theme_state);

        let click_controller = gtk::GestureClick::new();
        click_controller.connect_pressed(move |controller, _, _, _| {
            *theme_state_clone.borrow_mut() = title.clone();

            if let Some(widget) = controller.widget() {
                if widget.has_css_class("selected") {
                    widget.remove_css_class("selected");
                } else {
                    widget.add_css_class("selected");
                }
            }
        });

        bx.add_controller(click_controller);

        Self {
            inner: WidgetWrapper { inner: bx },
        }
    }
}
