#[macro_use]
extern crate napi_derive;

mod declares;
mod screenshots_impl;

#[napi]
pub fn package_name() -> String {
    "screen_capture".to_string()
}