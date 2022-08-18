use bansheelong_types::Date;
use iced::{ Button, Column, Container, Length, Row, Scrollable, Space, Text, alignment };

use crate::constants;
use crate::meals::{ Message, PlannerState, View, get_scroll_position, right_panel };
use crate::style;

impl View {
	pub(crate) fn get_meal_planner(&mut self) -> Row<Message> {
		let day_buttons = self.planner.day_button_states.iter_mut();

		let selected_date = if let Some(day) = self.planner.day_index {
			Some(Date {
				day: day as u8,
				month: self.planner.month_index as u8 + 1,
				year: 22,
			})
		} else {
			None
		};

		let args = right_panel::PlannerRightPanelArguments {
			calendar_state: self.calendar_state,
			database: self.database.as_ref().unwrap().clone(),
			day_buttons,
			image: &self.planner.image,
			image_state: &mut self.planner.image_state,
			ingredients_state: &mut self.planner.ingredients_state,
			meal_add_state: &mut self.planner.meal_add_state,
			menu_state: &self.menu_state,
			month_index: self.planner.month_index,
			next_month_state: &mut self.planner.next_month_state,
			previous_month_state: &mut self.planner.previous_month_state,
			recipe_index: self.planner.recipe_index,
			selected_date,
			state: self.planner.state,
			year_index: self.planner.year_index,
			window_state: &self.window_state,
		};

		let (right_panel, remaining_width) = right_panel::get_planner_right_panel(args);

		// meal list
		let mut scrollable = Scrollable::new(&mut self.planner.recipes_state)
		.width(Length::Units(remaining_width))
		.height(Length::Fill)
		.padding([20, 15, 20, 0])
		.style(style::TodoScrollable)
		.on_scroll_absolute(move |offset| Message::RecipesScroll(offset))
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
					Text::new("Planned meals")
						.width(Length::Fill)
						.horizontal_alignment(alignment::Horizontal::Center)
				)
					.style(style::SpecialMenuButton)
					.width(Length::Fill)
					.height(Length::Units(self.menu_state.button_height))
					.on_press(Message::SwitchToPlanned)
			)
		);

		// construct recipes list
		scrollable = self.database.as_ref().unwrap().meals_database.recipes.iter()
			.zip(self.planner.recipe_button_states.iter_mut())
			.enumerate()
			.fold(scrollable, |scrollable, (index, (recipe, button_state))| {
				scrollable.push(
					Button::new(
						button_state,
						Container::new(
							Row::new()
								.width(Length::Fill)
								.push(
									Text::new(recipe.name.clone())
										.width(Length::Fill)
								)
								.push(
									if let Some(minutes) = recipe.minutes {
										Text::new(format!(
											"{} {}{}",
											minutes,
											if let PlannerState::MealSelect = self.planner.state {
												"min"
											} else {
												"minute"
											},
											if minutes != 1 {
												"s"
											} else {
												""
											}
										))
									} else {
										Text::new("")
									}
								)
						)
							.width(Length::Fill)
							.padding(10)
					)
						.on_press(Message::PlannerRecipeSelect(index as usize))
						.style(style::DarkButton)
						.padding(0)
				)
				.push(Space::new(Length::Units(0), Length::Units(10)))
			});

		Row::new()
			.push(scrollable)
			.push(right_panel)
	}
}