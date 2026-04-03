use eframe::egui;
use crate::models::Contact;

pub fn show_contact_list(
	ui      : &mut egui::Ui,
	contacts: &[Contact],
	selected: &mut Option<usize>,
) {
	if contacts.is_empty() {
		ui.label("No contacts yet.");
		return;
	}
	for (i, contact) in contacts.iter().enumerate() {
		let is_selected = *selected == Some(i);
		if ui.selectable_label(is_selected, &contact.name).clicked() {
			if is_selected {
				*selected = None;
			} else {
				*selected = Some(i);
			}
		}
	}
}