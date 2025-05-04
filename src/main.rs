#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod dl;

use dirs;
use dl::b_music;
use eframe::egui;
use tokio;

#[tokio::main]
async fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "This is my life",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    link: String,
    directory: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let default_directory = dirs::audio_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from(""));
        Self {
            link: "".to_string(),
            directory: default_directory,
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
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {});
        egui::Window::new("Music").resizable(false).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let link_label = ui.label("Youtube link: ");
                ui.text_edit_singleline(&mut self.link)
                    .labelled_by(link_label.id);

                let dir_label = ui.label("Directory: ");
                ui.text_edit_singleline(&mut self.directory)
                    .labelled_by(dir_label.id);

                if ui.button("Download").clicked() {
                    let link = self.link.clone();
                    let directory = self.directory.clone();
                    tokio::task::spawn(async move {
                        b_music::download(link, directory).await;
                    });
                }
            });
        });
    }
}
