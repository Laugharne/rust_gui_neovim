use eframe::egui;
use serde::{Deserialize, Serialize};

const APP_KEY: &str = "settings-dashboard";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
enum Theme {
	Dark,
	Light,
	System,
}

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
	// Display
	theme    : Theme,
	font_size: f32,
	show_grid: bool,

	// Audio
	volume       : f32,
	notifications: bool,
	sound_effects: bool,

	// Editor
	tab_size    : usize,
	word_wrap   : bool,
	line_numbers: bool,
	auto_save   : bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			theme        : Theme::Dark,
			font_size    : 16.0,
			show_grid    : true,
			volume       : 75.0,
			notifications: true,
			sound_effects: true,
			tab_size     : 4,
			word_wrap    : true,
			line_numbers : true,
			auto_save    : false,
		}
	}
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
	Display,
	Audio,
	Editor,
}

pub struct MyApp {
	settings  : Settings,
	active_tab: Tab,
}

impl MyApp {
	pub fn new(cc: &eframe::CreationContext) -> Self {
		let settings: Settings = cc
			.storage
			.and_then(|s| eframe::get_value(s, APP_KEY))
			.unwrap_or_default();
		Self {
			settings,
			active_tab: Tab::Display,
		}
	}
}

impl eframe::App for MyApp {
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, APP_KEY, &self.settings);
	}

	fn update(
		&mut self,
		ctx: &egui::Context,
		_frame: &mut eframe::Frame,
	) {
		ctx.set_pixels_per_point(1.5);

		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Settings Dashboard");
				ui.with_layout(
					egui::Layout::right_to_left(egui::Align::Center),
					|ui| {
						if ui.button("Reset All").clicked() {
							self.settings = Settings::default();
						}
					},
				);
			});
		});

		egui::SidePanel::left("tabs")
			.default_width(120.0)
			.show(ctx, |ui| {
				ui.add_space(8.0);
				ui.selectable_value(
					&mut self.active_tab,
					Tab::Display,
					"Display",
				);
				ui.selectable_value(
					&mut self.active_tab,
					Tab::Audio,
					"Audio",
				);
				ui.selectable_value(
					&mut self.active_tab,
					Tab::Editor,
					"Editor",
				);
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			match self.active_tab {
				Tab::Display => self.display_settings(ui),
				Tab::Audio   => self.audio_settings(ui),
				Tab::Editor  => self.editor_settings(ui),
			}
		});
	}
}

impl MyApp {
	fn display_settings(&mut self, ui: &mut egui::Ui) {
		ui.heading("Display Settings");
		ui.separator();
		ui.add_space(8.0);

		ui.label("Theme:");
		ui.horizontal(|ui| {
			ui.radio_value(
				&mut self.settings.theme,
				Theme::Dark,
				"Dark",
			);
			ui.radio_value(
				&mut self.settings.theme,
				Theme::Light,
				"Light",
			);
			ui.radio_value(
				&mut self.settings.theme,
				Theme::System,
				"System",
			);
		});
		ui.add_space(8.0);

		ui.horizontal(|ui| {
			ui.label("Font Size:");
			ui.add(egui::Slider::new(
				&mut self.settings.font_size,
				10.0..=32.0,
			));
		});
		ui.add_space(8.0);

		ui.checkbox(
			&mut self.settings.show_grid,
			"Show Grid",
		);
	}

	fn audio_settings(&mut self, ui: &mut egui::Ui) {
		ui.heading("Audio Settings");
		ui.separator();
		ui.add_space(8.0);

		ui.horizontal(|ui| {
			ui.label("Volume:");
			ui.add(
				egui::Slider::new(
					&mut self.settings.volume,
					0.0..=100.0,
				)
				.suffix("%"),
			);
		});
		ui.add_space(8.0);

		ui.checkbox(
			&mut self.settings.notifications,
			"Enable Notifications",
		);
		ui.add_space(4.0);

		ui.checkbox(
			&mut self.settings.sound_effects,
			"Sound Effects",
		);
	}

	fn editor_settings(&mut self, ui: &mut egui::Ui) {
		ui.heading("Editor Settings");
		ui.separator();
		ui.add_space(8.0);

		ui.horizontal(|ui| {
			ui.label("Tab Size:");
			ui.add(egui::Slider::new(
				&mut self.settings.tab_size,
				2..=8,
			));
		});
		ui.add_space(8.0);

		ui.checkbox(
			&mut self.settings.word_wrap,
			"Word Wrap",
		);
		ui.add_space(4.0);

		ui.checkbox(
			&mut self.settings.line_numbers,
			"Line Numbers",
		);
		ui.add_space(4.0);

		ui.checkbox(
			&mut self.settings.auto_save,
			"Auto Save",
		);
	}
}