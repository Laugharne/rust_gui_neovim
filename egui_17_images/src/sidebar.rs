use eframe::egui;

use crate::app::MyApp;

pub fn show(
	app    : &mut MyApp,
	ctx    : &egui::Context,
	names  : &[&str],
	sources: &[egui::ImageSource],
) {
	egui::SidePanel::left("thumbnails")
		.min_width(180.0)
		.show(ctx, |ui| {
			ui.heading("Gallery");
			ui.separator();

			egui::ScrollArea::vertical().show(ui, |ui| {
				for i in 0..names.len() {
					let response: egui::Response = ui.add(
						egui::Image::new(sources[i].clone())
							.max_width(160.0)
							.corner_radius(4.0)
							.sense(egui::Sense::click()),
					);
					if response.clicked() {
						app.selected = i;
					}
					if app.selected == i {
						ui.strong(names[i]);
					} else {
						ui.label(names[i]);
					}
					ui.add_space(8.0);
				}
			});
		});
}