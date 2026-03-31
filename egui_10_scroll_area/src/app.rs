use eframe::egui;

pub struct MyApp {
	logs   : Vec<String>,
	next_id: usize,
}

impl Default for MyApp {
  fn default() -> Self {
	Self {
		logs: vec![
			"App started".to_string(),
			"Loading config...".to_string(),
			"Config loaded successfully".to_string(),
			"Connecting to database...".to_string(),
			"Database connected".to_string(),
			"Server listening on port 8080".to_string(),
			"GET /api/users — 200 OK".to_string(),
			"POST /api/login — 200 OK".to_string(),
			"GET /api/dashboard — 200 OK".to_string(),
			"Warning: slow query detected (450ms)".to_string(),
			"GET /api/reports — 200 OK".to_string(),
			"POST /api/data — 201 Created".to_string(),
			"Cache cleared".to_string(),
			"GET /api/users — 200 OK".to_string(),
			"Connection pool: 5/20 active".to_string(),
		],
		next_id: 15,
	}
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx, |ui| {
		ui.heading("Log Viewer");
		ui.separator();

		if ui.button("Add Log Entry").clicked() {
			self.next_id += 1;
			self.logs.push(format!("Event #{}", self.next_id));
		}

		ui.separator();

		egui::ScrollArea::vertical()
			.stick_to_bottom(true)
			.show(ui, |ui| {
				for (i, log) in self.logs.iter().enumerate() {
					ui.label(format!("[{}] {}", i + 1, log));
				}
			});
	});
  }
}