use eframe::egui;

pub struct MyApp {
	display   : String,
	first     : Option<f64>,
	operator  : Option<char>,
	reset_next: bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			display   : "0".to_string(),
			first     : None,
			operator  : None,
			reset_next: false,
		}
	}
}

impl MyApp {
	fn press_digit(&mut self, digit: &str) {
		if self.reset_next {
			self.display    = String::new();
			self.reset_next = false;
		}
		if self.display == "0" && digit != "." {
			self.display = digit.to_string();
		} else if digit == "." && self.display.contains(".") {
			return;
		} else {
			self.display.push_str(digit);
		}
	}

	fn press_operator(&mut self, op: char) {
		self.evaluate();
		self.first      = self.display.parse::<f64>().ok();
		self.operator   = Some(op);
		self.reset_next = true;
	}

	fn evaluate(&mut self) {
		if let (Some(first), Some(op)) = (self.first, self.operator)
		{
			if let Ok(second) = self.display.parse::<f64>() {
				let result = match op {
					'+' => first + second,
					'-' => first - second,
					'*' => first * second,
					'/' => {
						if second == 0.0 {
							self.display = "Error".to_string();
							self.first = None;
							self.operator = None;
							self.reset_next = true;
							return;
						}
						first / second
					}
					_ => return,
				};
				if result == result.floor() {
					self.display = format!("{}", result as i64);
				} else {
					self.display = format!("{:.6}", result)
						.trim_end_matches("0")
						.trim_end_matches(".")
						.to_string();
				}
			}
		}
		self.first = None;
		self.operator = None;
		self.reset_next = true;
	}

	fn clear(&mut self) {
		self.display = "0".to_string();
		self.first = None;
		self.operator = None;
		self.reset_next = false;
	}
}

impl eframe::App for MyApp {
	fn update(
		&mut self,
		ctx: &egui::Context,
		_frame: &mut eframe::Frame,
	) {
		ctx.set_pixels_per_point(1.5);

		egui::TopBottomPanel::top("display")
			.exact_height(60.0)
			.show(ctx, |ui| {
			ui.add_space(8.0);
			egui::Frame::new()
				.fill(egui::Color32::from_rgb(30, 30, 40))
				.corner_radius(8.0)
				.inner_margin(12.0)
				.show(ui, |ui| {
					ui.with_layout(
						egui::Layout::right_to_left(egui::Align::Center),
						|ui| {
							ui.label(
								egui::RichText::new(&self.display)
									.size(28.0)
									.color(egui::Color32::WHITE)
									.monospace(),
							);
						},
					);
				});
			ui.add_space(8.0);
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);

			let btn_size = egui::vec2(
				(ui.available_width() - 12.0) / 4.0,
				35.0,
			);

			let rows = [
				["C", "+/-", "%", "/"],
				["7", "8", "9", "*"],
				["4", "5", "6", "-"],
				["1", "2", "3", "+"],
				["0", ".", "=", ""],
			];

			for row in &rows {
				ui.horizontal(|ui| {
					for &label in row {
						if label.is_empty() {
							continue;
						}

						let is_op =
							["/", "*", "-", "+", "="].contains(&label);
						let is_top =
							["C", "+/-", "%"].contains(&label);

						let button = if is_op {
							egui::Button::new(
								egui::RichText::new(label)
									.size(20.0)
									.color(egui::Color32::WHITE),
							)
							.fill(egui::Color32::from_rgb(255, 149, 0))
						} else if is_top {
							egui::Button::new(
								egui::RichText::new(label).size(20.0),
							)
							.fill(egui::Color32::from_rgb(80, 80, 80))
						} else {
							egui::Button::new(
								egui::RichText::new(label).size(20.0),
							)
							.fill(egui::Color32::from_rgb(50, 50, 55))
						};

						let size = if label == "0" {
							egui::vec2(btn_size.x * 2.0 + 4.0, btn_size.y)
						} else {
							btn_size
						};

						if ui.add_sized(size, button).clicked() {
							match label {
								"C" => self.clear(),
								"+/-" => {
									if let Ok(val) =
										self.display.parse::<f64>()
									{
										let neg = -val;
										if neg == neg.floor() {
											self.display =
												format!("{}", neg as i64);
										} else {
											self.display = format!("{}", neg);
										}
									}
								}
								"%" => {
									if let Ok(val) =
										self.display.parse::<f64>()
									{
										let pct = val / 100.0;
										self.display = format!("{}", pct);
									}
								}
								"=" => self.evaluate(),
								"+" | "-" | "*" | "/" => {
									self.press_operator(
										label.chars().next().unwrap(),
									);
								}
								_ => self.press_digit(label),
							}
						}
					}
				});
			}
		});
	}
}