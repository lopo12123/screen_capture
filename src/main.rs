#[macro_use]
extern crate napi_derive;

use crate::screen_capture::ScreenCapture;

mod declares;
mod utils;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

fn main() {
    match ScreenCapture::capture_with_crop() {
        Some(v) => {
            println!("done");
        }
        None => {
            println!("fail");
        }
    }

    // ScreenCapture::capture_with_crop();

    // for screen in ScreenCapture::capture() {
    //     println!("rgba: {}, size: {}, w: {}, h: {}", screen.rgba.len(), screen.buffer.len(), screen.physical_width, screen.physical_height);
    // }
}

#[cfg(test)]
mod unit_test {}