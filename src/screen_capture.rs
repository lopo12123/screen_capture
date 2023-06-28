use fltk::app;
use crate::declares::{CaptureInfo, ScreenInfo};
use crate::fltk_impl::FltkImpl;
use crate::screenshots_impl::ScreenshotsImpl;
use crate::utils::{get_origin_xywh, p1p2_to_xywh};

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
    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标. 需要转换为 sf=1)
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_area_by_id(screen_id, x, y, w, h)
    }

    /// re-export
    ///
    /// 截取指定点所在的屏幕的指定区域 (px,py为全局坐标, x,y为相对于当前屏幕的x,y坐标. 需要转换为 sf=1)
    pub fn capture_area_by_point(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        ScreenshotsImpl::capture_area_by_point(px, py, x, y, w, h)
    }


    /// re-export
    ///
    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, scale_factor: f32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_bounding(sfp: Option<f32>) -> Option<(u32, f32, i32, i32, i32, i32)> {
        let sfp = match sfp {
            Some(v) => {
                println!("The 'request_bounding' method is called with 'sfp' of {v}, continuing ...");
                v
            }
            None => {
                println!("The 'request_bounding' method is called without 'sfp', performing auto-detect ...");
                let p = app::get_mouse();
                println!("- Mouse coordinates detected: {:?}", p);
                match FltkImpl::get_screen_of_pointer(p) {
                    Some(v) => {
                        println!("- Automatically get an 'sfp' of {} (screen {{{}}})", v.scale_factor, v.screen_num);
                        v.scale_factor
                    }
                    None => {
                        println!("- Failed to get 'sfp', use the default value of 1.0.");
                        1.0
                    }
                }
            }
        };

        println!("========== ========= ========== ========= ========== =========");

        FltkImpl::request_bounding(sfp)
    }

    /// re-export
    ///
    /// 交互式选择某区域并截取
    pub fn request_capture(sfp: Option<f32>) -> Option<CaptureInfo> {
        println!("The 'request_capture' method is called, and there are two tasks to be performed.");

        println!("- Task 1 start. performing 'request_bounding' ...");
        println!("========== ========= ========== ========= ========== =========");
        match ScreenCapture::request_bounding(sfp) {
            Some((sid, sf, x1, y1, x2, y2)) => {
                println!("- End of Task 1. The user has selected the area: [start = ({x1}, {y1}), end = ({x2}, {y2}), sf = {sf}] on screen {{{sid}}}");
                println!("- Task 2 start. performing 'capture_area' ...");
                let (mut x, mut y, mut w, mut h) = p1p2_to_xywh(x1, y1, x2, y2);
                if sf != 1.0 {
                    println!("'scale_factor' is detected not to be 1.0, performing coordinate conversion...");
                    let (_x, _y, _w, _h) = get_origin_xywh(sf, (x, y, w as i32, h as i32));
                    x = _x;
                    y = _y;
                    w = _w as u32;
                    h = _h as u32;
                }
                println!("Capturing target area... (xywh = ({w}, {y}, {w}, {h}))");
                println!("========== ========= ========== ========= ========== =========");
                ScreenCapture::capture_area_by_id(sid, x, y, w, h)
            }
            None => {
                println!("- Task 1 end.");
                None
            }
        }
    }
}