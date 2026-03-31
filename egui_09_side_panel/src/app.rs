use eframe::egui;

pub struct MyApp {
	notes   : Vec<Note>,
	selected: usize,
}

struct Note {
	title: String,
	body : String,
}

impl Default for MyApp {
  fn default() -> Self {
	Self {
		notes: vec![
			Note {
				title: "Welcome".to_string(),
				body : "Welcome to the Note Viewer!\n\nSelect a note from the sidebar to read it.".to_string(),
			},
			Note {
				title: "Rust Tips".to_string(),
				body : "Use ownership to manage memory.\n\nBorrow with & for read access.\n\nUse &mut for write access.".to_string(),
			},
			Note {
				title: "egui Layouts".to_string(),
				body : "Horizontal groups widgets in a row.\n\nVertical stacks them top to bottom.\n\nGrid aligns in rows and columns.".to_string(),
			},
			Note {
				title: "Shopping List".to_string(),
				body : "Apples\nBread\nMilk\nCheese\nPasta".to_string(),
			},
		],
		selected: 0,
	}
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

	egui::SidePanel::left("notes_panel")
		.min_width(150.0)
		.show(ctx, |ui| {

			ui.heading("Notes");
			ui.separator();

			for i in 0..self.notes.len() {
				let selected = i == self.selected;
				if ui.selectable_label(selected, &self.notes[i].title).clicked() {
					self.selected = i;
				}
			}
	  });

	egui::CentralPanel::default().show(ctx, |ui| {
		let note = &self.notes[self.selected];

		ui.heading(&note.title);
		ui.separator();

		ui.label(&note.body);
	});

  }
}