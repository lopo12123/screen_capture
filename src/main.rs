use crate::screen_capture::ScreenCapture;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

fn main() {
    println!("use as test");

    // 交互式框选
    let area = ScreenCapture::request_bounding(None);
    println!("area: {:?}", area);

    // let area = ScreenCapture::request_capture(None);
    // println!("area: {:?}", area.is_some());
}

#[cfg(test)]
mod unit_test {
    use std::thread::sleep;
    use std::time::Duration;
    use fltk::app;
    use fltk::prelude::*;
    use fltk::window::Window;
    use crate::screenshots_impl::ScreenshotsImpl;
    use crate::utils::get_real_wh_before_scale;

    #[test]
    fn sf() {
        let app = app::App::default();
        let mut wind = Window::new(640, 360, 1280, 720, "Hello from rust");
        // wind.set_border(false);
        wind.end();
        wind.show();
        app.run().unwrap();
    }
}