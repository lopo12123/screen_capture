use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{Display, Surface};
use imgui::{Context, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use crate::imgui_impl::prefab::{BoundingBox, create_screen_pair};

pub struct System {
    // 主事件循环
    pub event_loop: EventLoop<()>,
    // winit 平台相关
    pub platform: WinitPlatform,
    // imgui 上下文
    pub imgui: Context,
    // glium Display
    pub display: Display,
    // gilum 渲染
    pub renderer: Renderer,
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            mut platform,
            mut imgui,
            display,
            mut renderer,
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            // 和窗口事件相关的逻辑 (在此处更新 imgui 内部时间系统)
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            // 主事件队列被清空 ==> 通知绘制 ui
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            // 绘制 ui
            Event::RedrawRequested(_) => {
                // 开启新的一帧
                let ui = imgui.new_frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 0.0);
                platform.prepare_render(ui, display.gl_window().window());
                renderer
                    .render(&mut target, imgui.render())
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            // 系统事件被发送到 winit
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => *control_flow = ControlFlow::Exit,
            // 其他事件
            event => {
                platform.handle_event(imgui.io_mut(), display.gl_window().window(), &event);
            }
        })
    }
}

pub fn prepare_system(physical_xywh: BoundingBox) -> System {
    // 事件循环
    let event_loop = EventLoop::new();

    // imgui 上下文
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    // winit 平台
    let mut platform = WinitPlatform::init(&mut imgui);

    // display 和 renderer
    let (display, renderer) = create_screen_pair(
        &mut imgui,
        &event_loop,
        physical_xywh,
    );

    platform.attach_window(
        imgui.io_mut(),
        display.gl_window().window(),
        HiDpiMode::Default,
        // HiDpiMode::Locked(1.0),
    );

    System {
        event_loop,
        platform,
        imgui,
        display,
        renderer,
    }
}