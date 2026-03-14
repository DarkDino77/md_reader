mod note;
mod ui;

use eframe::egui;
use egui::epaint::textures;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "test",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    counter: i32,
    text: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        handle_input(ctx, &mut self.text);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
            ui_counter(ui, &mut self.counter);
            display_text(ui, &self.text);
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

fn display_text(ui: &mut egui::Ui, text: &String) {
    ui.horizontal(|ui| ui.label(text));
}

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
