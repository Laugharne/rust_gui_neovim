use eframe::egui;

pub struct MyApp {
	recipes : Vec<Recipe>,
	selected: usize,
}

struct Recipe {
	title      : String,
	category   : String,
	time       : String,
	servings   : u32,
	ingredients: Vec<String>,
	steps      : Vec<String>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			recipes: vec![

				Recipe {
					title      : "Pasta Carbonara".to_string(),
					category   : "Italian".to_string(),
					time       : "25 min".to_string(),
					servings   : 4,
					ingredients: vec![
						"400g spaghetti".to_string(),
						"200g pancetta".to_string(),
						"4 egg yolks".to_string(),
						"100g parmesan".to_string(),
						"Black pepper".to_string(),
					],
					steps: vec![
						"Boil pasta in salted water.".to_string(),
						"Fry pancetta until crispy.".to_string(),
						"Mix egg yolks with parmesan.".to_string(),
						"Toss hot pasta with pancetta.".to_string(),
						"Stir in egg mixture off heat.".to_string(),
					],
				},

				Recipe {
					title      : "Greek Salad".to_string(),
					category   : "Mediterranean".to_string(),
					time       : "10 min".to_string(),
					servings   : 2,
					ingredients: vec![
						"2 tomatoes".to_string(),
						"1 cucumber".to_string(),
						"Red onion".to_string(),
						"Feta cheese".to_string(),
						"Olive oil".to_string(),
					],
					steps: vec![
						"Chop tomatoes and cucumber.".to_string(),
						"Slice red onion thinly.".to_string(),
						"Combine in a bowl.".to_string(),
						"Add feta and drizzle oil.".to_string(),
					],
				},

				Recipe {
					title      : "Chicken Stir Fry".to_string(),
					category   : "Asian".to_string(),
					time       : "20 min".to_string(),
					servings   : 3,
					ingredients: vec![
						"500g chicken breast".to_string(),
						"Bell peppers".to_string(),
						"Soy sauce".to_string(),
						"Garlic and ginger".to_string(),
						"Rice".to_string(),
					],
					steps: vec![
						"Slice chicken into strips.".to_string(),
						"Heat oil in a wok.".to_string(),
						"Cook chicken until golden.".to_string(),
						"Add vegetables and stir fry.".to_string(),
						"Season with soy sauce.".to_string(),
						"Serve over rice.".to_string(),
					],

				},
			],
			selected: 0,
		}
	}

}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
	  ui.horizontal(|ui| {
		ui.heading("Recipe Viewer");
		ui.separator();
		for i in 0..self.recipes.len() {
		  if ui.selectable_label(
			i == self.selected,
			&self.recipes[i].title,
		  ).clicked() {
			self.selected = i;
		  }
		}
	  });
	});

	egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
	  let recipe = &self.recipes[self.selected];
	  ui.horizontal(|ui| {
		ui.label(format!("Category: {}", recipe.category));
		ui.separator();

		ui.label(format!("Time: {}", recipe.time));
		ui.separator();
		
		ui.label(format!("Servings: {}", recipe.servings));
	  });
	});

	egui::CentralPanel::default().show(ctx, |ui| {
		let recipe = &self.recipes[self.selected];

		ui.heading(&recipe.title);
		ui.separator();

		ui.heading("Ingredients");
		for item in &recipe.ingredients {
			ui.label(format!("  - {}", item));
		}

		ui.add_space(10.0);
		ui.heading("Steps");
		for (i, step) in recipe.steps.iter().enumerate() {
			ui.label(format!("  {}. {}", i + 1, step));
		}
	});
  }

}