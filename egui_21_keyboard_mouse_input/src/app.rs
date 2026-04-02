use eframe::egui;

const COLORS: [(egui::Color32, &str); 4] = [
	(egui::Color32::from_rgb(255, 154, 86), "orange"),
	(egui::Color32::from_rgb(100, 180, 255), "blue"),
	(egui::Color32::from_rgb(166, 227, 161), "green"),
	(egui::Color32::from_rgb(243, 139, 168), "pink"),
];

pub struct MyApp {
	x          : f32,
	y          : f32,
	color_index: usize,
	speed      : f32,
	markers    : Vec<(egui::Pos2, egui::Color32)>,
	log        : Vec<String>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			x          : 200.0,
			y          : 150.0,
			color_index: 0,
			speed      : 3.0,
			markers    : Vec::new(),
			log        : Vec::new(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.request_repaint();

		ctx.input(|i| {
			if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
				self.y -= self.speed;
				self.log.push("Move up".to_string());
			}
			if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
				self.y += self.speed;
				self.log.push("Move down".to_string());
			}
			if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
				self.x -= self.speed;
				self.log.push("Move left".to_string());
			}
			if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
				self.x += self.speed;
				self.log.push("Move right".to_string());
			}
			if i.key_pressed(egui::Key::Tab) {
				self.color_index = (self.color_index + 1) % COLORS.len();
				self.log.push(format!("Color: {}", COLORS[self.color_index].1));
			}
			if i.key_pressed(egui::Key::Space) {
				self.x = 200.0;
				self.y = 150.0;
				self.markers.clear();
				self.log.push("Reset".to_string());
			}
		});

		self.x = self.x.clamp(10.0, 430.0);
		self.y = self.y.clamp(10.0, 330.0);

		if self.log.len() > 12 {
			self.log.drain(0..self.log.len() - 12);
		}

		let color = COLORS[self.color_index].0;

		egui::SidePanel::left("controls").min_width(180.0).show(ctx, |ui| {
			ui.heading("Key Explorer");
			ui.separator();

			ui.label("Arrow keys or WASD to move");
			ui.label("Tab to cycle color");
			ui.label("Click canvas to place marker");
			ui.label("Space to reset");
			ui.separator();

			ui.label(format!("Position: ({:.0}, {:.0})", self.x, self.y));
			ui.label(format!("Color: {}", COLORS[self.color_index].1));
			ui.label(format!("Markers: {}", self.markers.len()));

			ui.add(
				egui::DragValue::new(&mut self.speed)
					.speed(0.1)
					.range(1.0..=10.0)
					.prefix("Speed: "),
			);

			ui.separator();
			ui.label("Input Log:");
			for entry in self.log.iter().rev() {
				ui.small(entry);
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let size: egui::Vec2 = egui::vec2(440.0, 340.0);
			let (canvas, response) = ui.allocate_exact_size(
				size,
				egui::Sense::click(),
			);

			if response.clicked() {
				if let Some(pos) = response.interact_pointer_pos() {
					let local: egui::Pos2 = egui::pos2(
						pos.x - canvas.min.x,
						pos.y - canvas.min.y,
					);
					self.markers.push((local, color));
					self.log.push(format!(
						"Click at ({:.0}, {:.0})", local.x, local.y
					));
				}
			}

			let painter = ui.painter();
			painter.rect_filled(
				canvas, 0.0,
				egui::Color32::from_rgb(30, 30, 46),
			);

			for (pos, c) in &self.markers {
				let screen_pos: egui::Pos2 = egui::pos2(
					canvas.min.x + pos.x,
					canvas.min.y + pos.y,
				);
				painter.circle_filled(screen_pos, 5.0, *c);
			}

			let center: egui::Pos2 = egui::pos2(
				canvas.min.x + self.x,
				canvas.min.y + self.y,
			);
			let rect = egui::Rect::from_center_size(
				center, egui::vec2(30.0, 30.0),
			);
			painter.rect_filled(rect, 4.0, color);
			painter.rect_stroke(
				rect, 4.0,
				egui::Stroke::new(2.0, egui::Color32::WHITE),
				egui::StrokeKind::Outside,
			);
		});
	}
}