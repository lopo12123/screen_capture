use fltk::app;
use crate::declares::{ScreenInfo};
use crate::fltk_impl::box_selection_impl::BoxSelectionImpl;
use crate::screenshots_impl::ScreenshotsImpl;
use crate::utils::get_real_xywh_before_scale;

pub mod box_selection_impl;

pub struct FltkImpl {}

impl FltkImpl {
    /// 获取当前鼠标所在的屏幕
    pub fn get_screen_of_pointer(pointer: (i32, i32)) -> Option<ScreenInfo> {
        match app::Screen::num_at(pointer) {
            Ok(sn) => {
                let sf = app::screen_scale(sn);

                // 真实宽高和原始宽高
                let (x, y, w, h) = app::screen_xywh(sn);
                let (x1, y1, w1, h1) = get_real_xywh_before_scale(sf, (x, y, w, h));

                // 中心点
                let xc = x1 + w1 / 2;
                let yc = y1 + h1 / 2;

                // 中心点所在的屏幕信息
                let display_info = ScreenshotsImpl::get_by_point(xc, yc);

                Some(ScreenInfo {
                    is_primary: display_info.map_or(false, |v| v.is_primary),
                    screen_id: display_info.map_or(1, |v| v.id),
                    screen_num: sn,
                    scale_factor: sf,
                    xywh_real: (x, y, w, h),
                    xywh_origin: (x1, y1, w1, h1),
                })
            }
            Err(_) => None
        }
    }

    /// 获取所有的屏幕信息
    pub fn get_screens() -> Vec<ScreenInfo> {
        let mut screens: Vec<ScreenInfo> = vec![];

        for sn in 0..app::screen_count() {
            let sf = app::screen_scale(sn);

            // 真实宽高和原始宽高
            let (x, y, w, h) = app::screen_xywh(sn);
            let (x1, y1, w1, h1) = get_real_xywh_before_scale(sf, (x, y, w, h));

            // 中心点
            let xc = x1 + w1 / 2;
            let yc = y1 + h1 / 2;

            // 中心点所在的屏幕信息
            let display_info = ScreenshotsImpl::get_by_point(xc, yc);

            screens.push(ScreenInfo {
                is_primary: display_info.map_or(false, |v| v.is_primary),
                screen_id: display_info.map_or(1, |v| v.id),
                screen_num: sn,
                scale_factor: sf,
                xywh_real: (x, y, w, h),
                xywh_origin: (x1, y1, w1, h1),
            })
        }

        screens
    }

    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, scale_factor: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_bounding(sfp: f32) -> Option<(u32, f32, i32, i32, i32, i32)> {
        let screens = FltkImpl::get_screens();
        let mut task = BoxSelectionImpl::new(sfp, screens);
        task.run()
    }
}
