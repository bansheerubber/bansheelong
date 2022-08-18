use iced::{ Button, Column, Container, Length, Row, Scrollable, Space, Text, alignment, image };

use crate::constants;
use crate::meals::{ Message, View, get_scroll_position };
use crate::style;

impl View {
	pub(crate) fn get_meal_planned(&mut self) -> Row<Message> {
		// construct the meal manager container
		let mut scrollable = Scrollable::new(&mut self.planned.meals_state)
			.width(Length::Units(300))
			.height(Length::Fill)
			.padding([20, 15, 20, 0])
			.style(style::TodoScrollable)
			.on_scroll_absolute(move |offset| Message::PlannedMealsScroll(offset))
			.min_height((get_scroll_position(&self.menu_state) as u16 + self.window_state.height) as u32)
			.push( // add menu navigation
				self.button_states
				.iter_mut()
				.zip(self.menu_state.buttons.iter())
				.fold(
					Column::new()
						.spacing(self.menu_state.button_spacing)
						.padding([0, 0, 20, 0]),
					|button_column, (state, (name, menu_type))| {
						if menu_type != &constants::Menu::Meals {
							button_column.push(
								Button::new(
									state,
									Text::new(name.clone())
										.width(Length::Fill)
										.horizontal_alignment(alignment::Horizontal::Center)
								)
									.style(style::TodoMenuButton)
									.width(Length::Fill)
									.height(Length::Units(self.menu_state.button_height))
									.on_press(Message::MenuChange(menu_type.clone()))
							)
						} else {
							button_column
						}
					}
				)
				.push(
					Button::new(
						&mut self.planned.switch_planner_state,
						Text::new("Add meal")
							.width(Length::Fill)
							.horizontal_alignment(alignment::Horizontal::Center)
					)
						.style(style::SpecialMenuButton)
						.width(Length::Fill)
						.height(Length::Units(self.menu_state.button_height))
						.on_press(Message::SwitchToPlanner)
				)
			);

		// construct planned meal list
		scrollable = self.database.as_ref().unwrap().meals_database.planned_meal_mapping.iter()
			.zip(self.planned.meal_button_states.iter_mut())
			.fold(scrollable, |scrollable, ((date, meal), button_state)| {
				let selected_meal = if self.planned.mapping.contains_key(date) {
					&self.planned.mapping[date]
				} else {
					meal
				};

				let selected_ingredient_count = selected_meal.ingredients.iter()
					.fold(0, |prev, planned_ingredient| {
						if planned_ingredient.acquired {
							prev + 1
						} else {
							prev
						}
					});
				
				scrollable.push(
					Button::new(
						button_state,
						Container::new(
							Row::new()
								.width(Length::Fill)
								.push(
									Text::new(format!("{}/{}/{}", date.month, date.day, date.year))
										.width(Length::Units(70))
								)
								.push(
									Text::new(selected_meal.recipe.name.clone())
										.width(Length::Fill)
								)
								.push(
									Text::new(
										if selected_ingredient_count == selected_meal.ingredients.len() {
											"\u{e2e6}"
										} else if selected_ingredient_count != 0 {
											"\u{e837}"
										} else {
											"\u{e836}"
										}
									)
										.width(Length::Shrink)
										.font(constants::ICONS)
								)
						)
							.width(Length::Fill)
							.padding(10)
					)
						.on_press(Message::PlannedMealSelect(date.clone()))
						.style(style::DarkButton)
						.padding(0)
				)
				.push(Space::new(Length::Units(0), Length::Units(10)))
			});

		let mut information_column = Column::new();
		if self.planned.meal_index.is_none() {
			information_column = information_column.push(
				Space::new(Length::Units(0), Length::Units(self.window_state.height - 40 - 20))
			);
		} else {
			let date = &self.planned.meal_index.unwrap();
			let selected_meal = if self.planned.mapping.contains_key(date) {
				&self.planned.mapping[date]
			} else {
				&self.database.as_ref().unwrap().meals_database.planned_meal_mapping[date]
			};

			// construct information column that lets us select which ingredients we have
			information_column = information_column
				.push(
					image::Viewer::new(&mut self.planned.image_state, self.planned.image.clone())
						.width(Length::Units(415))
				)
				.push(
					Space::new(Length::Units(0), Length::Units(5))
				)
				.push(
					Text::new(
						if let Some(minutes) = selected_meal.recipe.minutes {
							format!(
								"{} ({} minute{})",
								selected_meal.recipe.name.clone(),
								minutes,
								if minutes != 1 {
									"s"
								} else {
									""
								}
							)
						} else {
							selected_meal.recipe.name.clone()
						}
					)
						.width(Length::Fill)
						.horizontal_alignment(alignment::Horizontal::Center)
				)
				.push(
					Container::new(
						Container::new(Text::new(""))
							.style(style::VerticalRule)
							.width(Length::Fill)
							.height(Length::Units(2))
					)
						.width(Length::Fill)
						.padding([8, 0])
				)
				.push(
					Text::new("Ingredients")
				);
			
			// put the ingredients into the information column
			information_column = selected_meal.ingredients.iter()
				.zip(self.planned.ingredient_button_states.iter_mut())
				.enumerate()
				.fold(information_column, |information_column, (index, (planned_ingredient, button_state))| {
					let mut meal = selected_meal.clone();
					if let Some(ingredient) = meal.ingredients.get_mut(index) {
						ingredient.acquired = !ingredient.acquired;
					}

					information_column.push(
						Button::new(
							button_state,
							Row::new()
								.push(
									Text::new(
										if planned_ingredient.acquired {
											"\u{e2e6}"
										} else {
											"\u{e836}"
										}
									)
										.font(constants::ICONS)
								)
								.push(
									Space::new(Length::Units(6), Length::Units(0))
								)
								.push(
									Text::new(planned_ingredient.ingredient.name.clone())
								)
								.padding([10, 0, 0, 0])
						)
							.on_press(Message::APIUpdatePlannedMeal(meal))
							.style(style::DarkButton)
							.padding(0)
					)
				});

			if selected_meal.recipe.preparation_steps.len() > 0 {
				// horizontal line & title
				information_column = information_column
					.push(
						Container::new(
							Container::new(Text::new(""))
								.style(style::VerticalRule)
								.width(Length::Fill)
								.height(Length::Units(2))
						)
							.width(Length::Fill)
							.padding([8, 0])
					)
					.push(
						Text::new("Ingredient Prep")
					);
				
				// show preparation steps
				information_column = selected_meal.recipe.preparation_steps.iter()
					.enumerate()
					.fold(information_column, |information_column, (index, step)| {
						information_column.push(
							Row::new()
								.push(
									Text::new(format!("{}.", index + 1))
										.width(Length::Units(20))
								)
								.push(
									Space::new(Length::Units(6), Length::Units(0))
								)
								.push(
									Text::new(step)
								)
								.padding([10, 0, 0, 0])
						)
					});
			}
			
			if selected_meal.recipe.cooking_steps.len() > 0 {
				// horizontal line & title
				information_column = information_column
					.push(
						Container::new(
							Container::new(Text::new(""))
								.style(style::VerticalRule)
								.width(Length::Fill)
								.height(Length::Units(2))
						)
							.width(Length::Fill)
							.padding([8, 0])
					)
					.push(
						Text::new("Cooking")
					);
				
				// show cooking steps
				information_column = selected_meal.recipe.cooking_steps.iter()
					.enumerate()
					.fold(information_column, |information_column, (index, step)| {
						information_column.push(
							Row::new()
								.push(
									Text::new(format!("{}.", index + 1))
										.width(Length::Units(20))
								)
								.push(
									Space::new(Length::Units(6), Length::Units(0))
								)
								.push(
									Text::new(step)
								)
								.padding([10, 0, 0, 0])
						)
					});
			}

			// add remove meal button
			information_column = information_column
				.push(
					Space::new(Length::Units(0), Length::Units(15))
				)
				.push(
					Button::new(
						&mut self.planned.remove_meal_state,
						Text::new("Remove meal from schedule")
							.width(Length::Fill)
							.horizontal_alignment(alignment::Horizontal::Center)
					)
						.style(style::RemoveButton)
						.width(Length::Fill)
						.height(Length::Units(self.menu_state.button_height))
						.on_press(Message::APIRemovePlannedMeal(self.planned.meal_index.unwrap()))
				);
		}

		Row::new()
			.push(
				scrollable
			)
			.push(
				Space::new(Length::Units(5), Length::Units(0))
			)
			.push(
				Scrollable::new(&mut self.planned.ingredients_state)
					.push(	
						Container::new(
							information_column
						)
							.width(Length::Fill)
							.padding(10)
							.style(style::TodoItem)
					)
					.on_scroll_absolute(move |_| Message::PlannedIngredientsScroll)
					.width(Length::Fill)
					.height(Length::Fill)
					.padding([20, 15, 20, 0])
					.style(style::TodoScrollable)
			)
			.height(Length::Units(self.window_state.height))
	}
}
