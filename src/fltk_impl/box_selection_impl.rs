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
use std::rc::Rc;
use fltk::app::{App, event_coords, event_key, event_mouse_button, get_mouse, screen_count, screen_num, screen_scale, screen_size, screen_work_area, screen_xywh};
use fltk::button::Button;
use fltk::draw::{draw_rect, overlay_clear};
use fltk::enums::{Align, Key, Mode};
use fltk::group::Group;
use fltk::window::{DoubleWindow, OverlayWindow, WindowType};
use crate::declares::{ScreenInfo, ScreenInfoFltk};

/// 绘图的参数配置
struct BoxSelectionConfig {
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
    win: Window,
    // canvas: Frame,
    // area: (),
}

impl WindowPrefab {
    /// 预制窗口
    pub fn new(w: i32, h: i32) -> Self {
        let mut win: DoubleWindow = Window::default()
            .with_size(w, h)
            .with_label("截图");
        // - 无边框 & 隐藏任务栏
        win.set_border(false);
        // - 置顶
        win.make_modal(true);
        // - 全屏
        win.fullscreen(true);
        // - 设置所属屏幕
        win.set_screen_num(1);

        // win.handle(|w, ev| {
        //     match ev {
        //         Event::KeyDown => match event_key() {
        //             Key::Enter | Key::KPEnter => {
        //                 // TODO 关闭窗口并
        //                 Window::delete(w.to_owned());
        //                 println!("confirmed: {:?}", event_key().to_char());
        //                 true
        //             }
        //             _ => false
        //         },
        //         _ => false,
        //     }
        // });

        WindowPrefab {
            win
        }
    }

    /// 展示窗口
    pub fn show(&mut self) {
        self.win.show();
    }

    /// 销毁窗口
    pub fn close(self) {
        Window::delete(self.win);
    }
}

pub struct BoxSelectionImpl {
    /// 配置
    config: BoxSelectionConfig,
    /// app 实例
    app: App,
    /// 窗口实例
    wins: Vec<WindowPrefab>,
}

impl BoxSelectionImpl {
    /// 启动一个窗口
    pub fn setup_window() {}

    /// 新建一个实例
    pub fn new() -> Self {
        BoxSelectionImpl {
            config: BoxSelectionConfig::default(),
            app: App::default(),
            wins: vec![
                WindowPrefab::new(800, 600)
            ],
        }
    }

    /// 启动窗口进行框选
    pub fn start(&mut self) {
        for win in &mut self.wins {
            win.show();
        }
        self.app.run().unwrap();
    }

    pub fn stop(&self) {
        self.app.quit();
    }
}


pub fn get_select_area(screen_num: i32) {
    /// 获取目标屏幕大小和缩放比
    let scale_factor: f64 = screen_scale(screen_num).into();
    let (_, _, w, h) = screen_xywh(screen_num);
    let width = (w as f64 * scale_factor) as i32;
    let height = (h as f64 * scale_factor) as i32;
    println!("scale_factor: {scale_factor}; screen size: {width} x {height}");

    /// 画布背景色
    let canvas_background_color: Color = Color::White;
    /// 矩形线条粗细
    let rect_border_width: i32 = 2;
    /// 矩形背景颜色
    let rect_background_color: Color = Color::Black;

    // app
    let app = App::default();

    // 窗口
    let mut wind: DoubleWindow = Window::default()
        .with_size(width, height)
        .with_label("截图");
    // - 所属屏幕
    wind.set_screen_num(screen_num);
    wind.set_align(Align::TopLeft);

    println!("screen_num: {}", wind.screen_num());
    // - 无边框 & 隐藏任务栏
    wind.set_border(false);
    // - 置顶
    wind.make_modal(true);
    // - 全屏
    wind.fullscreen(true);

    // let (width, height) = (wind.width(), wind.height());
    // println!("w: {width}; h: {height}");

    // 画布
    let mut canvas = Frame::default()
        .with_size(width, height)
        .center_of(&wind);

    canvas.set_color(canvas_background_color);
    canvas.set_frame(FrameType::DownBox);

    wind.end();
    wind.show();

    // - 透明
    wind.set_color(Color::from_rgba_tuple((255, 255, 255, 0)));
    wind.set_opacity(0.3);

    // 离屏渲染
    let offs = Offscreen::new(canvas.width(), canvas.height()).unwrap();
    #[cfg(not(target_os = "macos"))]
    {
        offs.begin();
        draw_rect_fill(0, 0, width, height, canvas_background_color);
        offs.end();
    }

    let offs = Rc::from(RefCell::from(offs));

    // 设置画布的绘制函数
    canvas.draw({
        let offs = offs.clone();
        move |_| {
            let mut offs = offs.borrow_mut();
            if offs.is_valid() {
                // offs.rescale();
                offs.copy(0, 0, width, height, 0, 0);
            } else {
                offs.begin();
                draw_rect_fill(0, 0, width, height, Color::White);
                offs.copy(0, 0, width, height, 0, 0);
                offs.end();
            }
        }
    });

    wind.handle(|w, ev| {
        match ev {
            Event::KeyDown => match event_key() {
                Key::Enter | Key::KPEnter => {
                    // TODO 关闭窗口并
                    Window::delete(w.to_owned());
                    println!("confirmed: {:?}; {:?}", get_mouse(), event_coords());
                    true
                }
                _ => false
            },
            _ => false,
        }
    });

    canvas.handle({
        let mut coord: (i32, i32) = (0, 0);
        move |f, ev| {
            // println!("{}", ev);
            // println!("coords {:?}", app::event_coords());
            // println!("get mouse {:?}", app::get_mouse());
            let offs = offs.borrow_mut();
            match ev {
                Event::Push => {
                    // 记录按下位置
                    coord = event_coords();

                    println!("Event::Push: {:?}; {:?}", get_mouse(), event_coords());
                    true
                }
                Event::Released => {
                    // 记录松开位置
                    println!("Event::Released: {:?}; {:?}", get_mouse(), event_coords());
                    true
                }
                Event::Drag => {
                    offs.begin();

                    // 清屏
                    draw_rect_fill(0, 0, width, height, canvas_background_color);
                    // 设置颜色
                    set_draw_color(Color::Red);
                    // 设置粗细
                    set_line_style(LineStyle::Solid, rect_border_width);
                    // 获取鼠标当前位置
                    let pointer = event_coords();
                    // 绘制矩形框
                    draw_rect_fill(
                        min(coord.0, pointer.0),
                        min(coord.1, pointer.1),
                        (pointer.0 - coord.0).abs(),
                        (pointer.1 - coord.1).abs(),
                        rect_background_color,
                    );
                    offs.end();

                    // 同步到画布
                    f.redraw();
                    set_line_style(LineStyle::Solid, 0);
                    true
                }
                _ => false,
            }
        }
    });

    app.run().unwrap();
}