use eframe::egui;
use crate::toggle::Toggle;

pub struct MyApp {
	dark_mode    : bool,
	notifications: bool,
	auto_save    : bool,
	sound        : bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			dark_mode    : true,
			notifications: true,
			auto_save    : false,
			sound        : true,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if self.dark_mode {
			ctx.set_visuals(egui::Visuals::dark());
		} else {
			ctx.set_visuals(egui::Visuals::light());
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Widget Showcase");
			ui.label("Custom Toggle Switch Widget");
			ui.separator();

			egui::Grid::new("toggles")
				.num_columns(2)
				.spacing([20.0, 12.0])
				.show(ui, |ui| {
					ui.add(Toggle::new(&mut self.dark_mode, "Dark Mode"));
					ui.end_row();

					ui.add(Toggle::new(
						&mut self.notifications, "Notifications",
					));
					ui.end_row();

					ui.add(Toggle::new(
						&mut self.auto_save, "Auto Save",
					));
					ui.end_row();

					ui.add(Toggle::new(
						&mut self.sound, "Sound"
					));

					ui.end_row();
				});

			ui.separator();
			ui.label("Status:");
			ui.label(format!(
				"Dark: {} | Notify: {} | Save: {} | Sound: {}",
				self.dark_mode,
				self.notifications,
				self.auto_save,
				self.sound,
			));
		});
	}
}