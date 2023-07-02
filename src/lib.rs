#[macro_use]
extern crate napi_derive;

use crate::declares::SelectedImage;
use crate::screen_capture::ScreenCapture;

mod declares;
mod utils;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}

#[napi]
pub fn ttt() -> napi::Result<Option<SelectedImage>> {
    Ok(ScreenCapture::capture_with_crop())
}

#[cfg(test)]
mod unit_test {
    use crate::screen_capture::ScreenCapture;

    #[test]
    fn tt() {
        match ScreenCapture::capture_with_crop() {
            Some(v) => {
                println!("done");
            }
            None => {
                println!("fail");
            }
        }
    }
}