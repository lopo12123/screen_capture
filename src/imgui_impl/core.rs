use std::cmp::{max, min};
use glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{Display, Surface};
use imgui::{Context, ImColor32, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use crate::imgui_impl::prefab::{BoundingBox, create_screen_pair};

fn calc_select_area(p1: [f32; 2], p2: [f32; 2]) -> [f32; 4] {
    let [x1, y1] = p1;
    let [x2, y2] = p2;

    [
        if x1 < x2 { x1 } else { x2 },
        if y1 < y2 { y1 } else { y2 },
        if x1 > x2 { x1 } else { x2 },
        if y1 > y2 { x1 } else { x2 },
    ]
}

pub struct System {
    /// 主事件循环
    pub event_loop: EventLoop<()>,
    /// winit 平台相关
    pub platform: WinitPlatform,
    /// imgui 上下文
    pub imgui: Context,
    /// glium Display
    pub display: Display,
    /// gilum 渲染
    pub renderer: Renderer,

    // 点位坐标为 physical 坐标系
    /// 是否正在绘制矩形
    pub is_drawing_rect: bool,
    /// 选择的区域 \[xl,yl, xh, yh\]
    pub select_area: Option<[f32; 4]>,
    /// 绘制的起点
    start_point: Option<[f32; 2]>,
    /// 绘制的终点
    end_point: Option<[f32; 2]>,
    /// 当前的鼠标位置
    curr_point: Option<[f32; 2]>,
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) -> i32 {
        let System {
            mut event_loop,
            mut platform,
            mut imgui,
            display,
            mut renderer,
            mut is_drawing_rect,
            mut select_area,
            mut start_point,
            mut end_point,
            mut curr_point,
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run_return(move |event, _, control_flow| match event {
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

                // 绘制背景
                // TODO

                // 有起点 + (绘制中且有当前) 或 有终点
                let rect_end = if is_drawing_rect { curr_point } else { end_point };
                if start_point.is_some() && rect_end.is_some() {
                    // TODO 透明窗口装填矩形选框
                    ui.window("My window via callback")
                        .position([10.0, 10.0], imgui::Condition::Always)
                        .size([1000.0, 1000.0], imgui::Condition::Always)
                        .build(|| {
                            ui.get_window_draw_list()
                                .add_rect(
                                    start_point.unwrap(),
                                    rect_end.unwrap(),
                                    ImColor32::from_rgba(0x80, 0xc0, 0x40, 0xff),
                                )
                                .build();
                        });
                }

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 0.0);
                platform.prepare_render(ui, display.gl_window().window());
                renderer
                    .render(&mut target, imgui.render())
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            // 处理鼠标按下和松开事件
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button: MouseButton::Left, state, .. }, ..
            } => {
                if state == ElementState::Pressed {
                    // 按下: 设置 flag -> 设置起点 -> 重置终点
                    is_drawing_rect = true;
                    start_point = curr_point;
                    end_point = None
                } else {
                    // 释放: 设置 flag -> 设置终点 -> 计算区域
                    is_drawing_rect = false;
                    end_point = curr_point;
                    select_area = Some(calc_select_area(start_point.unwrap(), end_point.unwrap()));
                }
            }
            // 处理鼠标移动事件
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. }, ..
            } => {
                // FIXME: 处理边界问题
                curr_point = Some([position.x as f32, position.y as f32])
            }
            // 关闭窗口 (来自 winit)
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
        is_drawing_rect: false,
        select_area: None,
        start_point: None,
        end_point: None,
        curr_point: None,
    }
}