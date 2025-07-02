pub struct WidgetWrapper<T> {
    pub inner: T,
}

impl<T> std::ops::Deref for WidgetWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for WidgetWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<T> for WidgetWrapper<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
