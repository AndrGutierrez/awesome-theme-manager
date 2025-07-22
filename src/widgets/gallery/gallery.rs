use crate::{utils::widget_wrapper::WidgetWrapper, widgets::gallery::item::Item};
use gtk::{Box, Grid, prelude::*};

pub struct Gallery {
    pub inner: WidgetWrapper<Grid>,
}

impl Gallery {
    pub fn new() -> Self {
        let itm1 = Item::new().inner;
        let itm2 = Item::new().inner;
        let itm3 = Item::new().inner;
        let itm4 = Item::new().inner;
        let bx = Grid::builder()
            .margin_start(0)
            .column_spacing(10)
            .row_spacing(10)
            .build();
        bx.attach(itm2.as_ref(), 1, 0, 1, 1);
        bx.attach(itm1.as_ref(), 2, 0, 1, 1);
        bx.attach(itm3.as_ref(), 3, 0, 1, 1);
        bx.attach(itm4.as_ref(), 4, 0, 1, 1);
        //bx.attach(child, column, row, width, height);
        Self {
            inner: WidgetWrapper { inner: bx },
        }
    }
}

