use eframe::egui;

struct Song {
	title   : String,
	artist  : String,
	favorite: bool,
}

pub struct MyApp {
	songs     : Vec<Song>,
	new_title : String,
	new_artist: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			songs: vec![
				Song {
					title   : "Bohemian Rhapsody".to_string(),
					artist  : "Queen".to_string(),
					favorite: true,
				},
				Song {
					title   : "Hotel California".to_string(),
					artist  : "Eagles".to_string(),
					favorite: false,
				},
				Song {
					title   : "Stairway to Heaven".to_string(),
					artist  : "Led Zeppelin".to_string(),
					favorite: false,
				},
			],
			new_title : String::new(),
			new_artist: String::new(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.heading("Playlist Manager");
		});

		egui::TopBottomPanel::bottom("add_song").show(ctx, |ui| {
			ui.separator();
			ui.horizontal(|ui| {
				ui.label("Title:");
				ui.add_sized(
					[150.0, 20.0],
					egui::TextEdit::singleline(&mut self.new_title),
				);
				ui.label("Artist:");
				ui.add_sized(
					[150.0, 20.0],
					egui::TextEdit::singleline(&mut self.new_artist),
				);
				let can_add = !self.new_title.is_empty()
					&& !self.new_artist.is_empty();
				if ui
					.add_enabled(can_add, egui::Button::new("Add Song"))
					.clicked()
				{
					self.songs.push(Song {
						title   : self.new_title.clone(),
						artist  : self.new_artist.clone(),
						favorite: false,
					});
					self.new_title.clear();
					self.new_artist.clear();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.songs.is_empty() {
				ui.label("No songs yet. Add one below!");
				return;
			}

			ui.label(format!("{} songs", self.songs.len()));
			ui.separator();

			let mut to_remove: Option<usize> = None;

			egui::ScrollArea::vertical().show(ui, |ui| {
				for (i, song) in self.songs.iter_mut().enumerate() {
					ui.horizontal(|ui| {
						let star = if song.favorite { "★" } else { "☆" };
						if ui.button(star).clicked() {
							song.favorite = !song.favorite;
						}

						ui.label(
							egui::RichText::new(&song.title).strong(),
						);
						ui.label("—");
						ui.label(&song.artist);

						ui.with_layout(
							egui::Layout::right_to_left(egui::Align::Center),
							|ui| {
								if ui.button("Remove").clicked() {
									to_remove = Some(i);
								}
							},
						);
					});
					ui.separator();
				}
			});

			if let Some(i) = to_remove {
				self.songs.remove(i);
			}
		});
	}
}