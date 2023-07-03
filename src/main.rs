#![windows_subsystem = "windows"]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::fs::File;
use std::io::Write;
use crate::declares::SelectedImage;
use crate::screen_capture::ScreenCapture;

mod declares;
mod utils;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

fn main() {
    match ScreenCapture::capture_with_crop() {
        Some(SelectedImage { p1p2, buffer }) => {
            let w = (p1p2[2] - p1p2[0]) as u32;
            let h = (p1p2[3] - p1p2[1]) as u32;

            let image_name = format!("capture_{x}_{y}_{w}x{h}.png", x = p1p2[0], y = p1p2[1]);
            fs::write(image_name, &buffer).unwrap();

            println!("done with select!");
        }
        None => {
            println!("done without select!");
        }
    }
}

#[cfg(test)]
mod unit_test {}