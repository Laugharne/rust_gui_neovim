use eframe::egui;

struct Post {
	title: String,
	body : String,
}

pub struct MyApp {
	posts         : Vec<Post>,
	open_posts    : Vec<bool>,
	new_title     : String,
	new_body      : String,
	show_new_post : bool,
	confirm_delete: Option<usize>,
	show_about    : bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			posts: vec![
				Post {
					title: "Hello World".to_string(),
					body: "Welcome to the Post Manager!".to_string(),
				},
				Post {
					title: "Getting Started".to_string(),
					body: "Click a post to open it in a window.".to_string(),
				},
			],
			open_posts    : vec![false, false],
			new_title     : String::new(),
			new_body      : String::new(),
			show_new_post : false,
			confirm_delete: None,
			show_about    : false,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("Post Manager");
				ui.separator();
				if ui.button("New Post").clicked() {
					self.show_new_post = true;
				}
				if ui.button("About").clicked() {
					self.show_about = true;
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.posts.is_empty() {
				ui.label("No posts yet. Click 'New Post' to create one.");
			}
			for i in 0..self.posts.len() {
				ui.horizontal(|ui| {
					if ui.button(&self.posts[i].title).clicked() {
						self.open_posts[i] = true;
					}
					if ui.small_button("x").clicked() {
						self.confirm_delete = Some(i);
					}
				});
			}
		});

		for i in 0..self.posts.len() {
			let mut open: bool = self.open_posts[i];
			egui::Window::new(&self.posts[i].title)
				.open(&mut open)
				.resizable(true)
				.default_size([300.0, 200.0])
				.show(ctx, |ui| {
					ui.text_edit_multiline(&mut self.posts[i].body);
				});
			self.open_posts[i] = open;
		}

		if self.show_new_post {
			let mut show: bool = self.show_new_post;
			egui::Window::new("New Post")
				.open(&mut show)
				.resizable(false)
				.default_size([300.0, 200.0])
				.show(ctx, |ui| {
					ui.label("Title:");
					ui.text_edit_singleline(&mut self.new_title);
					ui.label("Body:");
					ui.text_edit_multiline(&mut self.new_body);
					if ui.button("Publish").clicked()
						&& !self.new_title.is_empty()
					{
						self.posts.push(Post {
							title: self.new_title.clone(),
							body: self.new_body.clone(),
						});
						self.open_posts.push(false);
						self.new_title.clear();
						self.new_body.clear();
						self.show_new_post = false;
					}
				});
			self.show_new_post = show;
		}

		if let Some(idx) = self.confirm_delete {
			let mut open: bool = true;
			egui::Window::new("Confirm Delete")
				.open(&mut open)
				.resizable(false)
				.collapsible(false)
				.anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
				.show(ctx, |ui| {
					ui.label(format!(
						"Delete '{}'?", self.posts[idx].title
					));
					ui.horizontal(|ui| {
						if ui.button("Delete").clicked() {
							self.posts.remove(idx);
							self.open_posts.remove(idx);
							self.confirm_delete = None;
						}
						if ui.button("Cancel").clicked() {
							self.confirm_delete = None;
						}
					});
				});
			if !open {
				self.confirm_delete = None;
			}
		}

		if self.show_about {
			let mut show: bool = self.show_about;
			egui::Window::new("About")
				.open(&mut show)
				.resizable(false)
				.collapsible(false)
				.show(ctx, |ui| {
					ui.heading("Post Manager");
					ui.label("A simple post management app.");
					ui.label("Built with egui in Neovim.");
				});
			self.show_about = show;
		}
	}
}