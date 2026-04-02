use eframe::egui;

use crate::app::MyApp;

pub fn show(
	app    : &mut MyApp,
	ctx    : &egui::Context,
	names  : &[&str],
	sources: &[egui::ImageSource],
) {
	egui::TopBottomPanel::top("controls").show(ctx, |ui| {
		ui.horizontal(|ui| {
			ui.heading("Preview");
			ui.separator();
			ui.label("Zoom:");
			ui.add(egui::Slider::new(&mut app.zoom, 0.5..=3.0));
		});
	});

	egui::CentralPanel::default().show(ctx, |ui| {
		egui::ScrollArea::both().show(ui, |ui| {
			let size: egui::Vec2 = egui::vec2(320.0 * app.zoom, 240.0 * app.zoom);
			ui.add(
				egui::Image::new(sources[app.selected].clone())
					.fit_to_exact_size(size)
					.corner_radius(8.0),
			);
			ui.add_space(8.0);
			ui.label(format!(
				"{} — {:.0} × {:.0}",
				names[app.selected], size.x, size.y
			));
		});
	});
}