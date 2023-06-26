use fltk::{
    app,
    draw::{
        draw_line, draw_point, draw_rect_fill, LineStyle, Offscreen, set_draw_color, set_line_style,
    },
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::cmp::min;
use std::fmt::format;
use std::ops::Mul;
use std::rc::Rc;
use fltk::app::{App, event, event_coords, event_key, event_mouse_button, get_mouse, quit, screen_count, screen_num, screen_scale, screen_size, screen_work_area, screen_xywh};
use fltk::button::Button;
use fltk::draw::{draw_rect, overlay_clear};
use fltk::enums::{Align, Key, Mode};
use fltk::group::Group;
use fltk::window::{DoubleWindow, OverlayWindow, WindowType};
use crate::declares::{ScreenInfo};
use crate::utils::{physical_to_logic, physical_to_logic_xy, physical_to_logic_xywh};

/// 绘图的参数配置
pub struct BoxSelectionConfig {
    /// 画布背景色
    canvas_background_color: Color,
    /// 矩形线条粗细
    rect_border_width: i32,
    /// 矩形背景颜色
    rect_background_color: Color,
}

impl BoxSelectionConfig {
    pub fn default() -> Self {
        BoxSelectionConfig {
            canvas_background_color: Color::White,
            rect_border_width: 2,
            rect_background_color: Color::Black,
        }
    }
}

pub struct WindowPrefab {
    screen: ScreenInfo,
    win: Window,
    start: Rc<RefCell<Option<(i32, i32)>>>,
    end: Rc<RefCell<Option<(i32, i32)>>>,
}

impl WindowPrefab {
    /// 预制窗口
    pub fn new(screen: ScreenInfo, config: BoxSelectionConfig) -> Self {
        let start = Rc::new(RefCell::new(None));
        let end = Rc::new(RefCell::new(None));

        let (x, y, w, h) = screen.xywh_logic;

        // region 窗口
        let mut win = Window::default()
            .with_pos(x, y)
            .with_size(w, h)
            .with_label(&format!("截图_{}", screen.screen_num));
        // - 设置风格
        win.set_frame(FrameType::FlatBox);
        // - 设置所属屏幕
        win.set_screen_num(screen.screen_num);
        // - 无边框 & 隐藏任务栏
        win.set_border(false);
        // - 置顶
        // win.make_modal(true);
        // - 全屏
        // win.fullscreen(true);

        // region 画布
        let mut canvas = Frame::default()
            .with_pos(0, 0)
            .with_size(w, h);
        // - 设置风格
        canvas.set_frame(FrameType::FlatBox);
        // - 背景色
        canvas.set_color(config.canvas_background_color);
        // endregion

        println!("Initialize window on screen {{{}}} with xywh_physical: {:?}, scale_factor: {}", screen.screen_num, screen.xywh_physical, screen.scale_factor);

        win.end();
        // endregion

        // region 离屏渲染与事件交互
        let offs = Offscreen::new(w, h).unwrap();
        #[cfg(not(target_os = "macos"))]
        {
            offs.begin();
            draw_rect_fill(0, 0, w, h, config.canvas_background_color);
            offs.end();
        }

        // block by shadowing
        let offs = Rc::new(RefCell::new(offs));

        // 设置画布的绘制函数
        canvas.draw({
            let offs = offs.clone();
            let canvas_bg = config.canvas_background_color;
            move |_| {
                let mut offs = offs.borrow_mut();
                if offs.is_valid() {
                    // 屏幕有缩放则重新放缩以正确绘制
                    offs.copy(0, 0, w, h, 0, 0);
                } else {
                    offs.begin();
                    draw_rect_fill(0, 0, w, h, canvas_bg);
                    offs.copy(0, 0, w, h, 0, 0);
                    offs.end();
                }
            }
        });

        // 监听画布交互
        canvas.handle({
            let sn = screen.screen_num;
            let sf = screen.scale_factor;

            let offs = offs.clone();
            let start = start.clone();
            let end = end.clone();

            let mut start_logic: (i32, i32) = (0, 0);

            move |f, ev| {
                let offs = offs.borrow_mut();
                match ev {
                    Event::Push => {
                        // 记录按下位置
                        start_logic = physical_to_logic_xy(event_coords(), sf);
                        *start.borrow_mut() = Some(start_logic);
                        *end.borrow_mut() = None;

                        println!("Event::Push on screen {{{}}} with at coords {:?}", sn, start_logic);
                        true
                    }
                    Event::Released => {
                        // 记录松开位置
                        let end_logic = physical_to_logic_xy(event_coords(), sf);
                        *end.borrow_mut() = Some(end_logic);

                        println!("Event::Released on screen {{{}}} at logic coords {:?}", sn, end_logic);
                        true
                    }
                    Event::Drag => {
                        offs.begin();

                        // 清屏
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        // 获取鼠标当前位置
                        let curr_logic = physical_to_logic_xy(event_coords(), sf);
                        // 绘制矩形框
                        let xywh_logic: (i32, i32, i32, i32) = (
                            min(start_logic.0, curr_logic.0),
                            min(start_logic.1, curr_logic.1),
                            (curr_logic.0 - start_logic.0).abs(),
                            (curr_logic.1 - start_logic.1).abs(),
                        );
                        draw_rect_fill(
                            xywh_logic.0,
                            xywh_logic.1,
                            xywh_logic.2,
                            xywh_logic.3,
                            config.rect_background_color,
                        );

                        offs.end();

                        // 同步到画布
                        f.redraw();
                        true
                    }
                    _ => false,
                }
            }
        });

        // block by shadowing
        let canvas = Rc::new(RefCell::new(canvas));

        // 监听窗口交互
        win.handle({
            let offs = offs.clone();
            let canvas = canvas.clone();
            move |_, ev| {
                match ev {
                    // 当窗口失去焦点时清除当前窗口的选框
                    Event::Unfocus => {
                        let mut offs = offs.borrow_mut();
                        offs.begin();
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        offs.end();
                        canvas.borrow_mut().redraw();
                        true
                    }
                    Event::KeyDown => match event_key() {
                        Key::Escape => {
                            println!("Quit. (cause Key::Escape is triggered)");
                            quit();
                            true
                        }
                        Key::Enter | Key::KPEnter => {
                            println!("Quit. (cause Key::Enter | Key::KPEnter is triggered)");
                            quit();
                            true
                        }
                        _ => false
                    },
                    _ => false,
                }
            }
        });
        // endregion

        WindowPrefab { screen, win, start, end }
    }

    /// 展示窗口
    pub fn show(&mut self) {
        println!("Show window on screen {{{}}} with xywh_logic: {:?}", self.screen.screen_num, self.screen.xywh_logic);

        self.win.show();

        // !以下必须在 `show` 之后调用
        // - 透明
        // self.win.set_color(Color::from_rgba_tuple((255, 255, 255, 0)));
        self.win.set_opacity(0.3);
    }

    /// 获取选框
    ///
    /// `(x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn get_bounding_box(&mut self) -> Option<(i32, i32, i32, i32)> {
        let p_start = self.start.borrow();
        let p_end = self.end.borrow();

        if p_start.is_none() || p_end.is_none() {
            None
        } else {
            // block by shadowing
            let p_start = p_start.unwrap();
            let p_end = p_end.unwrap();
            Some((p_start.0, p_start.1, p_end.0, p_end.1))
        }
    }
}

