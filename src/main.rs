use crate::screen_capture::ScreenCapture;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;

fn main() {
    println!("use as test");

    // 测试
    let area = ScreenCapture::request_select();
    println!("area: {:?}", area);

    // 获取屏幕
    // let screens = ScreenshotsImpl::get_screens();
    // println!("screens: {:#?}", screens);

    // 获取屏幕
    // let xywh = app::screen_xywh(0);
    // let scale = app::screen_scale(0);
    // println!("xywh: {xywh:?}; scale: {scale}");

    // 获取屏幕
    // let screens = FltkImpl::get_screens();
    // println!("screens: {:#?}", screens);
}

#[cfg(test)]
mod unit_test {
    use fltk::app;
    use fltk::prelude::*;
    use fltk::window::Window;

    #[test]
    fn sf() {
        let app = app::App::default();
        let mut wind = Window::new(0, 0, 1400, 800, "Hello from rust");
        wind.end();
        wind.show();
        app.run().unwrap();
    }
}