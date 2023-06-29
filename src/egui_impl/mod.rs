use std::cmp::{max, min};
use eframe::egui;
use crate::declares::CaptureInfo;
use crate::egui_impl::core::MyApp;

mod core;

pub struct EguiImpl {}

impl EguiImpl {
    /// 计算将所有屏幕盖住的窗口的 xywh
    fn calc_bounding(captures: &Vec<CaptureInfo>) -> (i32, i32, i32, i32) {
        let (mut xl, mut yl, mut xh, mut yh) = (0i32, 0i32, 0i32, 0i32);

        for capture in captures {
            xl = min(xl, capture.physical_x);
            yl = min(yl, capture.physical_y);
            xh = max(xh, capture.physical_x + capture.physical_width);
            yh = max(yh, capture.physical_y + capture.physical_height);
        }

        (xl, yl, xh - xl, yh - yl)
    }

    /// 传入图像信息开始交互式选择区域
    pub fn bounding(captures: Vec<CaptureInfo>) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        };
        eframe::run_native(
            "My egui App",
            options,
            Box::new(|_cc| Box::<MyApp>::default()),
        )
    }
}