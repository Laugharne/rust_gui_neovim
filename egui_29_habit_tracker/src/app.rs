use eframe::egui;

const DAYS: [&str; 7] =
	["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Habit {
	name: String,
	days: [bool; 7],
}

impl Habit {
	fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			days: [false; 7],
		}
	}

	fn streak(&self) -> usize {
		self
			.days
			.iter()
			.rev()
			.take_while(|&&d| d)
			.count()
	}

	fn completed(&self) -> usize {
		self.days.iter().filter(|&&d| d).count()
	}
}

pub struct MyApp {
	habits   : Vec<Habit>,
	new_habit: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			habits: vec![
				Habit::new("Exercise"),
				Habit::new("Read"),
				Habit::new("Meditate"),
			],
			new_habit: String::new(),
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

		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Habit Tracker");
				ui.separator();
				let total: usize =
					self.habits.iter().map(|h| h.completed()).sum();
				let possible: usize = self.habits.len() * 7;
				if possible > 0 {
					let pct: f32 = total as f32 / possible as f32;
					ui.label(format!(
						"{}/{} ({:.0}%)",
						total,
						possible,
						pct * 100.0
					));
				}
			});
		});

		egui::TopBottomPanel::bottom("add_habit").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("New habit:");
				ui.add_sized(
					[200.0, 20.0],
					egui::TextEdit::singleline(&mut self.new_habit),
				);
				let can_add = !self.new_habit.trim().is_empty();
				if ui
					.add_enabled(can_add, egui::Button::new("Add"))
					.clicked()
				{
					self
						.habits
						.push(Habit::new(self.new_habit.trim()));
					self.new_habit.clear();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.habits.is_empty() {
				ui.label("No habits yet. Add one below!");
				return;
			}

			egui::Grid::new("habit_grid")
				.striped(true)
				.min_col_width(40.0)
				.show(ui, |ui| {
					// Header row
					ui.label(
						egui::RichText::new("Habit").strong(),
					);
					for day in &DAYS {
						ui.label(
							egui::RichText::new(*day).strong(),
						);
					}
					ui.label(
						egui::RichText::new("Done").strong(),
					);
					ui.label(
						egui::RichText::new("Streak").strong(),
					);
					ui.label(
						egui::RichText::new("").strong(),
					);
					ui.end_row();

					// Habit rows
					let mut to_remove: Option<usize> = None;

					for (i, habit) in
						self.habits.iter_mut().enumerate()
					{
						ui.label(&habit.name);

						for day in habit.days.iter_mut() {
							let color = if *day {
								egui::Color32::from_rgb(100, 200, 100)
							} else {
								egui::Color32::from_rgb(80, 80, 80)
							};
							let symbol = if *day { "X" } else { "." };
							if ui
								.add(
									egui::Button::new(
										egui::RichText::new(symbol)
											.color(egui::Color32::WHITE),
									)
									.fill(color),
								)
								.clicked()
							{
								*day = !*day;
							}
						}

						ui.label(format!(
							"{}/7",
							habit.completed()
						));

						let streak: usize               = habit.streak();
						let streak_color: egui::Color32 = if streak >= 5 {
							egui::Color32::from_rgb(255, 200, 50)
						} else if streak >= 3 {
							egui::Color32::from_rgb(100, 200, 100)
						} else {
							egui::Color32::GRAY
						};
						ui.label(
							egui::RichText::new(format!("{}", streak))
								.color(streak_color),
						);

						if ui.button("Del").clicked() {
							to_remove = Some(i);
						}

						ui.end_row();
					}

					if let Some(i) = to_remove {
						self.habits.remove(i);
					}
				});
		});
	}
}