use crate::fltk_impl::FltkImpl;
use crate::screen_capture::ScreenCapture;
use crate::screenshots_impl::ScreenshotsImpl;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

fn main() {
    println!("use as test");

    // 交互式框选
    let area = ScreenCapture::request_select(None);
    println!("area: {:?}", area);
}

#[cfg(test)]
mod unit_test {
    use std::thread::sleep;
    use std::time::Duration;
    use fltk::app;
    use fltk::prelude::*;
    use fltk::window::Window;

    #[test]
    fn sf() {
        let app = app::App::default();
        let mut wind = Window::new(640, 360, 1280, 720, "Hello from rust");
        wind.end();
        wind.show();
        app.run().unwrap();
    }

    #[test]
    fn sf_real() {
        sleep(Duration::from_secs(2));

        let sc = app::get_mouse();
        println!("sc: {sc:?}");

        // let num = app::Screen::num_at(sc);
        // println!("num: {}", num.map_or("none".to_string(), |v| v.to_string()));

        // match app::focus() {
        //     Some(win) => {
        //         println!("{}, {}", win.x(), win.y());
        //     },
        //     None => {
        //         println!("none");
        //     }
        // }
    }
}