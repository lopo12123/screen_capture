use eframe::egui;
use eframe::egui::ColorImage;

#[derive(Default)]
pub struct MyApp {
    continuously_take_screenshots: bool,
    texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(screenshot) = self.screenshot.take() {
                self.texture = Some(ui.ctx().load_texture(
                    "screenshot",
                    screenshot,
                    Default::default(),
                ));
            }

            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.continuously_take_screenshots,
                    "continuously take screenshots",
                );

                if ui.button("save to 'top_left.png'").clicked() {
                    frame.request_screenshot();
                }

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    if self.continuously_take_screenshots {
                        if ui
                            .add(egui::Label::new("hover me!").sense(egui::Sense::hover()))
                            .hovered()
                        {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        };
                        frame.request_screenshot();
                    } else if ui.button("take screenshot!").clicked() {
                        frame.request_screenshot();
                    }
                });
            });

            if let Some(texture) = self.texture.as_ref() {
                ui.image(texture, ui.available_size());
            } else {
                ui.spinner();
            }

            ctx.request_repaint();
        });
    }
    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            self.screenshot = Some(screenshot);
        }
    }
}