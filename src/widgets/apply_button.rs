use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{Button, prelude::WidgetExt};

pub type ApplyButton = WidgetWrapper<gtk::Button>;

impl ApplyButton {
    pub fn new() -> Self {
        let button = Button::with_label("Apply");
        button.set_size_request(1, 50);
        Self { inner: button }
    }
}
