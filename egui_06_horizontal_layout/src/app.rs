use eframe::egui;

pub struct MyApp {
	message: String,
	count  : i32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			message: String::from("Ready"),
			count  : 0,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Action Toolbar");
			ui.separator();

			ui.horizontal(|ui| {

				if ui.button("Add").clicked() {
					self.count += 1;
					self.message = format!("Added! Count: {}", self.count);
				}

				if ui.button("Remove").clicked() {
					self.count -= 1;
					self.message = format!("Removed! Count: {}", self.count);
				}

				if ui.button("Reset").clicked() {
					self.count = 0;
					self.message = String::from("Reset!");
				}
			});

			ui.separator();

			ui.horizontal(|ui| {
				ui.label("Status:");
				ui.colored_label(egui::Color32::GREEN, &self.message);
				//ui.label(&self.message);
			});

			ui.horizontal(|ui| {
				ui.label("Count:");
				ui.monospace(self.count.to_string());
				//ui.label(format!("{}", self.count));
			});
		});
	}
}