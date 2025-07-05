use crate::utils::widget_wrapper::WidgetWrapper;
use gtk::{
    Button,
    prelude::{ButtonExt, WidgetExt},
};
use std::process::Command;
use std::thread;

pub type ApplyButton = WidgetWrapper<gtk::Button>;

use fs_extra::dir::{CopyOptions, copy};
use std::fs;
use std::path::Path;

fn sync_theme(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the full source path: ".awesome-themes/{source}"
    let src_path = Path::new(".awesome-themes").join(source);
    println!("{:?}", src_path.to_str());

    // Verify source exists and is a directory
    if !src_path.exists() || !src_path.is_dir() {
        //return Err(format!("Theme '{}' not found in .awesome-themes/", source).into());
    }

    let dest_path = Path::new(".config/awesome");

    // Clear destination directory by removing and recreating it
    if dest_path.exists() {
        fs::remove_dir_all(dest_path)?;
    }
    fs::create_dir_all(dest_path)?;

    // Copy all contents from source to destination
    let mut options = CopyOptions::new();
    options.content_only = true; // Don't copy the directory itself, just its contents
    options.overwrite = true;

    copy(&src_path, dest_path, &options)?;

    Ok(())
}

fn change_theme(theme_name: &str) {
    sync_theme(theme_name);
    //Command::new("awesome-client")
    //    .arg("awesome.restart()")
    //    .status()
    //    .expect("Failed to restart Awesome WM");
}
impl ApplyButton {
    pub fn new() -> Self {
        let button = Button::builder().label("Apply").build();
        button.connect_clicked(move |button| {
            thread::spawn(move || {
                change_theme("multicolor");
            })
            .join()
            .unwrap();
        });
        button.set_size_request(100, 40);
        button.add_css_class("suggested-action");
        Self { inner: button }
    }
}
