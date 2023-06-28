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
use fltk::button::Button;
use fltk::enums::{Key};
use fltk::group::{Pack, PackType};
use fltk::image::SvgImage;
use crate::declares::{ScreenInfo};
use crate::utils::{calc_boundary_constraints, get_position_of_buttons, get_real_coord_of_event, get_real_wh_before_scale};

const SVG_CANCEL: &str = r##"<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" stroke="red" stroke-width="1.5" stroke-linecap="round"><path d="M5.26904 5.39746L18.4684 18.5968"/><path d="M18.7307 5.39746L5.39738 18.7308"/></svg>"##;
const SVG_CONFIRM: &str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" fill="green" width="200" height="200"><path d="M892.064 261.888a31.936 31.936 0 0 0-45.216 1.472L421.664 717.248l-220.448-185.216a32 32 0 1 0-41.152 48.992l243.648 204.704a31.872 31.872 0 0 0 20.576 7.488 31.808 31.808 0 0 0 23.36-10.112L893.536 307.136a32 32 0 0 0-1.472-45.248z"/></svg>"##;

// 取消、确认 按钮
fn create_button_pair<F1, F2>(on_cancel: F1, on_confirm: F2) -> Pack
    where F1: FnMut(&mut Button) + 'static,
          F2: FnMut(&mut Button) + 'static,
{
    let mut cancel = SvgImage::from_data(SVG_CANCEL).unwrap();
    let mut confirm = SvgImage::from_data(SVG_CONFIRM).unwrap();
    cancel.scale(30, 30, true, true);
    confirm.scale(30, 30, true, true);

    let mut btn_cancel = Button::new(0, 0, 30, 30, None);
    let mut btn_confirm = Button::new(30, 0, 30, 30, None);
    btn_cancel.set_frame(FrameType::FlatBox);
    btn_confirm.set_frame(FrameType::FlatBox);
    btn_cancel.visible_focus(false);
    btn_confirm.visible_focus(false);

    btn_cancel.set_color(Color::White);
    btn_confirm.set_color(Color::White);
    btn_cancel.set_image(cancel.into());
    btn_confirm.set_image(confirm.into());

    btn_cancel.set_callback(on_cancel);
    btn_confirm.set_callback(on_confirm);

    let mut pack = Pack::new(-100, -100, 70, 30, None)
        .with_type(PackType::Horizontal);
    pack.visible_focus(false);
    pack.add(&btn_cancel);
    pack.add(&btn_confirm);

    pack
}

/// 当前活跃的窗口序号
///
/// - -2: 初始状态
/// - -1: 无活跃窗口
/// - >= 0: 活跃窗口编号
static mut ACTIVE_SCREEN_NUM: i32 = -2;

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

