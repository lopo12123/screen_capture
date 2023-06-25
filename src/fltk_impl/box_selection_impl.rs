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
use fltk::app::{App, event, event_coords, event_key, event_mouse_button, get_mouse, quit, screen_count, screen_num, screen_scale, screen_size, screen_work_area, screen_xywh};
use fltk::button::Button;
use fltk::draw::{draw_rect, overlay_clear};
use fltk::enums::{Align, Key, Mode};
use fltk::group::Group;
use fltk::window::{DoubleWindow, OverlayWindow, WindowType};
use crate::declares::{ScreenInfo, ScreenInfoFltk};

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
    screen: ScreenInfoFltk,
    win: Window,
    start: Option<(i32, i32)>,
    end: Option<(i32, i32)>,
}

impl WindowPrefab {
    /// 预制窗口
    pub fn new(screen: ScreenInfoFltk, config: BoxSelectionConfig) -> Self {
        let (x, y, w, h) = screen.xywh;

        // region 窗口
        let mut win = Window::default()
            .with_pos(x, y)
            .with_size(w, h)
            .with_label("截图");
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

        win.end();
        // endregion

        // region 离屏渲染
        let offs = Offscreen::new(w, h).unwrap();
        #[cfg(not(target_os = "macos"))]
        {
            offs.begin();
            draw_rect_fill(0, 0, w, h, config.canvas_background_color);
            offs.end();
        }
        let offs = Rc::new(RefCell::new(offs));

        // 设置画布的绘制函数
        canvas.draw({
            let offs = offs.clone();
            let canvas_bg = config.canvas_background_color;
            move |_| {
                let mut offs = offs.borrow_mut();
                if offs.is_valid() {
                    // offs.rescale();
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
            let mut coord: (i32, i32) = (0, 0);
            move |f, ev| {
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
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        // 设置颜色
                        set_draw_color(Color::Red);
                        // 设置粗细
                        set_line_style(LineStyle::Solid, config.rect_border_width);
                        // 获取鼠标当前位置
                        let pointer = event_coords();
                        // 绘制矩形框
                        draw_rect_fill(
                            min(coord.0, pointer.0),
                            min(coord.1, pointer.1),
                            (pointer.0 - coord.0).abs(),
                            (pointer.1 - coord.1).abs(),
                            config.rect_background_color,
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
        // endregion

        println!("Initialize window on screen {{{}}} with parameter xywh: {:?}", screen.screen_num, screen.xywh);

        win.handle(|_, ev| {
            match ev {
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
        });

        WindowPrefab {
            screen,
            win,
            start: None,
            end: None,
        }
    }

    /// 展示窗口
    pub fn show(&mut self) {
        println!("Show window on screen {{{}}} with parameter xywh: {:?}", self.screen.screen_num, self.screen.xywh);

        self.win.show();

        // !以下必须在 `show` 之后调用
        // - 透明
        // self.win.set_color(Color::from_rgba_tuple((255, 255, 255, 0)));
        self.win.set_opacity(0.3);
    }
}

pub struct BoxSelectionImpl {
    /// app 实例
    app: App,
    /// 窗口实例
    prefabs: Vec<WindowPrefab>,
}

impl BoxSelectionImpl {
    /// 启动一个窗口
    pub fn setup_window() {}

    /// 新建一个实例
    pub fn new(screens: Vec<ScreenInfoFltk>) -> Self {
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
    pub fn start(&mut self) {
        for prefab in &mut self.prefabs {
            prefab.show();
        }

        self.app.run().unwrap();
    }

    /// 停止应用
    pub fn stop(&self) {
        App::quit(self.app);
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
    let offs = Rc::new(RefCell::new(offs));

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