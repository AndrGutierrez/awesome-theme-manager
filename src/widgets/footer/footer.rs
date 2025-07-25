use crate::utils::widget_wrapper::WidgetWrapper;
use crate::widgets::apply_button::ApplyButton;
use gtk::{ActionBar, prelude::*};

pub type Footer = WidgetWrapper<gtk::ActionBar>;

impl Footer {
    pub fn new() -> Self {
        let footer = ActionBar::new();
        footer.set_height_request(50);

        let apply_button = ApplyButton::new();

        footer.pack_end(apply_button.as_ref());

        Self { inner: footer }
    }
}
