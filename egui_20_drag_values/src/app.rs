use eframe::egui;

pub struct MyApp {
	x       : f64,
	y       : f64,
	rotation: f64,
	scale   : f64,
	color   : [f32; 3],
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			x       : 200.0,
			y       : 150.0,
			rotation: 0.0,
			scale   : 1.0,
			color   : [1.0, 0.5, 0.0],
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::SidePanel::left("controls").min_width(180.0).show(ctx, |ui| {
			ui.heading("Transform");
			ui.separator();

			egui::Grid::new("transform_grid")
				.num_columns(2)
				.spacing([10.0, 8.0])
				.show(ui, |ui| {
					ui.label("X:");
					ui.add(
						egui::DragValue::new(&mut self.x)
							.speed(1.0)
							.range(0.0..=400.0),
					);
					ui.end_row();

					ui.label("Y:");
					ui.add(
						egui::DragValue::new(&mut self.y)
							.speed(1.0)
							.range(0.0..=300.0),
					);
					ui.end_row();

					ui.label("Rotation:");
					ui.add(
						egui::DragValue::new(&mut self.rotation)
							.speed(0.5)
							.range(0.0..=360.0)
							.suffix("°"),
					);
					ui.end_row();

					ui.label("Scale:");
					ui.add(
						egui::DragValue::new(&mut self.scale)
							.speed(0.01)
							.range(0.1..=3.0)
							.prefix("x"),
					);
					ui.end_row();
				});

			ui.separator();
			ui.label("Color:");
			ui.color_edit_button_rgb(&mut self.color);

			ui.separator();
			if ui.button("Reset").clicked() {
				*self = Self::default();
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let size: egui::Vec2 = egui::vec2(440.0, 340.0);
			let (canvas, response) = ui.allocate_exact_size(
				size,
				egui::Sense::click_and_drag(),
			);

			if response.dragged_by(egui::PointerButton::Primary) {
				let delta: egui::Vec2 = response.drag_delta();
				self.x                = (self.x + delta.x as f64).clamp(0.0, 400.0);
				self.y                = (self.y + delta.y as f64).clamp(0.0, 300.0);
			}

			if response.dragged_by(egui::PointerButton::Secondary) {
				let delta: egui::Vec2 = response.drag_delta();
				self.rotation         = (self.rotation + delta.x as f64 * 0.5).clamp(0.0, 360.0);
			}

			if response.hovered() {
				let scroll: f32 = ui.input(|i| i.smooth_scroll_delta.y);
				if scroll != 0.0 {
					self.scale = (self.scale + scroll as f64 * 0.002).clamp(0.1, 3.0);
				}
			}

			let painter = ui.painter();
			painter.rect_filled(
				canvas, 0.0,
				egui::Color32::from_rgb(30, 30, 46),
			);

			let cx: f32            = canvas.min.x + self.x as f32;
			let cy: f32            = canvas.min.y + self.y as f32;
			let center: egui::Pos2 = egui::pos2(cx, cy);

			let half_w: f32 = 40.0 * self.scale as f32;
			let half_h: f32 = 30.0 * self.scale as f32;
			let angle: f32  = self.rotation as f32 * std::f32::consts::PI / 180.0;

			let corners: [[f32; 2]; 4] = [
				[-half_w, -half_h],
				[half_w, -half_h],
				[half_w, half_h],
				[-half_w, half_h],
			];

			let rotated: Vec<egui::Pos2> = corners
				.iter()
				.map(|[dx, dy]| {
					let rx = dx * angle.cos() - dy * angle.sin();
					let ry = dx * angle.sin() + dy * angle.cos();
					egui::pos2(cx + rx, cy + ry)
				})
				.collect();

			let rgba: egui::Rgba = egui::Rgba::from_rgb(
				self.color[0], self.color[1], self.color[2],
			);
			let color: egui::Color32 = egui::Color32::from(rgba);

			painter.add(egui::Shape::convex_polygon(
				rotated.clone(),
				color,
				egui::Stroke::NONE,
			));

			painter.add(egui::Shape::closed_line(
				rotated,
				egui::Stroke::new(2.0, egui::Color32::WHITE),
			));

			painter.circle_stroke(
				center, 3.0,
				egui::Stroke::new(1.0, egui::Color32::WHITE),
			);
		});
	}
}