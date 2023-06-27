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

    #[test]
    fn ss() {
        let xy = get_real_wh_before_scale(1.5, (424, 960));
        println!("xy: {xy:?}");
        let screen1 = ScreenshotsImpl::get_by_point(xy.0 + 2, xy.1 + 2).map_or(1, |v| v.id);
        println!("{screen1}");
    }
}