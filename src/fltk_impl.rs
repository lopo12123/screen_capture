use fltk::{
    app,
    draw::{
        draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle, Offscreen,
    },
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use fltk::app::get_mouse;
use fltk::draw::{draw_rect, overlay_clear};
use fltk::window::{DoubleWindow, OverlayWindow, WindowType};


pub fn get_select_area(width: i32, height: i32) {
    /// 画布背景色
    let canvas_background_color: Color = Color::from_rgba_tuple((255, 255, 255, 26));
    /// 矩形边框颜色
    let rect_border_color: Color = Color::from_rgba_tuple((255, 255, 255, 80));
    /// 矩形线条粗细
    let rect_border_width: i32 = 2;
    /// 矩形背景颜色
    let rect_background_color: Color = Color::from_rgba_tuple((255, 255, 255, 80));

    // 获取屏幕大小
    // let (w_f64, h_f64) = app::screen_size();
    // let width = w_f64.into();
    // let height = h_f64.into();
    // println!("screen size: {width} x {height}");
    // app
    let app = app::App::default();  // .with_scheme(app::Scheme::Base);

    // 窗口
    // let mut wind = OverlayWindow::default()
    let mut wind: DoubleWindow = Window::default()
        .with_size(width, height)
        .with_label("截图");
    // - 置顶
    // wind.set_on_top();
    // - 无边框
    wind.set_border(false);
    // wind.set_frame(FrameType::NoBox);
    // - 隐藏任务栏

    // 画布
    let mut canvas = Frame::default()
        .with_size(width, height)
        .center_of(&wind);

    canvas.set_color(canvas_background_color);
    canvas.set_frame(FrameType::DownBox);

    wind.end();
    wind.show();
    // - 背景色
    // wind.set_color(Color::from_rgba_tuple((255, 255, 255, 255)));
    // wind.set_opacity(0.5);

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
                offs.rescale();
                offs.copy(0, 0, width, height, 0, 0);
            } else {
                offs.begin();
                draw_rect_fill(0, 0, width, height, Color::White);
                offs.copy(0, 0, width, height, 0, 0);
                offs.end();
            }
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
                    coord = app::event_coords();
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
                    let pointer = app::event_coords();
                    // 绘制矩形框
                    // draw_line(coord.0, coord.1, pointer.0, pointer.1);
                    // draw_rect(coord.0, coord.1, pointer.0)
                    draw_rect(
                        min(coord.0, pointer.0),
                        min(coord.1, pointer.1),
                        (pointer.0 - coord.0).abs(),
                        (pointer.1 - coord.1).abs(),
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

// #
// mod unit_test {
//
// }
