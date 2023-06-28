use crate::ffi_use::to_ffi::{FfiConvertible, ScreenInfoFfi, CaptureInfoFfi, BoundingBox};
use crate::screen_capture::ScreenCapture;

pub mod to_ffi;

#[napi]
pub struct FfiUse {}

#[napi]
impl FfiUse {
    /// ffi compatible conversion
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

    /// ffi compatible conversion
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

    /// ffi compatible conversion
    ///
    /// 截取指定id的屏幕
    #[napi]
    pub fn capture_by_id(screen_id: u32) -> napi::Result<Option<CaptureInfoFfi>> {
        Ok(match ScreenCapture::capture_by_id(screen_id) {
            Some(v) => Some(v.to_ffi()),
            None => None
        })
    }

    /// ffi compatible conversion
    ///
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    #[napi]
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> napi::Result<Option<CaptureInfoFfi>> {
        Ok(match ScreenCapture::capture_area_by_id(screen_id, x, y, w, h) {
            Some(v) => Some(v.to_ffi()),
            None => None
        })
    }


    /// ffi compatible conversion
    ///
    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, scale_factor: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    #[napi]
    pub fn request_bounding(sfp: Option<f64>) -> napi::Result<Option<BoundingBox>> {
        Ok(match ScreenCapture::request_bounding(sfp.map_or(None, |v| Some(v as f32))) {
            Some(v) => Some(BoundingBox::from_tuple(v)),
            None => None
        })
    }

    /// ffi compatible conversion
    ///
    /// 交互式选择某区域并截取
    #[napi]
    pub fn request_capture(sfp: Option<f64>) -> napi::Result<Option<CaptureInfoFfi>> {
        Ok(match ScreenCapture::request_capture(sfp.map_or(None, |v| Some(v as f32))) {
            Some(v) => {
                Some(v.to_ffi())
            }
            None => None
        })
    }
}