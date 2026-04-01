use eframe::egui::{self, Color32, RichText};

pub struct MyApp {
	font_size      : f32,
	show_headings  : bool,
	show_body      : bool,
	show_highlights: bool,
	show_warnings  : bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			font_size      : 20.0,
			show_headings  : true,
			show_body      : true,
			show_highlights: true,
			show_warnings  : true,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(1.5);

		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading(RichText::new("Text Styles").color(Color32::from_rgb(255, 154, 86)));
				ui.separator();
				ui.label("Font size:");
				ui.add(egui::Slider::new(&mut self.font_size, 12.0..=40.0));
			});
		});

		egui::SidePanel::left("controls").min_width(150.0).show(ctx, |ui| {
			ui.add_space(10.0);
			ui.heading("Sections");
			ui.add_space(10.0);
			ui.checkbox(&mut self.show_headings, "Headings");
			ui.checkbox(&mut self.show_body, "Body Text");
			ui.checkbox(&mut self.show_highlights, "Highlights");
			ui.checkbox(&mut self.show_warnings, "Warnings");
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.add_space(10.0);

				if self.show_headings {
					ui.label(
						RichText::new("Heading Styles")
							.size(self.font_size + 16.0)
							.strong()
							.color(Color32::WHITE),
					);
					ui.add_space(5.0);
					ui.label(
						RichText::new("Large heading with strong weight")
							.size(self.font_size + 8.0)
							.strong(),
					);
					ui.label(
						RichText::new("Medium heading in italics")
							.size(self.font_size + 4.0)
							.italics(),
					);
					ui.label(
						RichText::new("Small heading with underline")
							.size(self.font_size)
							.underline(),
					);
					ui.add_space(15.0);
					ui.separator();
					ui.add_space(15.0);
				}

				if self.show_body {
					ui.label(
						RichText::new("Body Text")
							.size(self.font_size + 16.0)
							.strong()
							.color(Color32::WHITE),
					);
					ui.add_space(5.0);
					ui.label(
						RichText::new("This is normal body text at the current font size.")
							.size(self.font_size),
					);
					ui.label(
						RichText::new("Bold text stands out for emphasis.")
							.size(self.font_size)
							.strong(),
					);
					ui.label(
						RichText::new("Italic text for subtle emphasis or quotes.")
							.size(self.font_size)
							.italics(),
					);
					ui.label(
						RichText::new("Strikethrough marks removed content.")
							.size(self.font_size)
							.strikethrough(),
					);
					ui.label(
						RichText::new("Monospace for code snippets.")
							.size(self.font_size)
							.monospace(),
					);
					ui.add_space(15.0);
					ui.separator();
					ui.add_space(15.0);
				}

				if self.show_highlights {
					ui.label(
						RichText::new("Color Highlights")
							.size(self.font_size + 16.0)
							.strong()
							.color(Color32::WHITE),
					);
					ui.add_space(5.0);
					ui.label(
						RichText::new("Success: Operation completed")
							.size(self.font_size)
							.color(Color32::from_rgb(166, 227, 161)),
					);
					ui.label(
						RichText::new("Info: System running normally")
							.size(self.font_size)
							.color(Color32::from_rgb(137, 180, 250)),
					);
					ui.label(
						RichText::new("Accent: Featured content")
							.size(self.font_size)
							.color(Color32::from_rgb(203, 166, 247)),
					);
					ui.label(
						RichText::new("Highlight: Important notice")
							.size(self.font_size)
							.color(Color32::from_rgb(249, 226, 175))
							.strong(),
					);
					ui.add_space(15.0);
					ui.separator();
					ui.add_space(15.0);
				}

				if self.show_warnings {
					ui.label(
						RichText::new("Warning Styles")
							.size(self.font_size + 16.0)
							.strong()
							.color(Color32::WHITE),
					);
					ui.add_space(5.0);
					ui.label(
						RichText::new("WARNING: Disk space low")
							.size(self.font_size)
							.color(Color32::from_rgb(250, 179, 135))
							.strong(),
					);
					ui.label(
						RichText::new("ERROR: Connection failed")
							.size(self.font_size)
							.color(Color32::from_rgb(243, 139, 168))
							.strong(),
					);
					ui.label(
						RichText::new("DEPRECATED: Use new_method() instead")
							.size(self.font_size)
							.color(Color32::from_rgb(180, 142, 173))
							.strikethrough()
							.italics(),
					);
					ui.label(
						RichText::new("FIXED: Memory leak resolved")
							.size(self.font_size)
							.color(Color32::from_rgb(166, 227, 161))
							.strikethrough(),
					);
				}
			});
		});
	}
}