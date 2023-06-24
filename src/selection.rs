use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
    dpi::LogicalSize,
};
use winit::dpi::LogicalPosition;
use winit::event::DeviceEvent;
use winit::platform::windows::WindowBuilderExtWindows;
use winit::window::{Fullscreen, Icon, Theme};

pub fn get_select_area() {
    // 事件循环实例
    let event_loop = EventLoop::new();
    // 窗口创建
    let window = WindowBuilder::new()
        // 无边框
        // .with_decorations(false)
        // 任务栏隐藏
        .with_skip_taskbar(true)
        // 置顶
        .with_always_on_top(true)
        // 透明
        .with_transparent(true)
        // 全屏
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        // 标题
        .with_title("截图")
        // 事件循环绑定
        .build(&event_loop)
        .unwrap();

    let icon = Icon::from_rgba(vec![255, 0, 0, 255], 1, 1).unwrap();
    window.set_window_icon(Some(icon));

    // 记录鼠标位置
    let mut last_mouse_position = LogicalPosition::new(0.0, 0.0);

    // 起点
    let mut start_point: Option<LogicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    // 关闭
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    // 右键退出
                    WindowEvent::MouseInput { button: MouseButton::Right, .. } => control_flow.set_exit(),
                    // 左键按下
                    WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                        start_point = Some(last_mouse_position.clone());
                    }
                    // 左键松开
                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                        let position = last_mouse_position.clone();
                        if let Some(start) = start_point {
                            let end = position;
                            let width = (end.x - start.x).abs() as u32;
                            let height = (end.y - start.y).abs() as u32;
                            let (x, y) = (start.x.min(end.x) as u32, start.y.min(end.y) as u32);

                            // Code to use selected region.
                            println!("Selected region: top_left=({}, {}), width={}, height={}", x, y, width, height);
                        }
                        start_point = None;
                    }
                    // 实时记录鼠标位置
                    WindowEvent::CursorMoved { position, .. } => {
                        last_mouse_position = position.to_logical(window.scale_factor());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}