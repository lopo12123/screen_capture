use fltk::app::{screen_count, screen_num, screen_scale, screen_xywh};
use crate::declares::ScreenInfoFltk;

pub mod box_selection_impl;

pub struct FltkImpl {}

impl FltkImpl {
    /// 获取所有的屏幕信息
    pub fn get_screens() -> Vec<ScreenInfoFltk> {
        let mut screens = vec![];
        for screen_num in 0..screen_count() {
            screens.push(ScreenInfoFltk {
                screen_num,
                scale_factor: screen_scale(screen_num),
                xywh: screen_xywh(screen_num),
            });

            println!("{:?}", ScreenInfoFltk {
                screen_num,
                scale_factor: screen_scale(screen_num),
                xywh: screen_xywh(screen_num),
            });
        }
        screens
    }

    /// 获取坐标所在的屏幕 screen_num
    pub fn get_screen_num(x: i32, y: i32) -> i32 {
        screen_num(x, y)
    }

    /// 交互式选择某区域
    pub fn request_select() {
        // TODO
    }
}
