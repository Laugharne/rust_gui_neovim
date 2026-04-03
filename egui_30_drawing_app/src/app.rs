use eframe::egui;

struct Stroke {
	points: Vec<egui::Pos2>,
	color : egui::Color32,
	width : f32,
}

pub struct MyApp {
	strokes       : Vec<Stroke>,
	current_stroke: Vec<egui::Pos2>,
	color         : [f32; 3],
	width         : f32,
	is_drawing    : bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			strokes       : Vec::new(),
			current_stroke: Vec::new(),
			color         : [1.0, 1.0, 1.0],
			width         : 3.0,
			is_drawing    : false,
		}
	}
}

impl eframe::App for MyApp {
	fn update(
		&mut self,
		ctx   : &egui::Context,
		_frame: &mut eframe::Frame,
	) {
		ctx.set_pixels_per_point(1.5);

		egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Drawing App");
				ui.separator();

				ui.label("Color:");
				ui.color_edit_button_rgb(&mut self.color);
				ui.separator();

				ui.label("Width:");
				ui.add(
					egui::Slider::new(&mut self.width, 1.0..=20.0),
				);
				ui.separator();

				ui.label(format!(
					"{} strokes",
					self.strokes.len()
				));

				if ui
					.add_enabled(
						!self.strokes.is_empty(),
						egui::Button::new("Undo"),
					)
					.clicked() {
						self.strokes.pop();
					}

				if ui
					.add_enabled(
						!self.strokes.is_empty(),
						egui::Button::new("Clear"),
					)
					.clicked() {
						self.strokes.clear();
					}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let (response, painter) = ui.allocate_painter(
				ui.available_size(),
				egui::Sense::click_and_drag(),
			);

			let rect = response.rect;

			// Dark canvas background
			painter.rect_filled(
				rect,
				0.0,
				egui::Color32::from_rgb(20, 20, 30),
			);

			// Handle drawing input
			if response.dragged_by(egui::PointerButton::Primary) {
				if let Some(pos) = response.interact_pointer_pos()
				{
					if rect.contains(pos) {
						if !self.is_drawing {
							self.is_drawing = true;
							self.current_stroke.clear();
						}
						self.current_stroke.push(pos);
					}
				}
				ctx.request_repaint();
			} else if self.is_drawing {
				// Mouse released — save the stroke
				self.is_drawing = false;
				if self.current_stroke.len() >= 2 {
					let color = egui::Color32::from_rgb(
						(self.color[0] * 255.0) as u8,
						(self.color[1] * 255.0) as u8,
						(self.color[2] * 255.0) as u8,
					);
					self.strokes.push(Stroke {
						points: self.current_stroke.clone(),
						color,
						width: self.width,
					});
				}
				self.current_stroke.clear();
			}

			// Draw completed strokes
			for stroke in &self.strokes {
				for pair in stroke.points.windows(2) {
					painter.line_segment(
						[pair[0], pair[1]],
						egui::Stroke::new(stroke.width, stroke.color),
					);
				}
			}

			// Draw current stroke in progress
			if self.current_stroke.len() >= 2 {
				let color = egui::Color32::from_rgb(
					(self.color[0] * 255.0) as u8,
					(self.color[1] * 255.0) as u8,
					(self.color[2] * 255.0) as u8,
				);
				for pair in self.current_stroke.windows(2) {
					painter.line_segment(
						[pair[0], pair[1]],
						egui::Stroke::new(self.width, color),
					);
				}
			}

			// Canvas border
			painter.rect_stroke(
				rect,
				0.0,
				egui::Stroke::new(
					1.0,
					egui::Color32::from_rgb(60, 60, 70),
				),
				egui::StrokeKind::Outside,
			);
		});
	}
}