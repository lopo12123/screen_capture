use fltk::app;
use crate::declares::{CaptureInfo, ScreenInfo};
use crate::fltk_impl::FltkImpl;
use crate::screenshots_impl::ScreenshotsImpl;

/// ffi 暴露的方法
pub struct ScreenCapture {}

#[allow(unused)]
impl ScreenCapture {
    /// re-export
    ///
    /// 获取所有屏幕信息
    pub fn get_screens() -> Vec<ScreenInfo> {
        FltkImpl::get_screens()
    }

    /// re-export
    ///
    /// 截取所有屏幕
    pub fn capture_all() -> Vec<CaptureInfo> {
        ScreenshotsImpl::capture_all()
    }

    /// re-export
    ///
    /// 截取指定id的屏幕
    pub fn capture_by_id(screen_id: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_by_id(screen_id)
    }

    /// re-export
    ///
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_area_by_id(screen_id, x, y, w, h)
    }

    /// re-export
    ///
    /// 截取指定点所在的屏幕的指定区域 (px,py为全局坐标, x,y为相对于当前屏幕的x,y坐标)
    pub fn capture_area_by_point(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_area_by_point(px, py, x, y, w, h)
    }


    /// re-export
    ///
    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_select(sfp: Option<f32>) -> Option<(u32, i32, i32, i32, i32)> {
        println!("========== ========= ========== ========= ========== =========");

        let sfp = match sfp {
            Some(v) => v,
            None => {
                println!("Call request_select without 'sfp', use automatic fetching");
                let p = app::get_mouse();
                println!("Mouse coordinates detected: {:?}", p);
                match FltkImpl::get_screen_of_pointer(p) {
                    Some(v) => {
                        println!("Automatically get an 'sfp' of {}", v.scale_factor);
                        v.scale_factor
                    }
                    None => {
                        println!("Failed to get 'sfp', use the default value of 1.0.");
                        1.0
                    }
                }
            }
        };

        println!("========== ========= ========== ========= ========== =========");

        FltkImpl::request_select(sfp)
    }
}