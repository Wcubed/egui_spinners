use egui::{CentralPanel, Context, Grid, Widget};
use ping::Ping;
use typing::Typing;

mod ping;
mod typing;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Spinners",
        native_options,
        Box::new(|cc| Ok(Box::new(SpinnerApp::new(cc)))),
    )
    .unwrap();
}

#[derive(Default)]
struct SpinnerApp {}

impl SpinnerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|style| style.visuals.striped = true);

        Self::default()
    }
}

impl eframe::App for SpinnerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("Spinner grid").num_columns(2).show(ui, |ui| {
                ui.label("Default");
                ui.spinner();
                ui.end_row();

                ui.label("Ping");
                Ping::new().ui(ui);
                ui.end_row();

                ui.label("Typing");
                Typing::new().ui(ui);
                ui.end_row();
            });
        });
    }
}
