use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
pub enum FontStyle {
	Proportional,
	Monospace,
}

impl FontStyle {
	fn label(&self) -> &str {
		match self {
			FontStyle::Proportional => "Proportional",
			FontStyle::Monospace    => "Monospace",
		}
	}

	fn family(&self) -> egui::FontFamily {
		match self {
			FontStyle::Proportional => egui::FontFamily::Proportional,
			FontStyle::Monospace    => egui::FontFamily::Monospace,
		}
	}
}

#[derive(PartialEq, Clone, Copy)]
pub enum FontSize {
  Small,
  Medium,
  Large,
  ExtraLarge,
}

impl FontSize {
	fn label(&self) -> &str {
		match self {
				FontSize::Small      => "Small",
				FontSize::Medium     => "Medium",
				FontSize::Large      => "Large",
				FontSize::ExtraLarge => "Extra Large",
		}
	}

	fn points(&self) -> f32 {
		match self {
			FontSize::Small      => 14.0,
			FontSize::Medium     => 20.0,
			FontSize::Large      => 28.0,
			FontSize::ExtraLarge => 40.0,
		}
	}
}

pub struct MyApp {
	font_style  : FontStyle,
	font_size   : FontSize,
	preview_text: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			font_style  : FontStyle::Proportional,
			font_size   : FontSize::Medium,
			preview_text: String::from("The quick brown fox jumps over the lazy dog."),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.heading("Font Preview");
		});

		egui::SidePanel::left("controls").min_width(200.0).show(ctx, |ui| {
			ui.heading("Settings");
			ui.add_space(10.0);

			ui.label("Font Style:");
			egui::ComboBox::from_id_salt("font_style")
			.selected_text(self.font_style.label())
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut self.font_style, FontStyle::Proportional, "Proportional");
				ui.selectable_value(&mut self.font_style, FontStyle::Monospace, "Monospace");
			});

			ui.add_space(10.0);

			ui.label("Font Size:");
			egui::ComboBox::from_id_salt("font_size")
			.selected_text(self.font_size.label())
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut self.font_size, FontSize::Small, "Small");
				ui.selectable_value(&mut self.font_size, FontSize::Medium, "Medium");
				ui.selectable_value(&mut self.font_size, FontSize::Large, "Large");
				ui.selectable_value(&mut self.font_size, FontSize::ExtraLarge, "Extra Large");
			});

			ui.add_space(10.0);

			ui.label("Preview Text:");
			ui.text_edit_multiline(&mut self.preview_text);
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Preview");
			ui.separator();
			ui.add_space(10.0);

			let font = egui::FontId::new(
				self.font_size.points(),
				self.font_style.family(),
			);

			ui.label(
				egui::RichText::new(&self.preview_text).font(font)
			);

			ui.add_space(20.0);
			ui.separator();
			ui.add_space(10.0);

			ui.label(format!(
				"Style: {}  |  Size: {} ({:.0}pt)",
				self.font_style.label(),
				self.font_size.label(),
				self.font_size.points(),
			));
		});
	}
}