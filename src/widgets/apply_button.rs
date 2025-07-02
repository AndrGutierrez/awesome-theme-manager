use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::Button;

pub type ApplyButton = WidgetWrapper<gtk::Button>;

impl ApplyButton {
    pub fn new() -> Self {
        let button = Button::with_label("Apply");
        Self { inner: button }
    }
}
