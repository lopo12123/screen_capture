use fltk::app::{screen_num};
use crate::declares::{ScreenInfo};
use crate::fltk_impl::box_selection_impl::BoxSelectionImpl;
use crate::screenshots_impl::ScreenshotsImpl;

pub mod box_selection_impl;

pub struct FltkImpl {}

impl FltkImpl {
    /// 获取所有的屏幕信息 (主屏幕在第一个)
    pub fn get_screens() -> Vec<ScreenInfo> {
        let mut primary_screen: Option<ScreenInfo> = None;
        let mut screens: Vec<ScreenInfo> = vec![];
        let ss_screens = ScreenshotsImpl::get_screens();

        for ss_screen in ss_screens {
            let sf = ss_screen.scale_factor;
            // let x_physical = (ss_screen.x as f32) as i32;
            // let y_physical = (ss_screen.y as f32) as i32;
            // let w_physical = (ss_screen.width as f32 * sf) as i32;
            // let h_physical = (ss_screen.height as f32 * sf) as i32;

            let info = ScreenInfo {
                is_primary: ss_screen.is_primary,
                screen_id: ss_screen.id,
                // TODO 有异常, 获取屏幕数不准
                screen_num: screen_num(
                    (ss_screen.x as f32 * sf) as i32,
                    (ss_screen.y as f32 * sf) as i32,
                ),
                scale_factor: sf,
                xywh_real: (
                    ss_screen.x,
                    ss_screen.y,
                    (ss_screen.width as f32 * sf) as i32,
                    (ss_screen.height as f32 * sf) as i32,
                ),
            };

            if ss_screen.is_primary {
                primary_screen = Some(info);
            } else {
                screens.push(info);
            }
        }

        match primary_screen {
            Some(v) => screens.insert(0, v),
            None => {}
        }

        screens
    }

    /// 交互式选择某区域
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn request_select() -> Option<(u32, i32, i32, i32, i32)> {
        let mut screens = FltkImpl::get_screens();
        // screens = vec![screens[1].clone()];
        let mut task = BoxSelectionImpl::new(screens);
        task.run()
    }
}
