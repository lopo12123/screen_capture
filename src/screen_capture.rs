use crate::declares::{CaptureInfo};
use crate::egui_impl::{EguiImpl};
use crate::screenshots_impl::ScreenshotsImpl;

/// 作为模块暴露的方法
pub struct ScreenCapture {}

#[allow(unused)]
impl ScreenCapture {
    /// 截取所有屏幕
    pub fn capture() -> Vec<CaptureInfo> {
        ScreenshotsImpl::capture_all()
    }

    /// 截取所有屏幕并裁剪
    pub fn capture_with_crop() {
        let captures = ScreenshotsImpl::capture_all();
        for capture in &captures {
            println!("{}, {}, {}, {}", capture.physical_x, capture.physical_y, capture.physical_width, capture.physical_height);
        }

        EguiImpl::bounding(captures);
        // EguiImpl::bounding(vec![captures[1].clone()]);

        println!("Task End!");
    }
}