#[macro_use]
extern crate napi_derive;

mod declares;
mod utils;
mod screenshots_impl;
mod imgui_impl;

pub mod screen_capture;

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}