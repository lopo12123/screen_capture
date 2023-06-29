use screenshots::{DisplayInfo, Screen};
use crate::declares::CaptureInfo;

/// `screenshots` 库拿到的坐标基本为物理坐标
pub struct ScreenshotsImpl {}

impl ScreenshotsImpl {
    /// 截取所有屏幕 (失败的屏幕静默忽略)
    pub fn capture_all() -> Vec<CaptureInfo> {
        // 获取所有屏幕
        match Screen::all() {
            Ok(screens) => {
                // 储存所有屏幕截图
                let mut shoots: Vec<CaptureInfo> = vec![];

                for screen in screens {
                    match screen.capture() {
                        Ok(image) => {
                            let DisplayInfo { id, x, y, scale_factor, .. } = screen.display_info;
                            shoots.push(CaptureInfo {
                                screen_id: id,
                                scale_factor: scale_factor as f64,
                                physical_x: x,
                                physical_y: y,
                                physical_width: image.width() as i32,
                                physical_height: image.height() as i32,
                                buffer: image.to_png().unwrap_or(vec![]),
                            });
                        }
                        Err(_) => {
                            println!("Failed to get screen image");
                        }
                    }
                }

                shoots
            }
            Err(_) => {
                println!("Failed to get screen information");
                vec![]
            }
        }
    }
}

