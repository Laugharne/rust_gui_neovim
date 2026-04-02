use eframe::egui;

pub struct MyApp {
	pub(crate) selected: usize,
	pub(crate) zoom    : f32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			selected: 0,
			zoom    : 1.0,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(1.5);

		let names: [&str; 4] = ["Sunset", "Forest", "Ocean", "Mountain"];
		let sources: [egui::ImageSource<'_>; 4] = [
			egui::include_image!("../assets/sunset.png"),
			egui::include_image!("../assets/forest.png"),
			egui::include_image!("../assets/ocean.png"),
			egui::include_image!("../assets/mountain.png"),
		];

		crate::sidebar::show(self, ctx, &names, &sources);
		crate::preview::show(self, ctx, &names, &sources);
	}
}