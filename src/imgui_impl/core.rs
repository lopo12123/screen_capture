use std::cell::RefCell;
use std::rc::Rc;
use glium::glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{Display, Surface, Texture2d};
use imgui::{Context, ImColor32, StyleColor, TextureId, Textures};
use imgui_glium_renderer::{Renderer, Texture};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use glium::backend::Facade;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::texture::{RawImage2d};
use glium::uniforms::SamplerBehavior;
use crate::declares::CaptureInfo;
use crate::imgui_impl::prefab::{BoundingBox, create_screen_pair, SelectedArea};
use crate::utils::clamp;

/// 蒙层的颜色
const MASK_COLOR: ImColor32 = ImColor32::from_rgba(0x00, 0x00, 0x00, 0x66);

/// 计算范围约束后的点位
fn calc_constrained_point(physical_point: PhysicalPosition<f64>, bounding: BoundingBox) -> [f32; 2] {
    let (x, y, w, h) = bounding;

    [
        clamp(physical_point.x as i32, x, x + w) as f32,
        clamp(physical_point.y as i32, y, y + h) as f32,
    ]
}

/// 给定任意两点计算其包围的矩形的 '左上点' 和 '右下点',
/// 即: \[xmin, ymin, xmax, ymax\]
fn calc_select_area(p1: [f32; 2], p2: [f32; 2]) -> [f32; 4] {
    let [x1, y1] = p1;
    let [x2, y2] = p2;

    [
        if x1 < x2 { x1 } else { x2 },
        if y1 < y2 { y1 } else { y2 },
        if x1 > x2 { x1 } else { x2 },
        if y1 > y2 { y1 } else { y2 },
    ]
}

/// 给定大矩形(Rect1)宽高(wh)和小矩形(Rect2)任意两点(p1, p2),
/// 返回 'Rect2 - Rect1' 区域的四个矩形的 '左上点' 和 '右下点'
fn calc_bounding_rect(wh: [f32; 2], p1: [f32; 2], p2: [f32; 2]) -> [[[f32; 2]; 2]; 4] {
    let [w, h] = wh;
    let [x1, y1, x2, y2] = calc_select_area(p1, p2);

    return [
        [[0.0, 0.0], [x1, y2]],
        [[x1, 0.0], [w, y1]],
        [[0.0, y2], [x2, h]],
        [[x2, y1], [w, h]],
    ];
}

/// 载入图像纹理
fn load_screen_images(
    gl_ctx: &impl Facade,
    renderer_textures: &mut Textures<Texture>,
    screen_infos: Vec<CaptureInfo>,
) -> Vec<(TextureId, i32, i32, u32, u32)> {
    let mut texture_infos = vec![];

    for screen_info in screen_infos {
        let CaptureInfo {
            physical_x, physical_y,
            physical_width, physical_height,
            rgba,
            ..
        } = screen_info;
        let raw = RawImage2d::from_raw_rgba(rgba, (physical_width, physical_height));
        let gl_texture = Texture2d::new(gl_ctx, raw).unwrap();
        let texture = Texture {
            texture: Rc::new(gl_texture),
            sampler: SamplerBehavior::default(),
        };
        let texture_id = renderer_textures.insert(texture);
        texture_infos.push((
            texture_id,
            physical_x,
            physical_y,
            physical_width,
            physical_height,
        ));
    }

    texture_infos
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
    pub physical_xywh: BoundingBox,
    /// 各屏幕的纹理id及位置信息
    pub screen_texture_list: Vec<(TextureId, i32, i32, u32, u32)>,
    /// 是否正在绘制矩形
    pub is_drawing_rect: bool,
    /// 绘制的起点
    start_point: Option<[f32; 2]>,
    /// 绘制的终点
    end_point: Option<[f32; 2]>,
    /// 当前的鼠标位置
    curr_point: Option<[f32; 2]>,
}

