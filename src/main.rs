use crate::screen_capture::ScreenCapture;
use crate::screenshots_impl::ScreenshotsImpl;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

fn main() {
    println!("use as test");


    let area = ScreenCapture::request_select();
    println!("area: {:?}", area);

//     let screens = ScreenshotsImpl::get_screens();
//     println!("screens: {:#?}", screens);
//
//     let screens = ScreenCapture::get_screens();
//     println!("screens: {:#?}", screens);
}