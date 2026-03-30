use eframe::egui;

#[derive(PartialEq)]
enum FontSize {
	Small,
	Medium,
	Large,
}

pub struct MyApp {
	dark_mode    : bool,
	show_sidebar : bool,
	notifications: bool,
	font_size    : FontSize,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			dark_mode    : true,
			show_sidebar : true,
			notifications: false,
			font_size    : FontSize::Medium,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Theme Preferences");
			ui.separator();

			ui.label("Appearance:");
			ui.checkbox(&mut self.dark_mode, "Dark mode");
			ui.checkbox(&mut self.show_sidebar, "Show sidebar");
			ui.checkbox(&mut self.notifications, "Enable notifications");

			ui.separator();

			ui.label("Font size:");
			ui.radio_value(&mut self.font_size, FontSize::Small,  "Small");
			ui.radio_value(&mut self.font_size, FontSize::Medium, "Medium");
			ui.radio_value(&mut self.font_size, FontSize::Large,  "Large");

			ui.separator();

			ui.label("Current settings:");
			ui.label(format!("  Dark mode: {}",     self.dark_mode));
			ui.label(format!("  Sidebar: {}",       self.show_sidebar));
			ui.label(format!("  Notifications: {}", self.notifications));
			ui.label(format!("  Font size: {}",     match self.font_size {
				FontSize::Small =>  "Small",
				FontSize::Medium => "Medium",
				FontSize::Large =>  "Large",
			}));
		});
	}
}