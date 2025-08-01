use crate::{utils::widget_wrapper::WidgetWrapper, widgets::gallery::item::Item};
use gtk::{Grid, Label, prelude::*};

pub struct Gallery {
    pub inner: WidgetWrapper<Grid>,
}

struct Theme {
    thumbnail: String,
    title: String,
}
fn get_themes() -> Vec<Theme> {
    let theme = Theme {
        thumbnail: "example.png".to_string(),
        title: "void-heart".to_string(),
    };
    let themes = vec![theme];
    return themes;
}

impl Gallery {
    pub fn new() -> Self {
        let mut items: Vec<_> = Vec::new();
        let themes = get_themes();
        for theme in themes {
            let title = Label::new(Some(theme.title.as_str()));
            let item = Item::new(theme.thumbnail.as_str(), &title).inner;
            items.push(item);
        }

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
