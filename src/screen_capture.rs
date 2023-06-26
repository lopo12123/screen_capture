use crate::declares::{CaptureInfo, ScreenInfo};
use crate::fltk_impl::FltkImpl;
use crate::screenshots_impl::ScreenshotsImpl;

/// ffi 暴露的方法
pub struct ScreenCapture {}

impl ScreenCapture {
    /// re-export [FltkImpl::get_screens]
    ///
    /// 获取所有屏幕信息
    pub fn get_screens() -> Vec<ScreenInfo> {
        FltkImpl::get_screens()
    }

    /// re-export [ScreenshotsImpl::capture_all]
    ///
    /// 截取所有屏幕
    pub fn capture_all() -> Vec<CaptureInfo> {
        ScreenshotsImpl::capture_all()
    }

    /// re-export [ScreenshotsImpl::capture_by_id]
    ///
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    pub fn capture_by_id(screen_id: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_by_id(screen_id)
    }

    /// re-export [ScreenshotsImpl::capture_area_by_id]
    ///
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_area_by_id(screen_id, x, y, w, h)
    }


    /// re-export [FltkImpl::request_select]
    ///
    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_select() -> Option<(u32, i32, i32, i32, i32)> {
        FltkImpl::request_select()
    }
}