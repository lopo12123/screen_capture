use std::cmp::{max, min};
use eframe::egui;
use eframe::emath::Numeric;
use crate::declares::CaptureInfo;
use crate::egui_impl::core::MyApp;

mod core;
mod assets;

pub struct EguiImpl {}

impl EguiImpl {
    /// 计算将所有屏幕盖住的窗口的 xywh
    ///
    /// `(x: f32, y: f32, w: f32, h: f32)`
    fn calc_bounding(captures: &Vec<CaptureInfo>) -> (f32, f32, f32, f32) {
        let (mut xl, mut yl, mut xh, mut yh) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        for capture in captures {
            xl = min(xl, capture.physical_x);
            yl = min(yl, capture.physical_y);
            xh = max(xh, capture.physical_x + capture.physical_width);
            yh = max(yh, capture.physical_y + capture.physical_height);
        }

        (xl as f32, yl as f32, (xh - xl) as f32, (yh - yl) as f32)
    }

    /// 传入图像信息开始交互式选择区域
    pub fn bounding(captures: Vec<CaptureInfo>) -> Result<(), eframe::Error> {
        let (x, y, w, h) = EguiImpl::calc_bounding(&captures);
        println!("Get Bounding: ({x}, {y}, {w}, {h})");

        let options = eframe::NativeOptions {
            always_on_top: true,
            // decorated: false,
            // resizable: false,
            initial_window_pos: Some(egui::pos2(x + 1.0, y + 1.0)),
            initial_window_size: Some(egui::vec2(3000.0, 2000.0)),
            ..Default::default()
        };

        // let ;
        eframe::run_native(
            "截图",
            options,
            Box::new(|ctx| Box::new(MyApp::new(ctx, captures))),
        )
    }
}