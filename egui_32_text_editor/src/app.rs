use eframe::egui;
use std::path::PathBuf;

pub struct MyApp {
	content  : String,
	file_path: Option<PathBuf>,
	status   : String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			content  : String::new(),
			file_path: None,
			status   : "Ready".to_string(),
		}
	}
}

impl MyApp {
	fn open_file(&mut self) {
		if let Some(path) = rfd::FileDialog::new()
			.add_filter("Text", &["txt", "rs", "md", "toml"])
			.add_filter("All", &["*"])
			.pick_file()
		{
			match std::fs::read_to_string(&path) {
				Ok(text) => {
					self.content = text;
					self.status  =
						format!("Opened: {}", path.display());
					self.file_path = Some(path);
				}
				Err(e) => {
					self.status = format!("Error: {}", e);
				}
			}
		}
	}

	fn save_file(&mut self) {
		if let Some(ref path) = self.file_path {
			match std::fs::write(path, &self.content) {
				Ok(()) => {
					self.status =
						format!("Saved: {}", path.display());
				}
				Err(e) => {
					self.status = format!("Error: {}", e);
				}
			}
		} else {
			self.save_file_as();
		}
	}

	fn save_file_as(&mut self) {
		if let Some(path) = rfd::FileDialog::new()
			.add_filter("Text", &["txt", "rs", "md", "toml"])
			.add_filter("All", &["*"])
			.save_file()
		{
			match std::fs::write(&path, &self.content) {
				Ok(()) => {
					self.status =
						format!("Saved: {}", path.display());
					self.file_path = Some(path);
				}
				Err(e) => {
					self.status = format!("Error: {}", e);
				}
			}
		}
	}

	fn new_file(&mut self) {
		self.content.clear();
		self.file_path = None;
		self.status    = "New file".to_string();
	}
}

impl eframe::App for MyApp {
	fn update(
		&mut self,
		ctx: &egui::Context,
		_frame: &mut eframe::Frame,
	) {
		ctx.set_pixels_per_point(1.5);

		egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui.button("New").clicked() {
					self.new_file();
				}
				if ui.button("Open").clicked() {
					self.open_file();
				}
				if ui.button("Save").clicked() {
					self.save_file();
				}
				if ui.button("Save As").clicked() {
					self.save_file_as();
				}
				ui.separator();
				if let Some(ref path) = self.file_path {
					ui.label(
						egui::RichText::new(
							path
								.file_name()
								.map(|n| n.to_string_lossy().to_string())
								.unwrap_or_default(),
						)
						.strong(),
					);
				} else {
					ui.label("Untitled");
				}
			});
		});

		egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label(&self.status);
				ui.with_layout(
					egui::Layout::right_to_left(egui::Align::Center),
					|ui| {
						ui.label(format!(
							"{} chars",
							self.content.len()
						));
					},
				);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.add(
					egui::TextEdit::multiline(&mut self.content)
						.desired_width(f32::INFINITY)
						.font(egui::TextStyle::Monospace),
				);
			});
		});
	}
}