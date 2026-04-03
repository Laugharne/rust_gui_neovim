use eframe::egui;
use crate::models::Contact;

pub fn show_contact_form(
	ui   : &mut egui::Ui,
	name : &mut String,
	email: &mut String,
	phone: &mut String,
) -> Option<Contact> {
	egui::Grid::new("contact_form")
		.num_columns(2)
		.spacing([10.0, 8.0])
		.show(ui, |ui| {
			ui.label("Name:");
			ui.text_edit_singleline(name);
			ui.end_row();

			ui.label("Email:");
			ui.text_edit_singleline(email);
			ui.end_row();

			ui.label("Phone:");
			ui.text_edit_singleline(phone);
			ui.end_row();
		});

	if ui.button("Add Contact").clicked() && !name.is_empty() {
		let contact = Contact {
			name : name.clone(),
			email: email.clone(),
			phone: phone.clone(),
		};
		name.clear();
		email.clear();
		phone.clear();
		return Some(contact);
	}
	None
}