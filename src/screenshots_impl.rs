use screenshots::{DisplayInfo, Screen};
use crate::declares::CaptureInfo;

/// `screenshots` 库拿到的坐标基本为物理坐标
pub struct ScreenshotsImpl {}

impl ScreenshotsImpl {
    /// 获取指定id的屏幕
    fn get_screen(screen_id: u32) -> Option<Screen> {
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

    /// 获取所有的屏幕信息 (xy为逻辑坐标, wh为物理坐标)
    pub fn get_screens() -> Vec<DisplayInfo> {
        let mut screens = vec![];

        for screen in Screen::all().unwrap() {
            screens.push(screen.display_info);
        };

        screens
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
        match ScreenshotsImpl::get_screen(screen_id) {
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

    /// 截取指定id的屏幕的指定区域 (x,y为相对于当前屏幕的x,y坐标)
    pub fn capture_area_by_id(screen_id: u32, x: i32, y: i32, w: u32, h: u32) -> Option<CaptureInfo> {
        match ScreenshotsImpl::get_screen(screen_id) {
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

