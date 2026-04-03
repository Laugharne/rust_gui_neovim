mod app;

use app::MyApp;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: eframe::egui::ViewportBuilder::default()
      .with_inner_size([300.0, 450.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Calculator",
    options,
    Box::new(|_cc| Ok(Box::new(MyApp::default()))),
  )
}