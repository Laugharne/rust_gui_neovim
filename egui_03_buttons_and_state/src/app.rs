use eframe::egui;

pub struct MyApp {
	counter: i32,
}

impl Default for MyApp {
	fn default() -> Self {
	Self {
		counter: 0,
	}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {

			ui.heading("Buttons and State");

			ui.separator();

			ui.label(format!("Counter: {}", self.counter));

			ui.horizontal(|ui| {
				if ui.button("Increment").clicked() {
					self.counter += 1;
				}
				if ui.button("Decrement").clicked() {
					self.counter -= 1;
				}
			});

			ui.separator();

			if ui.button("Reset").clicked() {
				self.counter = 0;
			}
		});
	}
}