impl System {
    pub fn new(physical_xywh: BoundingBox, captures: Vec<CaptureInfo>) -> System {
        // 事件循环
        let event_loop = EventLoop::new();

        // imgui 上下文
        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        // winit 平台
        let mut platform = WinitPlatform::init(&mut imgui);

        // display 和 renderer
        let (display, mut renderer) = create_screen_pair(
            &mut imgui,
            &event_loop,
            physical_xywh,
        );

        // 计算获取各屏幕图像的 textureId + xywh
        let screen_texture_list = load_screen_images(
            display.get_context(),
            renderer.textures(),
            captures,
        );

        // 窗口附加到 winit
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
            physical_xywh,
            screen_texture_list,
            is_drawing_rect: false,
            start_point: None,
            end_point: None,
            curr_point: None,
        }
    }

    pub fn run(self) -> (i32, SelectedArea) {
        let System {
            mut event_loop,
            mut platform,
            mut imgui,
            display,
            mut renderer,
            physical_xywh,
            screen_texture_list,
            mut is_drawing_rect,
            mut start_point,
            mut end_point,
            mut curr_point,
        } = self;
        // 存储选择结果
        let result: Rc<RefCell<SelectedArea>> = Rc::new(RefCell::new(SelectedArea::empty()));

        // 设置窗口背景色黑色
        imgui.style_mut().colors[StyleColor::WindowBg as usize] = [0.0, 0.0, 0.0, 1.0];

        let exit_code = event_loop.run_return({
            // 用于帧同步
            let mut last_frame = Instant::now();

            // 用于保存结果
            let select_area = result.clone();

            move |event, _, control_flow| match event {
                // region 和窗口事件相关的逻辑 (在此处更新 imgui 内部时间系统)
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                // endregion
                // region 主事件队列被清空 ==> 通知绘制 ui
                Event::MainEventsCleared => {
                    let gl_window = display.gl_window();
                    platform
                        .prepare_frame(imgui.io_mut(), gl_window.window())
                        .expect("Failed to prepare frame");
                    gl_window.window().request_redraw();
                }
                // endregion
                // region 绘制 ui
                Event::RedrawRequested(_) => {
                    // 开启新的一帧
                    let ui = imgui.new_frame();
                    let (x, y, w, h) = physical_xywh;

                    // region 绘制屏幕图像
                    // 在目标位置绘制各屏幕图像
                    ui.window("screen_images")
                        .position([x as f32, y as f32], imgui::Condition::Always)
                        .size([w as f32, h as f32], imgui::Condition::Always)
                        .title_bar(false)
                        .resizable(false)
                        // .draw_background(false)
                        .build(|| {
                            let draw_list = ui.get_window_draw_list();

                            for screen_texture in &screen_texture_list {
                                let (tid, sx, sy, sw, sh) = screen_texture;
                                draw_list
                                    .add_image(
                                        tid.to_owned(),
                                        [*sx as f32, *sy as f32],
                                        [*sx as f32 + *sw as f32, *sy as f32 + *sh as f32],
                                    )
                                    .build();
                            }
                        });
                    // endregion

                    // region 交互绘制矩形
                    // 终点: 绘制中(当前点) / 绘制结束(结束点)
                    let rect_end = if is_drawing_rect { curr_point } else { end_point };
                    // 透明窗口装填矩形选框交互功能
                    ui.window("bounding_mask")
                        .position([x as f32, y as f32], imgui::Condition::Always)
                        .size([w as f32, h as f32], imgui::Condition::Always)
                        .title_bar(false)
                        .resizable(false)
                        .draw_background(false)
                        .build(|| {
                            let draw_list = ui.get_window_draw_list();

                            // 有选区: 绘制选区外蒙层
                            if start_point.is_some() && rect_end.is_some() {
                                for rect in calc_bounding_rect([w as f32, h as f32], start_point.unwrap(), rect_end.unwrap()) {
                                    draw_list
                                        .add_rect(rect[0], rect[1], MASK_COLOR)
                                        .filled(true)
                                        .build();
                                }
                            }
                            // 无选区: 绘制全屏蒙层
                            else {
                                draw_list
                                    .add_rect([0.0, 0.0], [w as f32, h as f32], MASK_COLOR)
                                    .filled(true)
                                    .build();
                            }
                        });
                    // endregion

                    let mut frame = display.draw();
                    frame.clear_color_srgb(1.0, 1.0, 1.0, 0.0);
                    platform.prepare_render(ui, display.gl_window().window());
                    renderer
                        .render(&mut frame, imgui.render())
                        .expect("Rendering failed");
                    frame.finish().expect("Failed to swap buffers");

                    // 缓存捕获的区域
                    if start_point.is_some() && end_point.is_some() {
                        if start_point.unwrap() == end_point.unwrap() {
                            println!("Ignore! (cause only one pixel is selected)");
                        } else {
                            let p1p2 = calc_select_area(start_point.unwrap(), end_point.unwrap());

                            // 更新目标区域
                            if select_area.borrow().check(p1p2) {
                                let [x1, y1, x2, y2] = p1p2;
                                let (_, pixel_h) = display.get_framebuffer_dimensions();
                                // 获取屏幕帧的 rgba 阵列
                                let frame_pixels: Vec<Vec<(u8, u8, u8, u8)>> = display.read_front_buffer().unwrap();
                                // 选中区域的 rgba 阵列
                                let mut selected_pixels: Vec<Vec<(u8, u8, u8, u8)>> = vec![];
                                for y in (y1 as usize)..(y2 as usize) {
                                    let mut row = vec![(0u8, 0u8, 0u8, 0u8); (x2 as usize) - (x1 as usize)];
                                    // NOTE: read_front_buffer 获取的图像的y方向是反的, 需要反向拿取
                                    row.clone_from_slice(&frame_pixels[pixel_h as usize - y][(x1 as usize)..(x2 as usize)]);
                                    selected_pixels.push(row);
                                }

                                let y = selected_pixels.len();
                                let x = selected_pixels[0].len();

                                // 更新结果
                                select_area.borrow_mut().update(p1p2, selected_pixels);

                                println!("Update Capture! p1p2: {:?}; buffer: {} x {} x 4", p1p2, x, y);
                            }
                        }
                    }
                }
                // endregion
                // region 处理鼠标按下和松开事件
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
                    }
                }
                // endregion
                // region 处理鼠标移动事件
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. }, ..
                } => {
                    // 更新当前点位, 处理边界问题
                    curr_point = Some(calc_constrained_point(position, physical_xywh))
                }
                // endregion
                // region 处理按键事件: 'ESC'
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        }, ..
                    }, ..
                } => {
                    println!("Exit (cause 'ESC' was pressed)");
                    select_area.borrow_mut().clear();
                    *control_flow = ControlFlow::Exit;
                }
                // endregion
                // region 处理按键事件: 'Enter'
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Return),
                            ..
                        }, ..
                    }, ..
                } => {
                    println!("Exit (cause 'Enter' was pressed)");
                    *control_flow = ControlFlow::Exit;
                }
                // endregion
                // region 关闭窗口
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested, ..
                } => {
                    println!("Exit (cause 'WindowEvent::CloseRequested' was sent)");
                    *control_flow = ControlFlow::Exit;
                }
                // endregion
                // region 其他事件
                event => {
                    platform.handle_event(imgui.io_mut(), display.gl_window().window(), &event);
                }
                // endregion
            }
        });

        let v = (exit_code, result.borrow().to_owned());
        v
    }
}