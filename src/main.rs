mod winit_impl;
mod piston_impl;
mod wgpu_impl;
mod fltk_impl;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    // selection::get_select_area();
    // piston_impl::get_select_area();
    fltk_impl::get_select_area(WIDTH, HEIGHT);
}