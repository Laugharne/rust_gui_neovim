use eframe::egui;

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([800.0, 600.0]),
		..Default::default()
	};

	eframe::run_native(
		"Labels and Headings",
		options,
		Box::new(|_cc| Ok(Box::new(MyApp::default()))),
	)
}

struct MyApp {
	name: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name: "World".to_string(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Labels and Headings");

			ui.separator();

			ui.label("This is a normal label.");
			ui.label("Labels display read-only text.");

			ui.separator();

			ui.heading("About This App");
			ui.label("Headings are larger and bolder than labels.");
			ui.label( format!("Hello, {}!", self.name));

			ui.separator();

			ui.monospace("fn main() -> eframe::Result");
			ui.monospace("	// monospace text looks like code");
		});
	}
}
