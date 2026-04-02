use eframe::egui;

pub struct MyApp {
	count    : u32,
	name     : String,
	dark_mode: bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			count    : 0,
			name     : String::new(),
			dark_mode: true,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(2.0);

		egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Tooltip Demo");
				ui.separator();

				// Simple text tooltip
				if ui.button("Count").on_hover_text("Click to increment the counter").clicked() {
					self.count += 1;
				}

				// Rich tooltip with custom UI
				if ui
					.button("Reset")
					.on_hover_ui(|ui| {
						ui.heading("Reset");
						ui.label("Sets the counter back to zero.");
						ui.label("Also clears the name field.");
					})
					.clicked()
				{
					self.count = 0;
					self.name.clear();
				}

				// Toggle with tooltip
				let theme_label: &str = if self.dark_mode { "Dark" } else { "Light" };
				if ui
					.button(theme_label)
					.on_hover_text("Toggle between dark and light mode")
					.clicked()
				{
					self.dark_mode = !self.dark_mode;
				}
			});
		});

		if self.dark_mode {
			ctx.set_visuals(egui::Visuals::dark());
		} else {
			ctx.set_visuals(egui::Visuals::light());
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.add_space(10.0);

			// Label with tooltip
			ui.label(
				egui::RichText::new(format!("Count: {}", self.count))
					.size(28.0),
			)
			.on_hover_text("This value increases each time you click Count");

			ui.add_space(10.0);
			ui.separator();
			ui.add_space(10.0);

			// Text input with tooltip
			ui.horizontal(|ui| {
				ui.label("Name:");
				ui
					.text_edit_singleline(&mut self.name)
					.on_hover_text("Type your name here");
			});

			ui.add_space(10.0);
			ui.separator();
			ui.add_space(10.0);

			// Rich tooltip at pointer position
			ui.heading("Hover Examples");
			ui.add_space(5.0);

			ui.label("Hover over each item below:")
				.on_hover_ui_at_pointer(|ui| {
					ui.label("This tooltip follows your cursor!");
				});

			ui.add_space(5.0);

			ui.horizontal(|ui| {
				ui.label("Status:")
					.on_hover_text("Shows the current app state");

				let status: egui::RichText = if self.name.is_empty() {
					egui::RichText::new("No name entered")
						.color(egui::Color32::from_rgb(255, 150, 50))
				} else {
					egui::RichText::new(format!("Hello, {}!", self.name))
						.color(egui::Color32::from_rgb(100, 200, 100))
				};

				ui.label(status)
					.on_hover_ui(|ui| {
						ui.strong("Status Info");
						ui.label(format!("Name length: {} characters", self.name.len()));
						ui.label(format!("Counter value: {}", self.count));
					});
			});
		});
	}
}