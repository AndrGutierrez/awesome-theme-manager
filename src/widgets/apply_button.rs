use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{
    Button,
    prelude::{ButtonExt, WidgetExt},
};
use std::thread;

use std::process::Command;
pub type ApplyButton = WidgetWrapper<gtk::Button>;

use crate::widgets::confirmation_dialog::ConfirmationDialog;
use fs_extra::dir::{CopyOptions, copy};
use gtk::{Window, prelude::*};
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
        Err(err) => {}
    }
    Command::new("awesome-client")
        .arg("awesome.restart()")
        .status()
        .expect("Failed to restart Awesome WM");
}
impl ApplyButton {
    pub fn new(window: &impl IsA<Window>) -> Self {
        let button = Button::builder().label("Apply").build();

        let dialog = ConfirmationDialog::new(
            window,
            "Confirm Theme Change",
            "Are you sure you want to apply the 'void-heart' theme?\nThis will restart AwesomeWM.",
        );
        button.connect_clicked(move |_| {
            thread::spawn(move || {
                change_theme("void-heart");
            })
            .join()
            .unwrap();

            // `await` the user's response without blocking the UI.
            let response = dialog.run();

            // Check if the user clicked the "OK" button.
            //if response == gtk::ResponseType::Ok {
            //    // The `change_theme` function performs blocking file I/O and runs an external command.
            //    // It MUST be run on a separate thread to avoid freezing the user interface.
            //    thread::spawn(move || {
            //        change_theme("void-heart");
            //    });
            //} else {
            //    //println!("User cancelled the theme change.");
            //}
        });
        button.set_size_request(100, 40);
        button.add_css_class("suggested-action");
        Self { inner: button }
    }
}
