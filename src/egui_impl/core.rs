use eframe::{CreationContext, egui};
use eframe::egui::{Color32, ColorImage, Frame, Image, ImageButton, Key, PointerButton, PointerState, Rect, Ui};
use eframe::glow::Context;
use egui_extras::RetainedImage;
use crate::declares::CaptureInfo;
use crate::egui_impl::assets::{SVG_BUTTON_SIZE, SVG_CANCEL, SVG_CONFIRM};

type ScreenImageInfo = (f32, f32, f32, f32, RetainedImage);

/// 载入按钮的svg转换为 RetainedImage
fn load_button_image() -> [RetainedImage; 2] {
    [
        RetainedImage::from_svg_str("cancel_button", SVG_CANCEL).unwrap(),
        RetainedImage::from_svg_str("confirm_button", SVG_CONFIRM).unwrap(),
    ]
}

// 添加按钮组, 返回是否关闭
fn add_button_group(ctx: &egui::Context, ui: &mut Ui, button_images: &[RetainedImage; 2]) -> bool {
    // 取消
    if ui.add(
        ImageButton::new((&button_images[0]).texture_id(ctx), SVG_BUTTON_SIZE)
            .frame(false)
    ).clicked_by(PointerButton::Primary) {
        println!("close!");

        return true;
    }

    // 确认
    if ui.add(ImageButton::new(
        (&button_images[1]).texture_id(ctx),
        SVG_BUTTON_SIZE)
        .frame(false)
    ).clicked_by(PointerButton::Primary) {
        println!("confirm!");

        return true;
    }

    false
}

/// 载入屏幕的buffer转换为 RetainedImage
fn load_screen_image(screens: &Vec<CaptureInfo>) -> Vec<ScreenImageInfo> {
    let mut images = vec![];

    for screen in screens {
        let screen_image = RetainedImage::from_image_bytes(
            format!("screen_{}", screen.screen_id),
            &screen.buffer,
        ).unwrap();

        images.push((
            screen.physical_x as f32,
            screen.physical_y as f32,
            screen.physical_width as f32 / screen.scale_factor as f32,
            screen.physical_height as f32 / screen.scale_factor as f32,
            screen_image,
        ))
    }

    images
}

// 添加屏幕图像
fn add_screen_image(ctx: &egui::Context, ui: &mut Ui, info: &ScreenImageInfo) {
    let (x, y, w, h, img) = info;
    let rect = Rect::from_min_size(
        egui::Pos2::new(*x, *y),
        egui::Vec2::new(*w, *h),
    );
    ui.put(rect, Image::new(img.texture_id(ctx), [*w, *h]));
}

pub struct MyApp {
    texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,

    screen_images: Vec<ScreenImageInfo>,
    button_images: [RetainedImage; 2],

    is_drawing_rect: bool,
    start_point: Option<[f32; 2]>,
    end_point: Option<[f32; 2]>,
    curr_point: Option<[f32; 2]>,
}

impl MyApp {
    pub fn new(_ctx: &CreationContext, captures: Vec<CaptureInfo>) -> MyApp {
        MyApp {
            texture: None,
            screenshot: None,
            screen_images: load_screen_image(&captures),
            button_images: load_button_image(),
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
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                // ESC 退出
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    println!("Exit (cause 'esc' was pressed)");
                    frame.close();
                    return;
                }

                // 屏幕图像绘制
                for screen_image in &self.screen_images {
                    add_screen_image(ctx, ui, screen_image);
                }

                // let (x, y, w, h, img) = &self.screen_images[0];
                // ui.add(Image::new(img.texture_id(ctx), [*w / 1.5, *h / 1.5]));

                // let (x, y, w, h, img) = &self.screen_images[0];
                // let rect = Rect::from_min_size(
                //     egui::Pos2::new(*x, *y),
                //     egui::Vec2::new(*w, *h),
                // );
                // ui.add(Image::new(img.texture_id(ctx), [*w, *h]));
                // ui.put(rect, Image::new(img.texture_id(ctx), [*w, *h]));
                // ui.put(rect, egui::Label::new("Example label"));

                // 选框绘制
                // todo

                // 按钮组
                if add_button_group(ctx, ui, &self.button_images) {
                    frame.close();
                };

                // if let Some(screenshot) = self.screenshot.take() {
                //     self.texture = Some(ui.ctx().load_texture(
                //         "screenshot",
                //         screenshot,
                //         Default::default(),
                //     ));
                // }
                //
                // if let Some(texture) = self.texture.as_ref() {
                //     ui.image(texture, ui.available_size());
                // } else {
                //     ui.spinner();
                // }

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