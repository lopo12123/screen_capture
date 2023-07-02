use glium::{Display, glutin};
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event_loop::{EventLoopWindowTarget};
use glium::glutin::platform::windows::WindowBuilderExtWindows;
use glium::glutin::window::WindowBuilder;
use imgui_glium_renderer::Renderer;

const TITLE: &str = "截图";

/// 物理坐标系下的 xywh
pub type BoundingBox = (i32, i32, i32, i32);

/// 窗口预制件 (使用逻辑坐标)
///
/// - 标题 "截图"
/// - 无边框
/// - 置顶
/// - 无任务栏
/// - 禁用改变大小
/// - 指定尺寸+位置
pub fn create_screen_pair(
    mut ctx: &mut imgui::Context,
    event_loop: &EventLoopWindowTarget<()>,
    physical_xywh: BoundingBox,
) -> (Display, Renderer) {
    let (x, y, w, h) = physical_xywh;

    let position = PhysicalPosition::new(x, y);
    let inner_size = PhysicalSize::new(w, h);

    let builder = WindowBuilder::default()
        .with_title(String::from(TITLE))
        .with_decorations(false)
        .with_always_on_top(true)
        .with_skip_taskbar(true)
        .with_resizable(false)
        .with_position(position)
        .with_inner_size(inner_size);

    let display = Display::new(
        builder,
        glutin::ContextBuilder::new().with_vsync(true),
        event_loop,
    ).unwrap();

    let renderer = Renderer::init(&mut ctx, &display).unwrap();

    (display, renderer)
}


/// 交互式选择选中的区域
pub struct SelectedArea {
    /// 即: \[xmin, ymin, xmax, ymax\]
    pub p1p2: [f32; 4],
    /// 目标区域的 rgba 阵列
    pub rgba: Vec<Vec<(u8, u8, u8, u8)>>,
}

impl SelectedArea {
    pub fn new(p1p2: [f32; 4], rgba: Vec<Vec<(u8, u8, u8, u8)>>) -> SelectedArea {
        SelectedArea { p1p2, rgba }
    }

    /// 检查是否是同一区域 (同一区域则不再执行捕获)
    pub fn check(&self, other_p1p2: [f32; 4]) -> bool {
        self.p1p2 == other_p1p2
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn tt() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 3, 4];

        println!("equal: {}", a == b);
    }
}