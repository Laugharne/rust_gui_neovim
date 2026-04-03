use eframe::egui;

pub struct Toggle<'a> {
	value: &'a mut bool,
	label: &'a str,
}

impl<'a> Toggle<'a> {
	pub fn new(value: &'a mut bool, label: &'a str) -> Self {
		Self { value, label }
	}
}

impl<'a> egui::Widget for Toggle<'a> {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		let size: egui::Vec2             = egui::vec2(40.0, 20.0);
		let (rect, response) = ui.allocate_exact_size(
			size, egui::Sense::click(),
		);

		if response.clicked() {
			*self.value = !*self.value;
		}

		let painter: &egui::Painter  = ui.painter();
		let bg_color: egui::Color32 = if *self.value {
			egui::Color32::from_rgb(100, 180, 100)
		} else {
			egui::Color32::from_rgb(120, 120, 120)
		};
		painter.rect_filled(rect, 10.0, bg_color);

		let circle_x: f32 = if *self.value {
			rect.right() - 10.0
		} else {
			rect.left() + 10.0
		};
		let circle_center: egui::Pos2 = egui::pos2(circle_x, rect.center().y);
		painter.circle_filled(
			circle_center, 7.0, egui::Color32::WHITE,
		);

		ui.label(self.label);

		response
	}
}