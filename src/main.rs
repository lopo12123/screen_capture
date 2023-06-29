#[macro_use]
extern crate napi_derive;

use std::env::{args};
use std::fs::File;
use std::io::Write;

use crate::screen_capture::ScreenCapture;
use crate::imgui_impl::ImguiImpl;

mod declares;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

/// # Examples
///
/// ``` shell
/// // Running without parameters will save the image as 'capture.png' by default
/// screen_capture.exe
/// // Running with --out=[file.suffix] will save the image as '[file.suffix]'
/// screen_capture.exe --out=my-capture.png
/// ```
fn main() {
    ScreenCapture::capture_with_crop();

    // for screen in ScreenCapture::capture() {
    //     println!("{}", screen.scale_factor);
    //     println!("{} x {}", screen.physical_x, screen.physical_y);
    //     println!("{} x {}", screen.physical_width, screen.physical_height);
    // }

    // let mut filename = "capture.png".to_string();
    // for arg in args() {
    //     if let Some(v) = arg.strip_prefix("--out=") {
    //         filename = v.to_string();
    //     }
    // }
    //
    // println!("filename: {:?}", filename);
    //
    // // 交互式框选并获取目标区域 buffer
    // match ScreenCapture::request_capture(None) {
    //     Some(v) => match File::create(&filename) {
    //         Ok(mut file) => match file.write_all(&v.buffer) {
    //             Ok(_) => println!("The captured image has been saved as '{filename}'"),
    //             Err(_) => println!("Fail to write file '{filename}'"),
    //         },
    //         Err(_) => {
    //             println!("Fail to create file '{filename}'");
    //         }
    //     }
    //     None => {
    //         println!("Cancel");
    //     }
    // }
}

#[cfg(test)]
mod unit_test {
    use std::cell::RefCell;
    use std::env::args;
    use std::rc::Rc;
    use std::thread::sleep;
    use std::time::Duration;
    use fltk::app;
    use fltk::app::{quit, redraw};
    use fltk::button::Button;
    use fltk::enums::{Color, Event, FrameType};
    use fltk::image::SvgImage;
    use fltk::prelude::*;
    use fltk::surface::ImageSurface;
    use fltk::window::Window;
    use crate::screenshots_impl::ScreenshotsImpl;

    #[test]
    fn mini() {
        let app = app::App::default();
        let mut wind = Window::new(640, 360, 1280, 720, "Hello from rust");

        wind.end();
        wind.show();
        app.run().unwrap();
    }

    #[test]
    fn point_test() {
        // 获取屏幕信息列表
        // for count in 0..fltk::app::screen_count() {
        //     println!("{count}: {:?}", fltk::app::screen_xywh(count));
        // }

        // All points are on the screen with screen_num=1
        let points = vec![
            (0, 0),
            (0, 1439),
            (2559, 0),
            (2559, 1439),
            (1450, 933),
            (1357, 1099),  // x
            (1357, 331),
            (1323, 1241),  // x
            (23, 234),
            (23, 1099),
        ];

        for point in points {
            println!("point: {point:?} => screen_num: {}", fltk::app::screen_num(point.0, point.1));
        }
    }
}