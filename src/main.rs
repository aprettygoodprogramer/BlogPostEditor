use dotenv::dotenv;
use eframe::egui;
use egui_commonmark::CommonMarkViewer;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;
struct MyApp {
    content: String,
    title: String,
    slug: String,
    runtime: tokio::runtime::Handle,
}

async fn upload_to_database(
    pool: &sqlx::PgPool,
    content: &str,
    title: &str,
    slug: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO posts (title, content, slug)
        VALUES ($1, $2, $3)
        "#,
        title,
        content,
        slug
    )
    .execute(pool)
    .await?;

    Ok(())
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.slug).desired_rows(1));
                        ui.add(egui::TextEdit::singleline(&mut self.title).desired_rows(1));
                        ui.add(egui::TextEdit::multiline(&mut self.content).desired_rows(1));
                    });
                    if ui.add(egui::Button::new("Upload")).clicked() {
                        let content = self.content.clone();
                        let title = self.title.clone();
                        let slug = self.slug.clone();
                        let ctx_clone = ctx.clone();
                        self.runtime.spawn(async move {
                            let pool = sqlx::PgPool::connect(
                                &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                            )
                            .await
                            .expect("Failed to connect to the database");

                            if let Err(e) = upload_to_database(&pool, &content, &title, &slug).await
                            {
                                eprintln!("Failed to upload to database: {}", e);
                            }
                            ctx_clone.request_repaint();
                        });
                    }

                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Markdown Preview:");
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut cache = egui_commonmark::CommonMarkCache::default();
                                CommonMarkViewer::new().show(ui, &mut cache, &self.content);
                            });
                        });
                    });
                });
            });
        });
    }
}

fn main() {
    dotenv::dotenv().ok();
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let runtime_handle = rt.handle().clone();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Markdown Previewer",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                content: String::from("# Hello\n- Markdown\n- Preview"),
                title: String::from("Title"),
                slug: String::from("Slug"),
                runtime: runtime_handle,
            }))
        }),
    );
}
