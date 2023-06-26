use fltk::app;
use fltk::app::{screen_num};
use crate::declares::{ScreenInfo};
use crate::fltk_impl::box_selection_impl::BoxSelectionImpl;
use crate::screenshots_impl::ScreenshotsImpl;

pub mod box_selection_impl;

pub struct FltkImpl {}

impl FltkImpl {
    /// 获取所有的屏幕信息
    pub fn get_screens() -> Vec<ScreenInfo> {
        let mut screens: Vec<ScreenInfo> = vec![];

        for sn in 0..app::screen_count() {
            let (x, y, w, h) = app::screen_xywh(sn);
            let sf = app::screen_scale(sn);
            screens.push(ScreenInfo {
                is_primary: ScreenshotsImpl::get_by_point(x, y).map_or(false, |v| v.is_primary),
                screen_id: ScreenshotsImpl::get_by_point(x, y).map_or(1, |v| v.id),
                screen_num: sn,
                scale_factor: sf,
                xywh_real: (x, y, w, h),
            })
        }

        screens
    }

    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_select() -> Option<(u32, i32, i32, i32, i32)> {
        let mut task = BoxSelectionImpl::new(FltkImpl::get_screens());
        task.run()
    }
}
