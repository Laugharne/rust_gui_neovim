mod app;

use app::MyApp;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: eframe::egui::ViewportBuilder::default()
      .with_inner_size([700.0, 500.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Settings Dashboard",
    options,
    Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
  )
}