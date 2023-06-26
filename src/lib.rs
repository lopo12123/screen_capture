#[macro_use]
extern crate napi_derive;

mod fltk_impl;
mod screenshots_impl;
mod screen_capture;
mod declares;

pub mod ffi_use;

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}