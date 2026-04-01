use eframe::egui;

pub struct MyApp {
	current_color: [f32; 3],
	saved_colors: Vec<[f32; 3]>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			current_color: [0.8, 0.4, 0.1],
			saved_colors: vec![
				[1.0, 0.0, 0.0],
				[0.0, 0.6, 0.0],
				[0.2, 0.4, 0.9],
			],
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.heading("Color Palette");
		});

		egui::SidePanel::left("picker").min_width(250.0).show(ctx, |ui| {
			ui.heading("Pick a Color");
			ui.add_space(10.0);

			ui.color_edit_button_rgb(&mut self.current_color);

			ui.add_space(10.0);

			let display = egui::Color32::from(egui::Rgba::from_rgb(
				self.current_color[0], self.current_color[1], self.current_color[2],
			));
			ui.label(format!(
				"R: {}  G: {}  B: {}",
				display.r(), display.g(), display.b(),
			));

			ui.add_space(10.0);

			if ui.button("Save Color").clicked() {
				self.saved_colors.push(self.current_color);
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Saved Colors");
			ui.separator();
			ui.add_space(10.0);

			let mut to_remove: Option<usize> = None;

			for (i, color) in self.saved_colors.iter().enumerate() {
				ui.horizontal(|ui| {
					let c = egui::Color32::from(egui::Rgba::from_rgb(
						color[0], color[1], color[2],
					));

					let (rect, _response) = ui.allocate_exact_size(
						egui::vec2(40.0, 20.0),
						egui::Sense::hover(),
					);
					ui.painter().rect_filled(rect, 4.0, c);

					ui.label(format!(
						"#{:02X}{:02X}{:02X}",
						c.r(), c.g(), c.b(),
					));

					if ui.button("Remove").clicked() {
						to_remove = Some(i);
					}
				});
			}

			if let Some(i) = to_remove {
				self.saved_colors.remove(i);
			}
		});
	}
}