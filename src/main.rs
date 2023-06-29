#[macro_use]
extern crate napi_derive;

use crate::egui_impl::demo;

mod declares;
mod screenshots_impl;
mod egui_impl;

fn main() {
    match demo() {
        Ok(_) => {
            println!("done");
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn point_test() {}
}