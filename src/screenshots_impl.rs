use screenshots::{DisplayInfo, Screen};
use crate::declares::CaptureInfo;

/// `screenshots` 库拿到的坐标基本为物理坐标
pub struct ScreenshotsImpl {}

impl ScreenshotsImpl {
    /// 获取指定id的屏幕
    fn get_screen_by_id(screen_id: u32) -> Option<Screen> {
        // 获取所有屏幕
        let screens = Screen::all().unwrap();

        // 寻找指定屏幕
        for screen in screens {
            if screen.display_info.id == screen_id {
                // 找到则返回指定屏幕
                return Some(screen);
            }
        };

        // 没有目标屏幕则返回 None
        None
    }

    /// 获取指定点所在的屏幕
    fn get_screen_by_point(px: i32, py: i32) -> Option<Screen> {
        Screen::from_point(px, py).ok()
    }

    /// 获取指定点所在的屏幕信息
    pub fn get_by_point(px: i32, py: i32) -> Option<DisplayInfo> {
        ScreenshotsImpl::get_screen_by_point(px, py).map_or(None, |v| Some(v.display_info))
    }

    /// 截取所有屏幕
    pub fn capture_all() -> Vec<CaptureInfo> {
        // 获取所有屏幕
        let screens = Screen::all().unwrap();

        // 储存所有屏幕截图
        let mut shoots: Vec<CaptureInfo> = vec![];

        for screen in screens {
            let image = screen.capture().unwrap();
            shoots.push(CaptureInfo {
                screen_id: screen.display_info.id,
                width: image.width(),
                height: image.height(),
                buffer: image.to_png().unwrap_or(vec![]),
            });
        }

        shoots
    }

    /// 截取指定id的屏幕
    pub fn capture_by_id(screen_id: u32) -> Option<CaptureInfo> {
        match ScreenshotsImpl::get_screen_by_id(screen_id) {
            Some(screen) => {
                let image = screen.capture().unwrap();
                Some(CaptureInfo {
                    screen_id: screen.display_info.id,
                    width: image.width(),
                    height: image.height(),
                    buffer: image.to_png().unwrap_or(vec![]),
                })
            }
            None => None,
        }
    }

    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标. 需要转换为 sf=1)
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        match ScreenshotsImpl::get_screen_by_id(screen_id) {
            Some(screen) => {
                let image = screen.capture_area(x, y, w, h).unwrap();
                Some(CaptureInfo {
                    screen_id: screen.display_info.id,
                    width: image.width(),
                    height: image.height(),
                    buffer: image.to_png().unwrap_or(vec![]),
                })
            }
            None => None,
        }
    }

    /// 截取指定点所在的屏幕的指定区域 (px,py为全局坐标, x,y为相对于当前屏幕的x,y坐标. 需要转换为 sf=1)
    pub fn capture_area_by_point(px: i32, py: i32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        match ScreenshotsImpl::get_screen_by_point(px, py) {
            Some(screen) => {
                let image = screen.capture_area(x, y, w, h).unwrap();
                Some(CaptureInfo {
                    screen_id: screen.display_info.id,
                    width: image.width(),
                    height: image.height(),
                    buffer: image.to_png().unwrap_or(vec![]),
                })
            }
            None => None,
        }
    }
}