pub struct BoxSelectionImpl {
    /// app 实例
    app: App,
    /// 窗口实例
    prefabs: Vec<WindowPrefab>,
}

impl BoxSelectionImpl {
    /// 新建一个实例
    pub fn new(screens: Vec<ScreenInfo>) -> Self {
        let mut win_of_screens = vec![];

        for screen in screens {
            win_of_screens.push(WindowPrefab::new(screen, BoxSelectionConfig::default()));
        }

        BoxSelectionImpl {
            app: App::default(),
            prefabs: win_of_screens,
        }
    }

    /// 启动窗口进行框选
    ///
    /// `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
    pub fn run(&mut self) -> Option<(u32, i32, i32, i32, i32)> {
        for prefab in &mut self.prefabs {
            prefab.show();
        }

        self.app.run().unwrap();

        let mut bounding_box: Option<(u32, i32, i32, i32, i32)> = None;

        for prefab in &mut self.prefabs {
            if let Some(v) = prefab.get_bounding_box() {
                bounding_box = Some((prefab.screen.screen_id, v.0, v.1, v.2, v.3))
            }
        }

        bounding_box
    }
}


// pub fn get_select_area(screen_num: i32) {
//     /// 获取目标屏幕大小和缩放比
//     let scale_factor: f64 = screen_scale(screen_num).into();
//     let (_, _, w, h) = screen_xywh(screen_num);
//     let width = (w as f64 * scale_factor) as i32;
//     let height = (h as f64 * scale_factor) as i32;
//     println!("scale_factor: {scale_factor}; screen size: {width} x {height}");
//
//     /// 画布背景色
//     let canvas_background_color: Color = Color::White;
//     /// 矩形线条粗细
//     let rect_border_width: i32 = 2;
//     /// 矩形背景颜色
//     let rect_background_color: Color = Color::Black;
//
//     // app
//     let app = App::default();
//
//     // 窗口
//     let mut wind: DoubleWindow = Window::default()
//         .with_size(width, height)
//         .with_label("截图");
//     // - 所属屏幕
//     wind.set_screen_num(screen_num);
//     wind.set_align(Align::TopLeft);
//
//     println!("screen_num: {}", wind.screen_num());
//     // - 无边框 & 隐藏任务栏
//     // wind.set_border(false);
//     // - 置顶
//     wind.make_modal(true);
//     // - 全屏
//     wind.fullscreen(true);
//
//     // let (width, height) = (wind.width(), wind.height());
//     // println!("w: {width}; h: {height}");
//
//     // 画布
//     let mut canvas = Frame::default()
//         .with_size(width, height)
//         .center_of(&wind);
//
//     canvas.set_color(canvas_background_color);
//     canvas.set_frame(FrameType::DownBox);
//
//     wind.end();
//     wind.show();
//
//     // - 透明
//     wind.set_color(Color::from_rgba_tuple((255, 255, 255, 0)));
//     wind.set_opacity(0.3);
//
//     // 离屏渲染
//     let offs = Offscreen::new(canvas.width(), canvas.height()).unwrap();
//     #[cfg(not(target_os = "macos"))]
//     {
//         offs.begin();
//         draw_rect_fill(0, 0, width, height, canvas_background_color);
//         offs.end();
//     }
//     let offs = Rc::new(RefCell::new(offs));
//
//     // 设置画布的绘制函数
//     canvas.draw({
//         let offs = offs.clone();
//         move |_| {
//             let mut offs = offs.borrow_mut();
//             if offs.is_valid() {
//                 // offs.rescale();
//                 offs.copy(0, 0, width, height, 0, 0);
//             } else {
//                 offs.begin();
//                 draw_rect_fill(0, 0, width, height, Color::White);
//                 offs.copy(0, 0, width, height, 0, 0);
//                 offs.end();
//             }
//         }
//     });
//
//     wind.handle(|w, ev| {
//         match ev {
//             Event::KeyDown => match event_key() {
//                 Key::Enter | Key::KPEnter => {
//                     // TODO 关闭窗口并
//                     Window::delete(w.to_owned());
//                     println!("confirmed: {:?}; {:?}", get_mouse(), event_coords());
//                     true
//                 }
//                 _ => false
//             },
//             _ => false,
//         }
//     });
//
//     canvas.handle({
//         let mut coord: (i32, i32) = (0, 0);
//         move |f, ev| {
//             // println!("{}", ev);
//             // println!("coords {:?}", app::event_coords());
//             // println!("get mouse {:?}", app::get_mouse());
//             let offs = offs.borrow_mut();
//             match ev {
//                 Event::Push => {
//                     // 记录按下位置
//                     coord = event_coords();
//
//                     println!("Event::Push: {:?}; {:?}", get_mouse(), event_coords());
//                     true
//                 }
//                 Event::Released => {
//                     // 记录松开位置
//                     println!("Event::Released: {:?}; {:?}", get_mouse(), event_coords());
//                     true
//                 }
//                 Event::Drag => {
//                     offs.begin();
//
//                     // 清屏
//                     draw_rect_fill(0, 0, width, height, canvas_background_color);
//                     // 设置颜色
//                     set_draw_color(Color::Red);
//                     // 设置粗细
//                     set_line_style(LineStyle::Solid, rect_border_width);
//                     // 获取鼠标当前位置
//                     let pointer = event_coords();
//                     // 绘制矩形框
//                     draw_rect_fill(
//                         min(coord.0, pointer.0),
//                         min(coord.1, pointer.1),
//                         (pointer.0 - coord.0).abs(),
//                         (pointer.1 - coord.1).abs(),
//                         rect_background_color,
//                     );
//                     offs.end();
//
//                     // 同步到画布
//                     f.redraw();
//                     set_line_style(LineStyle::Solid, 0);
//                     true
//                 }
//                 _ => false,
//             }
//         }
//     });
//
//     app.run().unwrap();
// }