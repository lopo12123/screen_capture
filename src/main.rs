use crate::screen_capture::ScreenCapture;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

fn main() {
    let area = ScreenCapture::request_select();
    println!("area: {:?}", area);
}
