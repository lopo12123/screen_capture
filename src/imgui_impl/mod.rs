use std::cmp::{max, min};
use crate::declares::{CaptureInfo};
use crate::imgui_impl::prefab::BoundingBox;

mod core;
mod prefab;

pub struct ImguiImpl {}

impl ImguiImpl {
    /// 计算将所有屏幕盖住的窗口的 xywh
    fn calc_bounding(captures: &Vec<CaptureInfo>) -> BoundingBox {
        let (mut xl, mut yl, mut xh, mut yh) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        for capture in captures {
            // println!("Screen To Bounding: xywh = ({}, {}, {}, {})", capture.physical_x, capture.physical_y, capture.physical_width, capture.physical_height);

            xl = min(xl, capture.physical_x);
            yl = min(yl, capture.physical_y);
            xh = max(xh, capture.physical_x + capture.physical_width);
            yh = max(yh, capture.physical_y + capture.physical_height);
        }

        (xl, yl, xh - xl, yh - yl)
    }

    /// 传入图像信息开始交互式选择区域
    pub fn bounding(captures: Vec<CaptureInfo>) {
        let system = core::System::new(ImguiImpl::calc_bounding(&captures));
        let exit_code = system.run();

        println!("Task End. (exit_code: {exit_code})");
    }
}