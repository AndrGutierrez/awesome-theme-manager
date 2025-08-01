use crate::{utils::widget_wrapper::WidgetWrapper, widgets::gallery::item::Item};
use gtk::{Grid, prelude::*};

pub struct Gallery {
    pub inner: WidgetWrapper<Grid>,
}

impl Gallery {
    pub fn new() -> Self {
        let itm1 = Item::new().inner;
        let itm2 = Item::new().inner;
        let itm3 = Item::new().inner;
        let itm4 = Item::new().inner;
        let items = [itm1, itm2, itm3, itm4];
        let bx = Grid::builder()
            .margin_start(0)
            .column_spacing(10)
            .row_spacing(10)
            .build();

        let mut column_count = 0;
        let mut row_count = 0;
        let max_columns = 3;
        for itm in items {
            if (column_count % max_columns) == 0 {
                row_count += 1;
                column_count = 0;
            }
            bx.attach(itm.as_ref(), column_count, row_count, 1, 1);
            column_count += 1;
        }
        Self {
            inner: WidgetWrapper { inner: bx },
        }
    }
}
