use std::{cell::RefCell, env, fs, rc::Rc};

use crate::{utils::widget_wrapper::WidgetWrapper, widgets::gallery::item::Item};
use gtk::{Grid, Label, prelude::*};
use std::path::Path;

pub struct Gallery {
    pub inner: WidgetWrapper<Grid>,
}

struct Theme {
    thumbnail: String,
    title: String,
}

fn get_themes() -> Vec<Theme> {
    let home = env::home_dir().expect("Could not find home directory");
    let themes_path = Path::new(".awesome-themes");
    let src_path = home.join(themes_path);
    let theme_dirs = fs::read_dir(&src_path).unwrap();
    let mut themes: Vec<Theme> = Vec::new();
    for dir in theme_dirs {
        if let Ok(entry) = dir {
            let theme_name = entry.file_name();
            let theme_path = src_path.join(&theme_name);

            let thumbnail_path = theme_path.join("wall.png");
            let theme = Theme {
                thumbnail: thumbnail_path.display().to_string(),
                title: theme_name.display().to_string(),
            };
            themes.push(theme);
        }
    }
    //return themes;
    return themes;
}

impl Gallery {
    pub fn new(theme_state: Rc<RefCell<String>>) -> Self {
        let mut items: Vec<_> = Vec::new();
        let themes = get_themes();
        for theme in themes {
            let title = Label::new(Some(theme.title.as_str()));
            let item = Item::new(theme.thumbnail.as_str(), theme.title, &theme_state).inner;
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
