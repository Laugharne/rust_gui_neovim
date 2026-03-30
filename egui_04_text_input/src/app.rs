use eframe::egui;

pub struct MyApp {
	name: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name: String::new(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {

			ui.heading("Greeting Form");
			ui.separator();
			ui.horizontal(|ui| {
				ui.label("Your name:");
				ui.text_edit_singleline(&mut self.name);
			});

			ui.separator();
			if !self.name.is_empty() {
				ui.label(format!("Hello, {}!", self.name));
			}
		});
	}
}