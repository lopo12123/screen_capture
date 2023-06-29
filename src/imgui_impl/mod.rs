use std::cmp::{max, min};
use crate::declares::{CaptureInfo};
use crate::imgui_impl::prefab::BoundingBox;

mod core;
mod prefab;

pub struct ImguiImpl {}

impl ImguiImpl {
    /// 计算将所有屏幕盖住的窗口的 xywh
    fn calc_bounding(captures: &Vec<CaptureInfo>) -> BoundingBox {
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
    pub fn bounding(captures: Vec<CaptureInfo>) {
        let system = core::prepare_system(ImguiImpl::calc_bounding(&captures));

        system.main_loop(move |_, ui| {
            ui.window("My window via callback")
                .position([10.0, 10.0], imgui::Condition::Always)
                .build(|| {
                    ui.text("This content appears in a window");

                    // Everything in this callback appears in the window, like this button:
                    ui.button("This button");
                });

            ui.window("鼠标位置")
                .build(|| ui.text(format!("Some variable: {:?}", ui.io().mouse_pos)));
        });
    }
}