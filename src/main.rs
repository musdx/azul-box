#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui;
use eframe::egui::{self, Button, Color32, RichText, global_theme_preference_buttons};

#[tokio::main]
async fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "azul_box",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    music_download: ui::music_dl::MusicDownload,
    video_download: ui::video_dl::VideoDownload,
    pinterest_download: ui::pinterest::PinterstDownload,
    image_convert: ui::img_convert::ImgConvert,
    video_convert: ui::video_convert::VideoConvert,
    colors: ui::colors::Colors,
    background: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            music_download: ui::music_dl::MusicDownload::default(),
            video_download: ui::video_dl::VideoDownload::default(),
            pinterest_download: ui::pinterest::PinterstDownload::default(),
            image_convert: ui::img_convert::ImgConvert::default(),
            video_convert: ui::video_convert::VideoConvert::default(),
            colors: ui::colors::Colors::default(),
            background: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();

        style
            .text_styles
            .get_mut(&egui::TextStyle::Heading)
            .unwrap()
            .size = 26.0;
        style
            .text_styles
            .get_mut(&egui::TextStyle::Body)
            .unwrap()
            .size = 20.0;
        style
            .text_styles
            .get_mut(&egui::TextStyle::Button)
            .unwrap()
            .size = 20.0;

        ctx.set_style(style);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Azul Box");
                ui.horizontal_wrapped(|ui| {
                    global_theme_preference_buttons(ui);
                    match self.background {
                        false => {
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("Transparent Background")
                                            .color(Color32::LIGHT_BLUE),
                                    )
                                    .fill(Color32::from_rgb(0, 92, 128)),
                                )
                                .clicked()
                            {
                                self.background = true;
                            }
                        }
                        true => {
                            if ui
                                .add(
                                    Button::new(RichText::new("Transparent Background"))
                                        .fill(Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                self.background = false;
                            }
                        }
                    };
                });
            });
        });

        if self.background {
            egui::CentralPanel::default().show(ctx, |ui| ui.label(""));
        }
        //music
        egui::Window::new("Music-dl")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| self.music_download.ui(ui));
        //Video
        egui::Window::new("Video-dl")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.video_download.ui(ui);
            });
        //Pinterest
        egui::Window::new("Pinterest-dl")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.pinterest_download.ui(ui);
            });
        //Img convert
        egui::Window::new("Image converter")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.image_convert.ui(ui);
            });
        //Video convert
        egui::Window::new("Video converter")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.video_convert.ui(ui);
            });
        //Color
        egui::Window::new("Colors picker")
            .default_open(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.colors.ui(ui);
            });
    }
}
