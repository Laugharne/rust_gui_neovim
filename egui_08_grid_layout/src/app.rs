use eframe::egui;

pub struct MyApp {
	name     : String,
	email    : String,
	phone    : String,
	subject  : String,
	submitted: bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name     : String::new(),
			email    : String::new(),
			phone    : String::new(),
			subject  : String::new(),
			submitted: false,
		}
	}
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx, |ui| {
		ui.heading("Contact Form");
		ui.separator();

		egui::Grid::new("contact_grid")
			.num_columns(2)
			.spacing([40.0, 8.0])
			.show(ui, |ui| {

				ui.label("Name:");
				ui.text_edit_singleline(&mut self.name);
				ui.end_row();

				ui.label("Email:");
				ui.text_edit_singleline(&mut self.email);
				ui.end_row();

				ui.label("Phone:");
				ui.text_edit_singleline(&mut self.phone);
				ui.end_row();

				ui.label("Subject:");
				ui.text_edit_singleline(&mut self.subject);
				ui.end_row();
		});

		ui.separator();

		if ui.button("Submit").clicked() {
			self.submitted = true;
		}

		if self.submitted {
			ui.separator();
			ui.label("Submitted!");

			egui::Grid::new("summary_grid")
				.num_columns(2)
				.spacing([40.0, 4.0])
				.show(ui, |ui| {

					ui.label("Name:");
					ui.label(&self.name);
					ui.end_row();

					ui.label("Email:");
					ui.label(&self.email);
					ui.end_row();

					ui.label("Phone:");
					ui.label(&self.phone);
					ui.end_row();

					ui.label("Subject:");
					ui.label(&self.subject);
					ui.end_row();
			});
		}
	});
  }
}