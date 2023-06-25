use crate::declares::ScreenInfo;
use crate::fltk_impl::FltkImpl;
use crate::screenshots_impl::ScreenshotsImpl;

/// ffi 暴露的方法
pub struct FfiExport {}

impl FfiExport {
    /// 获取所有屏幕信息
    pub fn get_screens() -> Vec<ScreenInfo> {
        let mut screens: Vec<ScreenInfo> = vec![];
        let ss_screens = ScreenshotsImpl::get_screens();

        for mut ss_screen in ss_screens {
            let sf = ss_screen.scale_factor;
            let x_physical = (ss_screen.x as f32 / sf) as i32;
            let y_physical = (ss_screen.y as f32 / sf) as i32;
            let w_physical = (ss_screen.width as f32 / sf) as i32;
            let h_physical = (ss_screen.height as f32 / sf) as i32;

            screens.push(ScreenInfo {
                is_primary: ss_screen.is_primary,
                screen_id: ss_screen.id,
                screen_num: FltkImpl::get_screen_num(x_physical, y_physical),
                scale_factor: sf,
                xywh_physical: (x_physical, y_physical, w_physical, h_physical),
                xywh_logic: (ss_screen.x, ss_screen.y, ss_screen.width as i32, ss_screen.height as i32),
            });
        }

        screens
    }
}