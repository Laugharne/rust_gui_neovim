use eframe::egui;

pub struct MyApp {
	dark_mode     : bool,
	accent_color  : [f32; 3],
	rounding      : f32,
	spacing       : f32,
	sample_text   : String,
	sample_checked: bool,
	sample_slider : f32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			dark_mode     : true,
			accent_color  : [0.4, 0.6, 1.0],
			rounding      : 4.0,
			spacing       : 8.0,
			sample_text   : "Hello egui!".to_string(),
			sample_checked: true,
			sample_slider : 0.5,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut visuals = if self.dark_mode {
			egui::Visuals::dark()
		} else {
			egui::Visuals::light()
		};

		let accent: egui::Color32 = egui::Color32::from_rgb(
			(self.accent_color[0] * 255.0) as u8,
			(self.accent_color[1] * 255.0) as u8,
			(self.accent_color[2] * 255.0) as u8,
		);
		visuals.selection.bg_fill = accent;
		visuals.widgets.active.bg_fill = accent;

		let mut style: egui::Style       = (*ctx.style()).clone();
		style.visuals                    = visuals;
		style.spacing.item_spacing       = egui::vec2(self.spacing, self.spacing);
		let rounding: egui::CornerRadius = egui::CornerRadius::same(self.rounding as u8);

		style.visuals.widgets.noninteractive.corner_radius = rounding;
		style.visuals.widgets.inactive.corner_radius       = rounding;
		style.visuals.widgets.active.corner_radius         = rounding;
		style.visuals.widgets.hovered.corner_radius        = rounding;
		ctx.set_style(style);

		egui::SidePanel::left("theme_panel")
			.min_width(200.0)
			.show(ctx, |ui| {
				ui.heading("Theme Editor");
				ui.separator();

				ui.checkbox(&mut self.dark_mode, "Dark Mode");
				ui.separator();

				ui.label("Accent Color:");
				ui.color_edit_button_rgb(&mut self.accent_color);
				ui.separator();

				ui.label("Rounding:");
				ui.add(
					egui::Slider::new(&mut self.rounding, 0.0..=20.0)
						.suffix(" px"),
				);

				ui.label("Spacing:");
				ui.add(
					egui::Slider::new(&mut self.spacing, 2.0..=20.0)
						.suffix(" px"),
				);

				ui.separator();
				if ui.button("Reset Theme").clicked() {
					self.dark_mode    = true;
					self.accent_color = [0.4, 0.6, 1.0];
					self.rounding     = 4.0;
					self.spacing      = 8.0;
				}
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Preview");
			ui.separator();

			ui.label("This panel shows your theme in action.");
			ui.separator();

			ui.text_edit_singleline(&mut self.sample_text);
			ui.checkbox(&mut self.sample_checked, "Sample Checkbox");
			ui.add(egui::Slider::new(
				&mut self.sample_slider, 0.0..=1.0,
			));

			ui.separator();
			ui.horizontal(|ui| {
				let _ = ui.button("Primary");
				let _ = ui.button("Secondary");
				let _ = ui.button("Tertiary");
			});

			ui.separator();
			ui.label("Buttons, sliders, and checkboxes all respond to the theme settings on the left.");
		});
	}
}