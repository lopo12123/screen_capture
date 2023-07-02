use std::cmp::{max, min};
use crate::declares::{CaptureInfo, SelectedImage};
use crate::imgui_impl::prefab::BoundingBox;

mod core;
mod prefab;

pub struct ImguiImpl {}

impl ImguiImpl {
    /// 计算将所有屏幕盖住的窗口的 xywh
    fn calc_bounding(captures: &Vec<CaptureInfo>) -> BoundingBox {
        let (mut xl, mut yl, mut xh, mut yh) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        for capture in captures {
            xl = min(xl, capture.physical_x);
            yl = min(yl, capture.physical_y);
            xh = max(xh, capture.physical_x + capture.physical_width as i32);
            yh = max(yh, capture.physical_y + capture.physical_height as i32);
        }

        (xl, yl, xh - xl, yh - yl)
    }

    /// 传入图像信息开始交互式选择区域
    pub fn bounding(captures: Vec<CaptureInfo>) -> Option<SelectedImage> {
        let system = core::System::new(ImguiImpl::calc_bounding(&captures), captures);
        let (exit_code, select_area) = system.run();

        println!("Task End. (exit_code = {exit_code}, p1p2 = {:?}, valid = {})", select_area.p1p2, select_area.valid);

        if select_area.valid {
            let [x1, y1, x2, y2] = select_area.p1p2.unwrap();
            Some(SelectedImage {
                p1p2: vec![x1 as f64, y1 as f64, x2 as f64, y2 as f64],
                buffer: select_area.get_buffer(),
            })
        } else {
            None
        }
    }
}