// use piston_window::{clear, PistonWindow, rectangle, WindowSettings};
//
// pub fn get_select_area() {
//     let mut window = WindowSettings::new("截图", [640, 800])
//         // 无边框
//         .decorated(false)
//         // 全屏
//         .fullscreen(true)
//         // 透明
//         // .transparent(true)
//         // esc 退出
//         .exit_on_esc(true)
//         // 创建实例
//         .build::<PistonWindow>().unwrap();
//
//     while let Some(event) = window.next() {
//         window.draw_2d(&event, |context, graphics, _device| {
//             clear([1.0; 4], graphics);
//             rectangle([1.0, 0.0, 0.0, 1.0], // red
//                       [0.0, 0.0, 100.0, 100.0],
//                       context.transform,
//                       graphics);
//         });
//     };
// }