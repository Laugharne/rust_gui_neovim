mod app;

use app::MyApp;

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: eframe::egui::ViewportBuilder::default().with_title("PERSISTENCE")
			.with_inner_size([1024.0, 768.0]),
		..Default::default()
	};
	eframe::run_native(
		"Settings App",
		options,
		Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
	)
}