use crate::declares::{CaptureInfo, SelectedImage};
use crate::imgui_impl::ImguiImpl;
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
    pub fn capture_with_crop() -> Option<SelectedImage> {
        let captures = ScreenshotsImpl::capture_all();
        ImguiImpl::bounding(captures)
    }
}