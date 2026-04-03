mod app;

use app::MyApp;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: eframe::egui::ViewportBuilder::default()
      .with_inner_size([800.0, 500.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Habit Tracker",
    options,
    Box::new(|_cc| Ok(Box::new(MyApp::default()))),
  )
}