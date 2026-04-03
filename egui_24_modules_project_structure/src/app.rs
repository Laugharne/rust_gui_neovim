use eframe::egui;
use crate::models::Contact;
use crate::ui;

pub struct MyApp {
	contacts : Vec<Contact>,
	selected : Option<usize>,
	new_name : String,
	new_email: String,
	new_phone: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			contacts: vec![
				Contact {
					name : "Alice Smith".to_string(),
					email: "alice@techcorp.com".to_string(),
					phone: "555-0101".to_string(),
				},
				Contact {
					name : "Bob Chen".to_string(),
					email: "bob@wayland.com".to_string(),
					phone: "555-0202".to_string(),
				},
			],
			selected : None,
			new_name : String::new(),
			new_email: String::new(),
			new_phone: String::new(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::SidePanel::left("contacts_panel")
			.min_width(200.0)
			.show(ctx, |ui| {
				ui.heading("Contacts");
				ui.separator();
				ui::show_contact_list(
					ui, &self.contacts, &mut self.selected,
				);
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(idx) = self.selected {
				if idx < self.contacts.len() {
					let contact = &self.contacts[idx];
					ui.heading(&contact.name);
					ui.separator();
					ui.label(format!("Email: {}", contact.email));
					ui.label(format!("Phone: {}", contact.phone));
					ui.separator();
					if ui.button("Delete").clicked() {
						self.contacts.remove(idx);
						self.selected = None;
					}
				}
			} else {
				ui.heading("Add New Contact");
				ui.separator();
				if let Some(contact) = ui::show_contact_form(
					ui,
					&mut self.new_name,
					&mut self.new_email,
					&mut self.new_phone,
				) {
					self.contacts.push(contact);
				}
			}
		});
	}
}