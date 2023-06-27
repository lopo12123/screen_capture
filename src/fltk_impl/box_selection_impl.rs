use fltk::{
    draw::{draw_rect_fill, Offscreen},
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use fltk::app::{App, event_coords, event_key, get_mouse, quit};
use fltk::enums::{Key};
use crate::declares::{ScreenInfo};
use crate::utils::get_real_coord;

/// 绘图的参数配置
pub struct BoxSelectionConfig {
    /// 画布背景色
    canvas_background_color: Color,
    /// 矩形背景颜色
    rect_background_color: Color,
}

impl BoxSelectionConfig {
    pub fn default() -> Self {
        BoxSelectionConfig {
            canvas_background_color: Color::Black,
            rect_background_color: Color::White,
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
    pub fn new(sfp: f32, screen: ScreenInfo, config: BoxSelectionConfig) -> Self {
        println!("Window {{{}}} config: {:?}", screen.screen_num, screen);

        let start = Rc::new(RefCell::new(None));
        let end = Rc::new(RefCell::new(None));

        let (x, y, w, h) = screen.xywh_real;

        // region 窗口
        let mut win = Window::new(x, y, w, h, "截图");
        // - 设置风格
        win.set_frame(FrameType::FlatBox);
        // - 设置位置、大小及所属屏幕
        // win.set_pos(x, y);
        // win.set_size(w, h);
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

        println!("Initialize window {{{}}} on screen {{{}}} with xywh: ({x}, {y}, {w}, {h}), scale_factor: {}", screen.screen_num, screen.screen_num, screen.scale_factor);

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
                let offs = offs.borrow_mut();
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

            let offs = offs.clone();
            let start = start.clone();
            let end = end.clone();

            let mut start_logic: (i32, i32) = (0, 0);

            move |f, ev| {
                let offs = offs.borrow_mut();
                let sfp = sfp;
                let sft = screen.scale_factor;
                match ev {
                    Event::Push => {
                        // 记录按下位置
                        // start_logic = event_coords();
                        start_logic = get_real_coord(sfp, sft, event_coords());
                        *start.borrow_mut() = Some(start_logic);
                        *end.borrow_mut() = None;

                        println!("Event::Push on screen {{{sn}}} at {start_logic:?}(this: coords {:?}, global: {:?})", event_coords(), get_mouse());
                        true
                    }
                    Event::Released => {
                        // 记录松开位置
                        // let end_logic = event_coords();
                        let end_logic = get_real_coord(sfp, sft, event_coords());
                        *end.borrow_mut() = Some(end_logic);

                        println!("Event::Released on screen {{{sn}}} at {end_logic:?} (this: coords {:?}, global: {:?})", event_coords(), get_mouse());
                        true
                    }
                    Event::Drag => {
                        offs.begin();

                        // 清屏
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        // 获取鼠标当前位置
                        // let curr_logic = event_coords();
                        let curr_logic = get_real_coord(sfp, sft, event_coords());
                        // 绘制矩形框
                        let xywh_real: (i32, i32, i32, i32) = (
                            min(start_logic.0, curr_logic.0),
                            min(start_logic.1, curr_logic.1),
                            (curr_logic.0 - start_logic.0).abs(),
                            (curr_logic.1 - start_logic.1).abs(),
                        );
                        draw_rect_fill(
                            xywh_real.0,
                            xywh_real.1,
                            xywh_real.2,
                            xywh_real.3,
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
                        println!("Clear. (cause Event::Unfocus is triggered)");

                        let offs = offs.borrow_mut();
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
        println!("Show window on screen {{{}}} with xywh_real: {:?}", self.screen.screen_num, self.screen.xywh_real);

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
    /// 当前聚焦的屏幕的 scale_factor
    sfp: f32,
    /// app 实例
    app: App,
    /// 窗口实例
    prefabs: Vec<WindowPrefab>,
}

impl BoxSelectionImpl {
    /// 新建一个实例
    pub fn new(sfp: f32, screens: Vec<ScreenInfo>) -> Self {
        println!("Setup system with sfp: {sfp}, screen_count: {}", screens.len());

        let mut win_of_screens = vec![];

        for screen in screens {
            win_of_screens.push(WindowPrefab::new(sfp, screen, BoxSelectionConfig::default()));
        }

        BoxSelectionImpl {
            sfp,
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