// use winit::{
//     dpi::LogicalSize,
//     event::{Event, MouseButton, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     window::WindowBuilder,
// };
//
// pub fn get_select_area() {
//     let event_loop = EventLoop::new();
//     let size = LogicalSize::new(800.0, 600.0);
//     let window = WindowBuilder::new()
//         .with_title("Mouse Selection Rectangle")
//         .with_inner_size(size)
//         .build(&event_loop)
//         .unwrap();
//
//     let mut start_pos = None;
//     let mut end_pos = None;
//     let mut selection_rect = None;
//
//     event_loop.run(move |event, _, control_flow| {
//         match event {
//             Event::WindowEvent { event, .. } => match event {
//                 WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//                 WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => match state {
//                     winit::event::ElementState::Pressed => {
//                         start_pos = window.cursor_position().ok();
//                         end_pos = None;
//                         selection_rect = None;
//                     }
//                     winit::event::ElementState::Released => {
//                         if start_pos.is_some() && end_pos.is_some() {
//                             selection_rect = Some((
//                                 (start_pos.unwrap().x as f32).min(end_pos.unwrap().x as f32),
//                                 (start_pos.unwrap().y as f32).min(end_pos.unwrap().y as f32),
//                                 (start_pos.unwrap().x as f32).max(end_pos.unwrap().x as f32),
//                                 (start_pos.unwrap().y as f32).max(end_pos.unwrap().y as f32),
//                             ));
//                         }
//                     }
//                 },
//                 WindowEvent::CursorMoved { position, .. } => {
//                     if start_pos.is_some() {
//                         end_pos = Some(position);
//                         selection_rect = Some((
//                             (start_pos.unwrap().x as f32).min(end_pos.unwrap().x as f32),
//                             (start_pos.unwrap().y as f32).min(end_pos.unwrap().y as f32),
//                             (start_pos.unwrap().x as f32).max(end_pos.unwrap().x as f32),
//                             (start_pos.unwrap().y as f32).max(end_pos.unwrap().y as f32),
//                         ));
//                     }
//                 }
//                 _ => {}
//             },
//             Event::RedrawRequested(_) => {
//                 let size = window.inner_size();
//                 let dpi_factor = window.scale_factor();
//                 let size = LogicalSize::new(size.width / dpi_factor, size.height / dpi_factor);
//                 let mut frame = window
//                     .draw_frame()
//                     .expect("Failed to get window frame to draw");
//                 if let Some((x1, y1, x2, y2)) = selection_rect {
//                     let rect = winit::window::DpiScaledRect::from_logical(
//                         winit::dpi::LogicalRect::new(
//                             x1, y1, x2 - x1, y2 - y1,
//                         ), dpi_factor,
//                     );
//                     let mut rect_paint = wgpu::RenderPassColorAttachmentDescriptor::clear([0.6, 0.6, 0.6, 0.5].into());
//                     rect_paint.region = wgpu::RenderPassRect {
//                         left: rect.x as u32,
//                         right: rect.x as u32 + rect.width as u32,
//                         bottom: size.height as u32 - rect.y as u32 - rect.height as u32,
//                         top: size.height as u32 - rect.y as u32,
//                         front: 0,
//                         back: 1,
//                     };
//                     frame.add_render_pass(rect_paint);
//                 }
//             }
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     });
// }
