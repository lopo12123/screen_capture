#[macro_use]
extern crate napi_derive;

use crate::screen_capture::ScreenCapture;

mod declares;
mod screenshots_impl;
mod egui_impl;
mod screen_capture;

fn main() {
    ScreenCapture::capture_with_crop();
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn point_test() {}
}