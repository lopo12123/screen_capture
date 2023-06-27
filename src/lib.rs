#[macro_use]
extern crate napi_derive;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

pub mod ffi_use;

// #[napi]
// pub fn get_screens() -> napi::Result<Option<Vec<ScreenInfoFfi>>> {
//     Ok(
//         screenshots::Screen::all()
//             .map_or(None, |ss| {
//                 let mut screens = vec![];
//
//                 for s in ss {
//                     let d = s.display_info;
//                     screens.push(ScreenInfoFfi {
//                         is_primary: d.is_primary,
//                         screen_id: d.id,
//                         screen_num: -1,
//                         scale_factor: d.scale_factor as f64,
//                         xywh_real: vec![d.x, d.y, d.width as i32, d.height as i32],
//                     })
//                 }
//
//                 Some(screens)
//             })
//     )
// }

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}