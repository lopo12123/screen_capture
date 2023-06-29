use eframe::egui;
use eframe::egui::{ColorImage, Key, PointerButton, PointerState};
use eframe::glow::Context;
use crate::declares::CaptureInfo;

pub struct MyApp {
    texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,

    is_drawing_rect: bool,
    start_point: Option<[f32; 2]>,
    end_point: Option<[f32; 2]>,
    curr_point: Option<[f32; 2]>,
}

impl MyApp {
    pub fn new(captures: Vec<CaptureInfo>) -> MyApp {
        MyApp {
            texture: None,
            screenshot: None,
            is_drawing_rect: false,
            start_point: None,
            end_point: None,
            curr_point: None,
        }
    }
}

/// 基础 app 生命周期
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // ctx.is_using_pointer()

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                // ESC 退出
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    println!("Exit (cause 'esc' was pressed)");
                    frame.close();
                    return;
                }


                let response = ui.allocate_response(egui::vec2(100.0, 200.0), egui::Sense::click());
                if response.dragged_by(PointerButton::Primary) {
                    println!("clicked!");
                }
                ui.painter().rect_stroke(response.rect, 0.0, (1.0, egui::Color32::WHITE));

                // match ctx.input(|i| i.pointer) {
                //     PointerState { down: [true, ..], .. } => {
                //         self.is_drawing_rect = true;
                //         // self.curr_point =
                //         println!("done");
                //     }
                //     _ => {}
                // }

                // 右键退出
                // if ctx.input(|i|i.b) { }

                if let Some(screenshot) = self.screenshot.take() {
                    self.texture = Some(ui.ctx().load_texture(
                        "screenshot",
                        screenshot,
                        Default::default(),
                    ));
                }

                if let Some(texture) = self.texture.as_ref() {
                    ui.image(texture, ui.available_size());
                } else {
                    ui.spinner();
                }

                // ctx.request_repaint();
            });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        println!("on exit.");
    }

    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            self.screenshot = Some(screenshot);
        }
    }
}