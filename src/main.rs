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
    use std::cell::RefCell;
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
    use crate::utils::get_real_wh_before_scale;

    const SVG_CANCEL: &str = r##"<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" stroke="#000000" stroke-width="1.5" stroke-linecap="round"><path d="M5.26904 5.39746L18.4684 18.5968"/><path d="M18.7307 5.39746L5.39738 18.7308"/></svg>"##;
    const SVG_CONFIRM: &str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200" fill="#000000"><path d="M892.064 261.888a31.936 31.936 0 0 0-45.216 1.472L421.664 717.248l-220.448-185.216a32 32 0 1 0-41.152 48.992l243.648 204.704a31.872 31.872 0 0 0 20.576 7.488 31.808 31.808 0 0 0 23.36-10.112L893.536 307.136a32 32 0 0 0-1.472-45.248z"/></svg>"##;

    fn create_button_pair(xy: (i32, i32)) -> (Button, Button) {
        let mut cancel = SvgImage::from_data(SVG_CANCEL).unwrap();
        let mut confirm = SvgImage::from_data(SVG_CONFIRM).unwrap();
        cancel.scale(30, 30, true, true);
        confirm.scale(30, 30, true, true);

        let mut btn_cancel = Button::new(xy.0, xy.1, 30, 30, None);
        let mut btn_confirm = Button::new(xy.0 + 40, xy.1, 30, 30, None);
        btn_cancel.set_frame(FrameType::NoBox);
        btn_confirm.set_frame(FrameType::NoBox);
        btn_cancel.visible_focus(false);
        btn_confirm.visible_focus(false);

        btn_cancel.set_color(Color::White);
        btn_confirm.set_color(Color::White);
        btn_cancel.set_image(cancel.into());
        btn_confirm.set_image(confirm.into());

        btn_cancel.set_callback(|btn| {
            btn.set_pos(90, 90);
            btn.redraw();
        });
        btn_confirm.set_callback(|btn| {
            btn.set_pos(110, 110);
        });

        (btn_cancel, btn_confirm)
    }

    #[test]
    fn sf() {
        let app = app::App::default();
        let mut wind = Window::new(640, 360, 1280, 720, "Hello from rust");

        create_button_pair((100, 100));

        // wind.add(&btn1);
        // wind.add(&btn2);
        // btn1.center_of(&wind);
        // btn2.center_of(&wind);

        wind.end();
        wind.show();
        app.run().unwrap();
    }

    #[test]
    fn btn() {
        let btn_cancel = SvgImage::from_data(SVG_CANCEL);
        let btn_confirm = SvgImage::from_data(SVG_CONFIRM);

        println!("{} | {}", btn_cancel.is_ok(), btn_cancel.is_ok());
    }

    fn closure() {
        fn create_pair<F>(on_cancel: F) where F: FnMut(&mut Button) + 'static {
            on_cancel();
        }

        let count = Rc::new(RefCell::new(0));

        create_pair({
            let count = count.clone();
            move || {
                *count.borrow_mut() += 1;
                let c = *count.borrow_mut();
                println!("current: {c}");
            }
        });
    }
}