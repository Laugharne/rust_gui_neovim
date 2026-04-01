use eframe::egui;

pub struct MyApp {
	file_name  : String,
	file_size  : f32,
	progress   : f32,
	downloading: bool,
	completed  : bool,
	speed      : f32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			file_name  : String::from("rust-egui-tutorial.zip"),
			file_size  : 256.0,
			progress   : 0.0,
			downloading: false,
			completed  : false,
			speed      : 0.0,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Update progress if downloading
		if self.downloading {
			self.speed       = 12.5 + (self.progress * 8.0);
			self.progress   += 0.00050;

			if self.progress    >= 1.0 {
				self.progress    = 1.0;
				self.downloading = false;
				self.completed   = true;
				self.speed       = 0.0;
			}
			ctx.request_repaint();
		}

		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.heading("Download Manager");
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.add_space(20.0);

			// File info
			ui.horizontal(|ui| {
				ui.label("File:");
				ui.strong(&self.file_name);
			});

			ui.horizontal(|ui| {
				ui.label("Size:");
				ui.label(format!("{:.1} MB", self.file_size));
			});

			ui.add_space(20.0);

			// Progress bar
			let bar = egui::ProgressBar::new(self.progress)
				.show_percentage()
				.animate(self.downloading);
			ui.add(bar);

			ui.add_space(10.0);

			// Status line
			let status = if self.completed {
				String::from("Download complete!")
			} else if self.downloading {
				let downloaded = self.file_size * self.progress;
				format!(
					"Downloading... {:.1} / {:.1} MB ({:.1} MB/s)",
					downloaded, self.file_size, self.speed,
				)
			} else {
				String::from("Ready to download")
			};
			ui.label(&status);

			ui.add_space(20.0);

			// Buttons
			ui.horizontal(|ui| {
				if self.completed {
					if ui.button("Download Again").clicked() {
						self.progress    = 0.0;
						self.completed   = false;
						self.downloading = true;
					}
				} else if self.downloading {
					if ui.button("Cancel").clicked() {
						self.downloading = false;
						self.progress    = 0.0;
					}
				} else {
					if ui.button("Start Download").clicked() {
						self.downloading = true;
						self.progress    = 0.0;
						self.completed   = false;
					}
				}

				if !self.downloading && !self.completed && self.progress == 0.0 {
					// nothing extra
				} else if !self.downloading && !self.completed {
					if ui.button("Reset").clicked() {
						self.progress = 0.0;
					}
				}
			});
		});
	}
}