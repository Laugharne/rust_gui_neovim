mod app;

use app::MyApp;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: eframe::egui::ViewportBuilder::default()
      .with_inner_size([900.0, 600.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Playlist Manager",
    options,
    Box::new(|_cc| Ok(Box::new(MyApp::default()))),
  )
}