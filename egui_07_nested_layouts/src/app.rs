use eframe::egui;

pub struct MyApp {
	selected: String,
	count   : i32,
}

impl Default for MyApp {
  fn default() -> Self {
	Self {
		selected: String::from("Overview"),
		count   : 0,
	}
  }
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Dashboard");
			ui.separator();

			ui.horizontal(|ui| {
				ui.vertical(|ui| {

					ui.set_max_width(150.0);
					ui.label("Navigation");
					ui.separator();
					if ui.button("Overview").clicked() {
						self.selected = String::from("Overview");
					}
					if ui.button("Stats").clicked() {
						self.selected = String::from("Stats");
					}
					if ui.button("Settings").clicked() {
						self.selected = String::from("Settings");
					}
				});

				ui.separator();

				ui.vertical(|ui| {

					ui.label("Details");
					ui.separator();

					ui.horizontal(|ui| {
						ui.label("Page:");
						ui.label(&self.selected);
					});

					ui.horizontal(|ui| {
						ui.label("Visits:");
						ui.label(format!("{}", self.count));
					});

					if ui.button("Visit").clicked() {
						self.count += 1;
					}
				});
			});
		});
	}
}