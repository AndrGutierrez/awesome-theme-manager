use crate::{utils::widget_wrapper::WidgetWrapper, widgets::gallery::item::Item};
use gtk::{Box, Grid, prelude::*};

pub struct Gallery {
    pub inner: WidgetWrapper<Grid>,
}

impl Gallery {
    pub fn new() -> Self {
        let itm1 = Item::new().inner;
        let itm2 = Item::new().inner;
        let bx = Grid::builder().margin_start(0).build();
        bx.attach(itm1.as_ref(), 3, 0, 2, 1);
        bx.attach(itm2.as_ref(), 1, 0, 2, 1);
        Self {
            inner: WidgetWrapper { inner: bx },
        }
    }
}
