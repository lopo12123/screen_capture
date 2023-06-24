use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
    dpi::LogicalSize,
};
use winit::dpi::LogicalPosition;
use winit::event::DeviceEvent;

fn capture() {
    let (window_width, window_height) = (800.0, 600.0);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mouse selection")
        .with_inner_size(LogicalSize::new(window_width, window_height))
        .build(&event_loop)
        .unwrap();

    // 记录鼠标位置
    let mut last_mouse_position = LogicalPosition::new(0.0, 0.0);

    // 起点
    let mut start_point: Option<LogicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    // 关闭
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn tt() {
        capture();
    }
}