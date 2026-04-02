mod app;
mod preview;
mod sidebar;

fn main() -> eframe::Result {
	let options: eframe::NativeOptions = eframe::NativeOptions {
		viewport: eframe::egui::ViewportBuilder::default()
			.with_inner_size([1024.0, 768.0]),
		..Default::default()
	};
	eframe::run_native(
		"Image Gallery",
		options,
		Box::new(|cc| {
			egui_extras::install_image_loaders(&cc.egui_ctx);
			Ok(Box::new(app::MyApp::default()))
		}),
	)
}