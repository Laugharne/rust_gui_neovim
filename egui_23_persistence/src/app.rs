use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyApp {
	username     : String,
	dark_mode    : bool,
	font_size    : f32,
	notifications: bool,
	volume       : f32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			username     : String::new(),
			dark_mode    : true,
			font_size    : 16.0,
			notifications: true,
			volume       : 0.8,
		}
	}
}

impl MyApp {
	pub fn new(cc: &eframe::CreationContext) -> Self {
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY)
				.unwrap_or_default();
		}
		Self::default()
	}
}

impl eframe::App for MyApp {
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if self.dark_mode {
			ctx.set_visuals(egui::Visuals::dark());
		} else {
			ctx.set_visuals(egui::Visuals::light());
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Settings");
			ui.separator();

			egui::Grid::new("settings_grid")
				.num_columns(2)
				.spacing([20.0, 10.0])
				.show(ui, |ui| {
					ui.label("Username:");
					ui.text_edit_singleline(&mut self.username);
					ui.end_row();

					ui.label("Dark Mode:");
					ui.checkbox(&mut self.dark_mode, "");
					ui.end_row();

					ui.label("Font Size:");
					ui.add(
						egui::Slider::new(&mut self.font_size, 12.0..=32.0)
							.suffix(" px"),
					);
					ui.end_row();

					ui.label("Notifications:");
					ui.checkbox(&mut self.notifications, "");
					ui.end_row();

					ui.label("Volume:");
					ui.add(
						egui::Slider::new(&mut self.volume, 0.0..=1.0)
							.show_value(true),
					);
					ui.end_row();
				});

			ui.separator();
			ui.label("Settings are saved automatically.");
			ui.label("Close and reopen to verify persistence.");

			ui.separator();
			if ui.button("Reset to Defaults").clicked() {
				*self = Self::default();
			}
		});
	}
}