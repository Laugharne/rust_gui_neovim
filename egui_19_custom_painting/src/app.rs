use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
enum ShapeKind {
	Line,
	Circle,
	Rectangle,
	Triangle,
}

struct DrawnShape {
	kind  : ShapeKind,
	color : egui::Color32,
	filled: bool,
}

pub struct MyApp {
	shape : ShapeKind,
	color : [f32; 3],
	filled: bool,
	shapes: Vec<DrawnShape>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			shape : ShapeKind::Line,
			color : [1.0, 0.5, 0.0],
			filled: false,
			shapes: Vec::new(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(2.0);

		egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Shape Canvas");
				ui.separator();
				ui.selectable_value(&mut self.shape, ShapeKind::Line, "Line");
				ui.selectable_value(&mut self.shape, ShapeKind::Circle, "Circle");
				ui.selectable_value(&mut self.shape, ShapeKind::Rectangle, "Rect");
				ui.selectable_value(&mut self.shape, ShapeKind::Triangle, "Tri");
				ui.separator();
				ui.color_edit_button_rgb(&mut self.color);
				ui.checkbox(&mut self.filled, "Filled");
				ui.separator();
				if ui.button("Add").clicked() {
					let c: [f32; 3] = self.color;
					let rgba = egui::Rgba::from_rgb(c[0], c[1], c[2]);
					self.shapes.push(DrawnShape {
						kind: self.shape,
						color: egui::Color32::from(rgba),
						filled: self.filled,
					});
				}
				if ui.button("Clear").clicked() {
					self.shapes.clear();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let size: egui::Vec2 = egui::vec2(440.0, 340.0);
			let (canvas, _) = ui.allocate_exact_size(
				size,
				egui::Sense::hover(),
			);
			let painter = ui.painter();
			painter.rect_filled(
				canvas, 0.0,
				egui::Color32::from_rgb(30, 30, 46),
			);

			for (i, s) in self.shapes.iter().enumerate() {
				let col: f32             = (i % 4) as f32;
				let row: f32             = (i / 4) as f32;
				let x: f32               = canvas.min.x + 55.0 + col * 100.0;
				let y: f32               = canvas.min.y + 55.0 + row * 100.0;
				let center: egui::Pos2   = egui::pos2(x, y);
				let stroke: egui::Stroke = egui::Stroke::new(2.0, s.color);

				match s.kind {
					ShapeKind::Line => {
						let p1: egui::Pos2 = egui::pos2(x - 30.0, y + 30.0);
						let p2: egui::Pos2 = egui::pos2(x + 30.0, y - 30.0);
						painter.line_segment([p1, p2], stroke);
					}
					ShapeKind::Circle => {
						if s.filled {
							painter.circle_filled(center, 30.0, s.color);
						} else {
							painter.circle_stroke(center, 30.0, stroke);
						}
					}
					ShapeKind::Rectangle => {
						let rect: egui::Rect = egui::Rect::from_center_size(
							center, egui::vec2(60.0, 50.0),
						);
						if s.filled {
							painter.rect_filled(rect, 4.0, s.color);
						} else {
							painter.rect_stroke(
								rect, 4.0, stroke,
								egui::StrokeKind::Outside,
							);
						}
					}
					ShapeKind::Triangle => {
						let pts: Vec<egui::Pos2> = vec![
							egui::pos2(x, y - 30.0),
							egui::pos2(x - 30.0, y + 20.0),
							egui::pos2(x + 30.0, y + 20.0),
						];
						if s.filled {
							painter.add(egui::Shape::convex_polygon(
								pts, s.color, egui::Stroke::NONE,
							));
						} else {
							painter.add(egui::Shape::closed_line(
								pts, stroke,
							));
						}
					}
				}
			}
		});
	}
}