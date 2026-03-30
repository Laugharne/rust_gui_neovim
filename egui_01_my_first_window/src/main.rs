use eframe::egui;


struct MyApp {
	name: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name: "world".to_string(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, | ui| {
			ui.heading("Hello, egui!");
			ui.label("Welcom to your 1st egui window.");
			ui.label(format!("Greetings, {}", self.name));
		});
	}
}

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([400.0,300.0]),
		..Default::default()
	};

	eframe::run_native(
		"My First Window",
		options,
		Box::new(|_cc| Ok(Box::new(MyApp::default()))),
	)
}
