use screenshots::{DisplayInfo, Screen};
use crate::declares::CaptureInfo;

/// `screenshots` 库拿到的坐标基本为物理坐标
pub struct ScreenshotsImpl {}

impl ScreenshotsImpl {
    /// 截取所有屏幕
    pub fn capture_all() -> Vec<CaptureInfo> {
        // 获取所有屏幕
        let screens = Screen::all().unwrap();

        // 储存所有屏幕截图
        let mut shoots: Vec<CaptureInfo> = vec![];

        for screen in screens {
            let DisplayInfo { id, x, y, scale_factor, .. } = screen.display_info;
            let image = screen.capture().unwrap();
            shoots.push(CaptureInfo {
                screen_id: id,
                scale_factor: scale_factor as f64,
                physical_x: x,
                physical_y: y,
                physical_width: image.width(),
                physical_height: image.width(),
                buffer: image.to_png().unwrap_or(vec![]),
            });
        }

        shoots
    }
}

