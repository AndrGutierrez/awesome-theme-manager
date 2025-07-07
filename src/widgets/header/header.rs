use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{HeaderBar, Label};

pub type Header = WidgetWrapper<gtk::HeaderBar>;
impl Header {
    pub fn new() -> Self {
        let header = HeaderBar::builder().build();
        let label = Label::builder().label("Awesome Theme Manager").build();
        header.set_title_widget(Some(&label));
        Self { inner: header }
    }
}

