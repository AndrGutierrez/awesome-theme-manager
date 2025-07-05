use crate::utils::widget_wrapper::WidgetWrapper;
use crate::widgets::apply_button::ApplyButton;
use gtk::{ActionBar, prelude::WidgetExt};

pub type Footer = WidgetWrapper<gtk::ActionBar>;
impl Footer {
    pub fn new() -> Self {
        let footer = ActionBar::new();
        footer.set_height_request(50);
        //let label = Label::builder().label("Awesome Theme Manager").build();
        //header.set_title_widget(Some(&label));
        let apply_button = ApplyButton::new();
        // apply_button.set_halign(gtk::Align::End); // Or Start/End based on your needs
        footer.pack_end(apply_button.as_ref());
        Self { inner: footer }
    }
}