/// 窗口预制件
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
        let (w, h) = get_real_wh_before_scale(screen.scale_factor, (w, h));

        // region 窗口设置
        // region 窗口
        // - 设置位置、大小、标题
        let mut win = Window::new(x, y, w, h, "截图");
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
        // endregion

        // region 画布
        let mut canvas = Frame::default()
            .with_pos(0, 0)
            .with_size(w, h);
        // - 设置风格
        canvas.set_frame(FrameType::FlatBox);
        // - 背景色
        canvas.set_color(config.canvas_background_color);
        // endregion

        // region 按钮组
        let on_cancel = Box::new({
            let _start = start.clone();
            let _end = end.clone();
            move |_: &mut Button| {
                println!("Quit. (cause 'cancel_button' is clicked)");

                // 清空缓存的 bounding box 信息
                *_start.borrow_mut() = None;
                *_end.borrow_mut() = None;
                quit();
            }
        });
        let on_confirm = Box::new(|_: &mut Button| {
            println!("Quit. (cause 'confirm_button' is clicked)");

            quit();
        });
        // 使用 Box 包裹以使闭包拥有 'static 生命周期
        let mut pack = create_button_pair(
            on_cancel,
            on_confirm,
        );
        win.add(&pack);
        // endregion

        println!("Initialize window {{{}}} on screen {{{}}} (id: {}) with xywh: ({x}, {y}, {w}, {h}), scale_factor: {}", screen.screen_num, screen.screen_num, screen.screen_id, screen.scale_factor);
        println!("========== ========= ========== ========= ========== =========");

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
            let screen_size = (screen.xywh_real.2, screen.xywh_real.3);

            let offs = offs.clone();
            let start = start.clone();
            let end = end.clone();

            let mut start_ev = (0, 0);
            let mut start_logic = (0, 0);

            move |f, ev| {
                let offs = offs.borrow_mut();
                let sfp = sfp;
                let sft = screen.scale_factor;
                match ev {
                    Event::Push => {
                        // 记录按下位置
                        start_ev = event_coords();
                        start_logic = get_real_coord_of_event(sfp, sft, start_ev);
                        *start.borrow_mut() = Some(start_logic);
                        *end.borrow_mut() = None;

                        println!("Event::Push on screen {{{sn}}} at {start_logic:?}(ev: {:?}, mouse: {:?})", start_ev, get_mouse());
                        true
                    }
                    Event::Released => {
                        // 记录松开位置
                        let end_ev = event_coords();
                        let end_ev_constrained = calc_boundary_constraints(end_ev, screen_size);
                        println!("ev: {end_ev:?}; bound: {end_ev_constrained:?}");
                        let end_logic = get_real_coord_of_event(sfp, sft, end_ev_constrained);
                        *end.borrow_mut() = Some(end_logic);

                        let (pack_x, pack_y) = get_position_of_buttons(start_ev, end_ev_constrained, screen_size);
                        pack.set_pos(pack_x, pack_y);

                        println!("Event::Released on screen {{{sn}}} at {end_logic:?} (ev: {:?}, mouse: {:?})", end_ev, get_mouse());
                        true
                    }
                    Event::Drag => {
                        offs.begin();

                        // 清屏
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        // 获取鼠标当前位置
                        // let curr_logic = event_coords();
                        let curr_logic = get_real_coord_of_event(sfp, sft, event_coords());
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
            let sn = screen.screen_num;
            let offs = offs.clone();
            let canvas = canvas.clone();
            let start = start.clone();
            let end = end.clone();
            move |_, ev| {
                match ev {
                    Event::Focus => {
                        unsafe { ACTIVE_SCREEN_NUM = sn; }

                        true
                    }
                    // 当窗口失去焦点时清除当前窗口的选框
                    Event::Unfocus => {
                        println!("Clear. (cause Event::Unfocus is triggered)");

                        // 清空画布
                        let offs = offs.borrow_mut();
                        offs.begin();
                        draw_rect_fill(0, 0, w, h, config.canvas_background_color);
                        offs.end();
                        canvas.borrow_mut().redraw();

                        // 清空缓存的 bounding box 信息
                        *start.borrow_mut() = None;
                        *end.borrow_mut() = None;

                        unsafe { ACTIVE_SCREEN_NUM = -1; }

                        true
                    }
                    Event::KeyDown => match event_key() {
                        Key::Escape => {
                            println!("Quit. (cause Key::Escape is triggered)");

                            // 清空缓存的 bounding box 信息
                            *start.borrow_mut() = None;
                            *end.borrow_mut() = None;
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

/// 交互式区域框选
pub struct BoxSelectionImpl {
    /// app 实例
    app: App,
    /// 窗口实例
    prefabs: Vec<WindowPrefab>,
}

impl BoxSelectionImpl {
    /// 新建一个实例
    pub fn new(sfp: f32, screens: Vec<ScreenInfo>) -> Self {
        println!("Setup system with sfp: {sfp}, screen_count: {}", screens.len());
        println!("========== ========= ========== ========= ========== =========");

        let mut win_of_screens = vec![];

        for screen in screens {
            win_of_screens.push(WindowPrefab::new(sfp, screen, BoxSelectionConfig::default()));
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
        // 展示全部窗口
        for prefab in &mut self.prefabs {
            prefab.show();
        }
        println!("========== ========= ========== ========= ========== =========");

        // 聚焦到最后一个窗口
        // match self.prefabs.last_mut() {
        //     Some(v) => v.focus(),
        //     None => {}
        // }
        // println!("========== ========= ========== ========= ========== =========");

        // 监听全部子窗口, 全部窗口都失去焦点则直接退出
        while self.app.wait() {
            unsafe {
                if ACTIVE_SCREEN_NUM == -1 {
                    println!("Quit. (cause all windows lost focus, i.e. ACTIVE_SCREEN_NUM = -1)");
                    quit();
                }
            }
        }

        // 获取
        let mut bounding_box: Option<(u32, i32, i32, i32, i32)> = None;
        for prefab in &mut self.prefabs {
            if let Some(v) = prefab.get_bounding_box() {
                bounding_box = Some((prefab.screen.screen_id, v.0, v.1, v.2, v.3))
            }
        }

        println!("Task End. Get area: {:?}", bounding_box);
        println!("========== ========= ========== ========= ========== =========");

        bounding_box
    }
}