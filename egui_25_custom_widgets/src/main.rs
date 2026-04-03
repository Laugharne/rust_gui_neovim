mod app;
mod toggle;

use app::MyApp;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: eframe::egui::ViewportBuilder::default()
      .with_inner_size([1024.0, 768.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Widget Showcase",
    options,
    Box::new(|_cc| Ok(Box::new(MyApp::default()))),
  )
}