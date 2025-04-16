use eframe::egui;
use egui_commonmark::CommonMarkViewer;

struct MyApp {
    my_string: String,
}

fn upload_to_database(input: &str) {
    let processed = input.replace("\n", "\\n").replace("\"", "\\\"");
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::TextEdit::multiline(&mut self.my_string).desired_rows(10));
                    });
                    if ui.add(egui::Button::new("Upload")).clicked() {
                        upload_to_database(&self.my_string);
                    }

                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Markdown Preview:");
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut cache = egui_commonmark::CommonMarkCache::default();
                                CommonMarkViewer::new().show(ui, &mut cache, &self.my_string);
                            });
                        });
                    });
                });
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Markdown Previewer",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                my_string: String::from("# Hello\n- Markdown\n- Preview"),
            }))
        }),
    );
}
