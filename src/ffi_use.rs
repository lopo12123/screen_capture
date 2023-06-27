use crate::ffi_use::to_ffi::{FfiConvertible, ScreenInfoFfi, CaptureInfoFfi, BoundingBox};
use crate::screen_capture::ScreenCapture;

pub mod to_ffi;

#[napi]
pub struct FfiUse {}

#[napi]
impl FfiUse {
    /// ffi compatible conversion [ScreenCapture::get_screens]
    ///
    /// 获取所有屏幕信息
    #[napi]
    pub fn get_screens() -> napi::Result<Vec<ScreenInfoFfi>> {
        let mut screens = vec![];

        for screen in ScreenCapture::get_screens() {
            screens.push(screen.to_ffi())
        }

        Ok(screens)
    }

    /// ffi compatible conversion [ScreenCapture::capture_all]
    ///
    /// 截取所有屏幕
    #[napi]
    pub fn capture_all() -> napi::Result<Vec<CaptureInfoFfi>> {
        let mut screens = vec![];

        for screen in ScreenCapture::capture_all() {
            screens.push(screen.to_ffi())
        }

        Ok(screens)
    }

    /// ffi compatible conversion [ScreenCapture::capture_by_id]
    ///
    /// 截取指定id的屏幕
    #[napi]
    pub fn capture_by_id(screen_id: u32) -> napi::Result<Option<CaptureInfoFfi>> {
        Ok(match ScreenCapture::capture_by_id(screen_id) {
            Some(v) => Some(v.to_ffi()),
            None => None
        })
    }

    /// ffi compatible conversion [ScreenCapture::capture_area_by_id]
    ///
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    #[napi]
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> napi::Result<Option<CaptureInfoFfi>> {
        Ok(match ScreenCapture::capture_area_by_id(screen_id, x, y, w, h) {
            Some(v) => Some(v.to_ffi()),
            None => None
        })
    }


    /// ffi compatible conversion [ScreenCapture::request_select]
    ///
    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    #[napi]
    pub fn request_select() -> napi::Result<Option<BoundingBox>> {
        // TODO
        Ok(match ScreenCapture::request_select(1.0) {
            Some(area) => Some(BoundingBox::from_tuple(area)),
            None => None
        })
    }
}