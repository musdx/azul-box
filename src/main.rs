#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui;

use std::path::PathBuf;
use ui::shares::yt_dlp_bin;
use crate::ui::shares::config::config_file_default;
use eframe::egui::{self, IconData, RichText, global_theme_preference_buttons};

#[tokio::main]
async fn main() -> eframe::Result {
    let icon = include_bytes!("../assets/logo.png").to_vec();
    let icon = IconData {
        rgba: icon,
        width: 32,
        height: 32,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "azul_box",
        options,
        Box::new(|_cc| {
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MainApp>::default())
        }),
    )
}

struct MainApp {
    music_download: ui::music_dl::MusicDownload,
    video_download: ui::video_dl::VideoDownload,
    pinterest_download: ui::pinterest::PinterstDownload,
    image_convert: ui::img_convert::ImgConvert,
    video_convert: ui::video_convert::VideoConvert,
    run_on_start: bool,
    yt: bool,
    ffmpeg: bool,
    pin: bool,
    azul_yt: PathBuf,
    check_result: i8,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            music_download: ui::music_dl::MusicDownload::default(),
            video_download: ui::video_dl::VideoDownload::default(),
            pinterest_download: ui::pinterest::PinterstDownload::default(),
            image_convert: ui::img_convert::ImgConvert::default(),
            video_convert: ui::video_convert::VideoConvert::default(),
            run_on_start: false,
            yt: true,
            ffmpeg: false,
            pin: false,
            azul_yt: PathBuf::new(),
            check_result: 0,
        }
    }
}

use crate::ui::shares::version_check;
use eframe::egui::Align2;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        if !self.run_on_start {
            // config_file();
            if let Some(bin_path) = yt_dlp_bin::ytdlp_cache() {
                self.azul_yt = bin_path;
            }
            config_file_default();
            self.check_result = version_check::version_check();
            self.run_on_start = true;
        };

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
                    let mut toasts = Toasts::new()
                        .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
                        .direction(egui::Direction::BottomUp);
                    if ui.button("Check For new version").clicked() {
                        if self.check_result > 0 {
                            println!("{}", self.check_result);
                            toasts.add(Toast {
                                text:
                                    "Your version is higher than github release. It must feel nice!"
                                        .into(),
                                kind: ToastKind::Success,
                                options: ToastOptions::default()
                                    .duration_in_seconds(10.0)
                                    .show_progress(true),
                                ..Default::default()
                            });
                        } else if self.check_result == 0 {
                            println!("{}", self.check_result);
                            toasts.add(Toast {
                                text: "You are on the lastest release".into(),
                                kind: ToastKind::Success,
                                options: ToastOptions::default()
                                    .duration_in_seconds(10.0)
                                    .show_progress(true),
                                ..Default::default()
                            });
                        } else {
                            println!("{}", self.check_result);
                            toasts.add(Toast {
                                text: "Your version is out of date".into(),
                                kind: ToastKind::Warning,
                                options: ToastOptions::default()
                                    .duration_in_seconds(10.0)
                                    .show_progress(true),
                                ..Default::default()
                            });
                        }
                    }
                    toasts.show(ctx);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| ui.label(""));
        egui::SidePanel::left("Panel")
            .resizable(true)
            .width_range(27.0..=90.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("Yt-dlp:").size(17.0));
                ui.add(egui::Checkbox::without_text(&mut self.yt));
                ui.separator();
                ui.label(RichText::new("Pinterest:").size(17.0));
                ui.add(egui::Checkbox::without_text(&mut self.pin));
                ui.separator();
                ui.label(RichText::new("Ffmpeg:").size(17.0));
                ui.add(egui::Checkbox::without_text(&mut self.ffmpeg));
                ui.separator();
            });
        if self.yt {
            //music
            egui::Window::new("Music-dl")
                .default_open(false)
                .resizable(false)
                .show(ctx, |ui| self.music_download.ui(ui, &self.azul_yt));
            //Video
            egui::Window::new("Video-dl")
                .default_open(false)
                .resizable(false)
                .show(ctx, |ui| {
                    self.video_download.ui(ui, &self.azul_yt);
                });
        }
        if self.pin {
            //Pinterest
            egui::Window::new("Pinterest-dl")
                .default_open(false)
                .resizable(false)
                .show(ctx, |ui| {
                    self.pinterest_download.ui(ui);
                });
        }
        if self.ffmpeg {
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
        }
    }
}
