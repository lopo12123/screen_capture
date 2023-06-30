#[macro_use]
extern crate napi_derive;

use crate::screen_capture::ScreenCapture;

mod declares;
mod screenshots_impl;
mod egui_impl;
mod screen_capture;

fn main() {
    // for screen in ScreenCapture::capture() {
    //     let crate::declares::CaptureInfo {
    //         scale_factor, physical_x, physical_y, physical_width, physical_height, ..
    //     } = screen;
    //     println!("xywh: {}, {}, {}, {}; sf: {}", physical_x, physical_y, physical_width, physical_height, scale_factor);
    // }

    ScreenCapture::capture_with_crop();
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn point_test() {}
}