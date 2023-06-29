#[macro_use]
extern crate napi_derive;

mod screenshots_impl;
mod screen_capture;
mod declares;
mod utils;

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}