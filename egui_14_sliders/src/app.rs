use eframe::egui;

pub struct MyApp {
	brightness: f32,
	contrast  : f32,
	saturation: f32,
	blur      : f32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			brightness: 0.0,
			contrast  : 100.0,
			saturation: 100.0,
			blur      : 0.0,
		}
	}
}

impl MyApp {
	fn preview_color(&self) -> egui::Color32 {
		let base_r = 100.0_f32;
		let base_g = 149.0_f32;
		let base_b = 237.0_f32;

		// Apply brightness
		let r = base_r + self.brightness;
		let g = base_g + self.brightness;
		let b = base_b + self.brightness;

		// Apply contrast (scale from midpoint 128)
		let factor = self.contrast / 100.0;
		let r = 128.0 + (r - 128.0) * factor;
		let g = 128.0 + (g - 128.0) * factor;
		let b = 128.0 + (b - 128.0) * factor;

		// Apply saturation (blend toward grayscale)
		let gray = 0.299 * r + 0.587 * g + 0.114 * b;
		let sat = self.saturation / 100.0;
		let r = gray + (r - gray) * sat;
		let g = gray + (g - gray) * sat;
		let b = gray + (b - gray) * sat;

		// Clamp to 0-255
		let r = r.clamp(0.0, 255.0) as u8;
		let g = g.clamp(0.0, 255.0) as u8;
		let b = b.clamp(0.0, 255.0) as u8;

		egui::Color32::from_rgb(r, g, b)
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut style = (*ctx.style()).clone();
		style.text_styles.insert(
			egui::TextStyle::Body,
			egui::FontId::proportional(18.0),
		);
		style.text_styles.insert(
			egui::TextStyle::Heading,
			egui::FontId::proportional(28.0),
		);
		style.text_styles.insert(
			egui::TextStyle::Button,
			egui::FontId::proportional(18.0),
		);
		style.text_styles.insert(
			egui::TextStyle::Monospace,
			egui::FontId::monospace(18.0),
		);
		ctx.set_style(style);

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.heading("Image Filters");
		});

		egui::SidePanel::left("controls_panel")
			.min_width(250.0)
			.show(ctx, |ui| {
				ui.heading("Controls");
				ui.add_space(10.0);

				ui.label("Brightness:");
				ui.add(
					egui::Slider::new(&mut self.brightness, -100.0..=100.0)
						.suffix(" %"),
				);
				ui.add_space(10.0);

				ui.label("Contrast:");
				ui.add(
					egui::Slider::new(&mut self.contrast, 0.0..=200.0)
						.suffix(" %"),
				);
				ui.add_space(10.0);

				ui.label("Saturation:");
				ui.add(
					egui::Slider::new(&mut self.saturation, 0.0..=200.0)
						.suffix(" %"),
				);
				ui.add_space(10.0);

				ui.label("Blur:");
				ui.add(
					egui::Slider::new(&mut self.blur, 0.0..=20.0)
						.suffix(" px"),
				);
				ui.add_space(20.0);

				ui.vertical_centered(|ui| {
					if ui.button("Reset All").clicked() {
						*self = MyApp::default();
					}
				});
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Preview");
			ui.separator();
			ui.add_space(10.0);

			let color: egui::Color32 = self.preview_color();
			let size: egui::Vec2     = egui::vec2(400.0, 300.0);
			let (rect, _response)    = ui.allocate_exact_size(
				size,
				egui::Sense::hover(),
			);
			ui.painter().rect_filled(
				rect,
				0.0,
				color,
			);

			ui.add_space(10.0);
			ui.label(format!(
				"Brightness: {}% | Contrast: {}% | Saturation: {}% | Blur: {:.1}px",
				self.brightness as i32,
				self.contrast   as i32,
				self.saturation as i32,
				self.blur,
			));
		});
	}
}