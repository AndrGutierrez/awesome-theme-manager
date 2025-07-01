use gtk::{HeaderBar, Label};
use std::ops::Deref;

pub struct Header {
    pub inner: HeaderBar,
}

impl Deref for Header {
    type Target = HeaderBar;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Header {
    pub fn new() -> Self {
        let header = HeaderBar::builder().build();
        let label = Label::builder().label("Awesome Theme Manager").build();
        header.set_title_widget(Some(&label));
        Self { inner: header }
    }
}

impl AsRef<HeaderBar> for Header {
    fn as_ref(&self) -> &HeaderBar {
        &self.inner
    }
}
