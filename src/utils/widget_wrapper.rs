use std::ops::{Deref, DerefMut};

pub struct WidgetWrapper<T: AsRef<gtk::Widget>> {
    pub inner: T,
}

impl<T: AsRef<gtk::Widget>> Deref for WidgetWrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: AsRef<gtk::Widget>> DerefMut for WidgetWrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: AsRef<gtk::Widget>> AsRef<gtk::Widget> for WidgetWrapper<T> {
    fn as_ref(&self) -> &gtk::Widget {
        self.inner.as_ref()
    }
}

