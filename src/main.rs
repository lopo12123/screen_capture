#[macro_use]
extern crate napi_derive;

use crate::screen_capture::ScreenCapture;

mod declares;
mod utils;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

fn main() {
    ScreenCapture::capture_with_crop();
}

#[cfg(test)]
mod unit_test {}