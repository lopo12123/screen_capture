use std::thread;
use std::thread::JoinHandle;
use crate::ffi_export::FfiExport;
use crate::fltk_impl::box_selection_impl::{BoxSelectionImpl, get_select_area, WindowPrefab};
use crate::fltk_impl::FltkImpl;
use crate::screenshots_impl::ScreenshotsImpl;

mod fltk_impl;
mod screenshots_impl;
mod ffi_export;
mod declares;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    get_select_area(1);

    // let mut p = BoxSelectionImpl::new();
    // p.start();
}
