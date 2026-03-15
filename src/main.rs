mod note;
mod ui;

use std::f32;

use eframe::egui;
use egui::FontId;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "test",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

struct Document {
    title: String,
    content: String,
    path: Option<std::path::PathBuf>,
    font_size: f32,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Untitiled".to_string(),
            content: String::new(),
            path: None,
            font_size: 16.0,
        }
    }
}

#[derive(Default)]
struct MyEguiApp {
    document: Document,
    md_display: bool,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // handle_input(ctx, &mut self.document.content);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&mut self.document.title.clone());
            ui_switch_to_md(ui, &mut self.md_display);
            if self.md_display {
                let mut cache = CommonMarkCache::default();
                CommonMarkViewer::new().show(ui, &mut cache, &self.document.content);
            } else {
                ui.add(
                    egui::TextEdit::multiline(&mut self.document.content)
                        .font(FontId::proportional(self.document.font_size))
                        .desired_width(f32::INFINITY),
                );
            }

            ui_font_size(ui, &mut self.document.font_size);
            ui_open_file(ui, &mut self.document);

            //display_text(ui, &self.text, &self.ui_font_size);
        });
    }
}

fn ui_font_size(ui: &mut egui::Ui, ui_font_size: &mut f32) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *ui_font_size = (*ui_font_size - 1.0).max(6.0);
        }
        ui.label(ui_font_size.to_string());
        if ui.button("+").clicked() {
            *ui_font_size += 1.0;
        }
    });
}

fn ui_switch_to_md(ui: &mut egui::Ui, md_display: &mut bool) {
    let mut s = String::new();
    if *md_display {
        s = "Switch to normal".to_string();
    } else {
        s = "Switch to md".to_string();
    }

    if ui.button(s).clicked() {
        *md_display = !*md_display;
    }
}

fn ui_display_md() {}

fn handle_input(ctx: &egui::Context, text: &mut String) {
    ctx.input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(t) => text.push_str(t),
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    ..
                } => {
                    text.pop();
                }
                egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    ..
                } => std::process::exit(0),
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    ..
                } => {
                    text.push_str("\n");
                }
                _ => {}
            }
        }
    });
}

fn open_file(document: &mut Document) {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            document.content = content;
            document.title = path_to_title(&path);
            document.path = Some(path);
        }
    }
}

fn path_to_title(path: &std::path::PathBuf) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Untiteld")
        .to_string()
}

fn ui_open_file(ui: &mut egui::Ui, document: &mut Document) {
    if ui.button("Open file").clicked() {
        open_file(document);
    }
}
