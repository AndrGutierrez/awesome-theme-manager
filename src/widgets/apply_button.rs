use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{
    Button, MessageDialog, ResponseType,
    prelude::{ButtonExt, WidgetExt},
};
use std::thread;

use std::process::Command;
pub type ApplyButton = WidgetWrapper<gtk::Button>;

use fs_extra::dir::{CopyOptions, copy};
use gtk::prelude::*;
use std::env;
use std::fs;
use std::path::Path;

fn sync_theme(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = env::home_dir().expect("Could not find home directory");
    let mut src_path = Path::new(".awesome-themes").join(source);
    src_path = home.join(src_path);

    if !src_path.exists() || !src_path.is_dir() {
        //return Err(format!("Theme '{}' not found in .awesome-themes/", source).into());
    }

    let mut dest_path = Path::new(".config/awesome").join("");
    dest_path = home.join(dest_path);

    if dest_path.exists() {
        fs::remove_dir_all(&dest_path)?;
    }
    fs::create_dir_all(&dest_path)?;

    let mut options = CopyOptions::new();
    options.content_only = true;
    options.overwrite = true;

    copy(&src_path, &dest_path, &options)?;

    Ok(())
}

fn change_theme(theme_name: &str) {
    match sync_theme(theme_name) {
        Ok(f) => f,
        Err(_) => todo!(),
    }
    Command::new("awesome-client")
        .arg("awesome.restart()")
        .status()
        .expect("Failed to restart Awesome WM");
}
impl ApplyButton {
    pub fn new() -> Self {
        let button = Button::builder().label("Apply").build();

        button.connect_clicked(move |_| {
            thread::spawn(move || {
                change_theme("void-heart");
            })
            .join()
            .unwrap();
            let dialog = MessageDialog::builder()
                //.transient_for(window)
                .modal(true)
                .buttons(gtk::ButtonsType::OkCancel) // This gives us OK and Cancel buttons
                .text("Confirm Theme Change")
                .secondary_text("Are you sure you want to apply this theme?")
                .build();

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Ok {
                    // User clicked OK/Apply
                }
                dialog.close();
            });

            dialog.show();
        });

        button.set_size_request(100, 40);
        button.add_css_class("suggested-action");
        Self { inner: button }
    }
}
