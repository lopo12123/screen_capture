use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::platform::windows::WindowBuilderExtWindows;
use glium::glutin::window::WindowBuilder;

const TITLE: &str = "截图";

/// 窗口预制件
///
/// - 标题 "截图"
/// - 无边框
/// - 置顶
/// - 无任务栏
/// - 禁用改变大小
/// - 指定尺寸+位置
pub fn window_prefab(logical_xywh: (f64, f64, f64, f64)) -> WindowBuilder {
    let (x, y, width, height) = logical_xywh;
    WindowBuilder::default()
        .with_title(String::from(TITLE))
        // .with_decorations(false)
        .with_always_on_top(true)
        .with_skip_taskbar(true)
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(width, height))
        .with_position(LogicalPosition::new(x, y))
}