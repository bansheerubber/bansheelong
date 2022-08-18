use std::collections::HashMap;
use std::sync::Arc;
use std::time::{ Duration, Instant };

use bansheelong_types::{ Date, IO, PlannedMeal };
use chrono::{ Datelike, Local, NaiveDate };
use iced::{ Button, Column, Command, Container, Element, Length, Row, Scrollable, Space, Text, alignment, button, image, scrollable };

use crate::constants;
use crate::meals::{ Message, PlannerState, has_image, right_panel };
use crate::menu::{ Menu, BUTTONS, BUTTON_AREA_SIZE, BUTTON_COUNT, BUTTON_HEIGHT, BUTTON_SPACING };
use crate::style;

static DAY_COUNT: [i8; 12] = [
	31, // january
	28, // february
	31, // march
	30, // april
	31, // may
	30, // june
	31, // july
	31, // august
	30, // september
	31, // october
	30, // november
	31, // december
];

fn get_current_month() -> u32 {
	Local::now().month() as u32 - 1
}

fn get_current_year() -> u8 {
	(Local::now().year() - 2000) as u8
}

#[derive(Debug)]
struct PlannerInfo {
	day_button_states: Vec<button::State>,
	day_index: Option<i8>,
	image: image::Handle,
	image_state: image::viewer::State,
	ingredients_state: scrollable::State,
	meal_add_state: button::State,
	month_index: u32, // starts from 0
	next_month_state: button::State,
	previous_month_state: button::State,
	recipe_button_states: Vec<button::State>,
	recipe_index: Option<usize>,
	recipes_position: f32,
	recipes_state: scrollable::State,
	state: PlannerState,
	year_index: u8, // starts at 0, 0 represents 2000
}

#[derive(Debug)]
struct PlannedInfo {
	image: image::Handle,
	image_state: image::viewer::State,
	ingredient_button_states: Vec<button::State>,
	ingredients_state: scrollable::State,
	mapping: HashMap<Date, PlannedMeal>,
	meal_button_states: Vec<button::State>,
	meal_index: Option<Date>,
	meals_position: f32,
	meals_state: scrollable::State,
	remove_meal_state: button::State,
	switch_planner_state: button::State,
}

#[derive(Debug)]
pub struct View {
	button_states: [button::State; BUTTON_COUNT as usize],
	database: Option<Arc<IO>>,
	last_interaction: Option<Instant>,
	planned: PlannedInfo,
	planner: PlannerInfo,
	showing_planner: bool,
}

impl View {
	pub fn new() -> Self {
		let scroll_position = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;

		let mut meals_state = scrollable::State::new();
		meals_state.snap_to_absolute(scroll_position);

		let mut recipes_state = scrollable::State::new();
		recipes_state.snap_to_absolute(scroll_position);

		let mut view = View {
			button_states: [button::State::new(); BUTTON_COUNT as usize],
			database: None,
			last_interaction: None,
			planned: PlannedInfo {
				image: image::Handle::from_path(format!(
					"{}/data/meals-images/placeholder.png",
					constants::get_directory()
				)),
				image_state: image::viewer::State::new(),
				ingredient_button_states: Vec::new(),
				ingredients_state: scrollable::State::new(),
				mapping: HashMap::new(),
				meal_button_states: Vec::new(),
				meal_index: None,
				meals_state,
				meals_position: scroll_position,
				remove_meal_state: button::State::new(),
				switch_planner_state: button::State::new(),
			},
			planner: PlannerInfo {
				day_index: None,
				day_button_states: Vec::new(),
				image: image::Handle::from_path(format!(
					"{}/data/meals-images/placeholder.png",
					constants::get_directory()
				)),
				image_state: image::viewer::State::new(),
				ingredients_state: scrollable::State::new(),
				meal_add_state: button::State::new(),
				month_index: 0,
				next_month_state: button::State::new(),
				previous_month_state: button::State::new(),
				recipe_button_states: Vec::new(),
				recipe_index: None,
				recipes_position: scroll_position,
				recipes_state,
				state: PlannerState::DaySelect,
				year_index: 0,
			},
			showing_planner: false,
		};

		view.transition_planner_state(PlannerState::DaySelect);
		view.select_month(get_current_year(), get_current_month());

		return view;
	}

	fn transition_planner_state(&mut self, state: PlannerState) {
		match state {
			PlannerState::DaySelect => {
				self.planner.day_index = None;
				self.planner.recipe_index = None;
				self.planned.meal_index = None;

				self.planner.month_index = get_current_month();
				self.planner.year_index = get_current_year();
			},
			PlannerState::MealSelect => {
				self.planner.recipe_index = None;
			},
		}

		self.planner.state = state;
	}

	fn select_planned_meal(&mut self, meal_index: Date) {
		self.planned.meal_index = Some(meal_index);

		self.planned.ingredient_button_states.clear();

		let selected_meal = &self.database.as_ref().unwrap().meals_database.planned_meal_mapping[&self.planned.meal_index.unwrap()];

		for _ in 0..selected_meal.ingredients.len() {
			self.planned.ingredient_button_states.push(button::State::new());
		}
	}

	fn select_month(&mut self, year_index: u8, month_index: u32) {
		self.planner.year_index = year_index;
		self.planner.month_index = month_index;

		let start_of_month = NaiveDate::from_ymd(2000 + year_index as i32, month_index + 1, 1);
		let start_of_month = if start_of_month.weekday() == chrono::Weekday::Sun {
			if month_index == 0 {
				0
			} else {
				start_of_month.iso_week().week0() + 1
			}
		} else {
			start_of_month.iso_week().week0()
		};
		let end_of_month = NaiveDate::from_ymd(2000 + year_index as i32, month_index + 1, DAY_COUNT[month_index as usize] as u32).iso_week().week0();
		let weeks = end_of_month - start_of_month + 1;

		self.planner.day_button_states.clear();
		for _ in 0..weeks * 7 {
			self.planner.day_button_states.push(button::State::new());
		}
	}

	fn get_meal_planner(&mut self) -> Row<Message> {
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

		let (right_panel, remaining_width) = right_panel::get_planner_right_panel(
			self.planner.state,
			self.planner.year_index,
			self.planner.month_index,
			day_buttons,
			&mut self.planner.previous_month_state,
			&mut self.planner.next_month_state,
			&mut self.planner.meal_add_state,
			&mut self.planner.ingredients_state,
			self.planner.recipe_index,
			selected_date,
			&self.planner.image,
			&mut self.planner.image_state,
			self.database.as_ref().unwrap().clone()
		);

		// meal list
		let mut scrollable = Scrollable::new(&mut self.planner.recipes_state)
		.width(Length::Units(remaining_width))
		.height(Length::Fill)
		.padding([20, 15, 20, 0])
		.style(style::TodoScrollable)
		.on_scroll_absolute(move |offset| Message::RecipesScroll(offset))
		.min_height(((BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) + constants::WINDOW_HEIGHT) as u32)
		.push( // add menu navigation
			self.button_states
			.iter_mut()
			.zip(BUTTONS.iter())
			.fold(
				Column::new()
					.spacing(BUTTON_SPACING)
					.padding([0, 0, 20, 0]),
				|button_column, (state, (name, menu_type))| {
					if menu_type != &Menu::Meals {
						button_column.push(
							Button::new(
								state,
								Text::new(name.clone())
									.width(Length::Fill)
									.horizontal_alignment(alignment::Horizontal::Center)
							)
								.style(style::TodoMenuButton)
								.width(Length::Fill)
								.height(Length::Units(BUTTON_HEIGHT))
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
					.height(Length::Units(BUTTON_HEIGHT))
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

	fn get_meal_planned(&mut self) -> Row<Message> {
		// construct the meal manager container
		let mut scrollable = Scrollable::new(&mut self.planned.meals_state)
			.width(Length::Units(300))
			.height(Length::Fill)
			.padding([20, 15, 20, 0])
			.style(style::TodoScrollable)
			.on_scroll_absolute(move |offset| Message::PlannedMealsScroll(offset))
			.min_height(((BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) + constants::WINDOW_HEIGHT) as u32)
			.push( // add menu navigation
				self.button_states
				.iter_mut()
				.zip(BUTTONS.iter())
				.fold(
					Column::new()
						.spacing(BUTTON_SPACING)
						.padding([0, 0, 20, 0]),
					|button_column, (state, (name, menu_type))| {
						if menu_type != &Menu::Meals {
							button_column.push(
								Button::new(
									state,
									Text::new(name.clone())
										.width(Length::Fill)
										.horizontal_alignment(alignment::Horizontal::Center)
								)
									.style(style::TodoMenuButton)
									.width(Length::Fill)
									.height(Length::Units(BUTTON_HEIGHT))
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
						.height(Length::Units(BUTTON_HEIGHT))
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
				Space::new(Length::Units(0), Length::Units(constants::WINDOW_HEIGHT - 40 - 20))
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
						.height(Length::Units(BUTTON_HEIGHT))
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
					.width(Length::Units(435))
					.height(Length::Fill)
					.padding([20, 15, 20, 0])
					.style(style::TodoScrollable)
			)
			.height(Length::Units(constants::WINDOW_HEIGHT))
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::APIAddPlannedMeal(_) => {
				self.transition_planner_state(PlannerState::DaySelect);
				Command::none()
			},
			Message::APIRemovePlannedMeal(_) => {
				self.planned.meal_index = None;
				Command::none()
			},
			Message::APIUpdatePlannedMeal(meal) => {
				self.planned.mapping.insert(meal.date, meal);
				Command::none()
			},
			Message::MenuChange(_) => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;

				self.showing_planner = false;

				self.transition_planner_state(PlannerState::DaySelect);

				Command::none()
			},
			Message::PlannedIngredientsScroll => {
				self.last_interaction = Some(Instant::now());
				self.planned.ingredients_state.set_force_disable(false);
				Command::none()
			},
			Message::PlannedMealsScroll(scroll) => {
				self.last_interaction = Some(Instant::now());
				self.planned.meals_position = scroll;
				self.planned.meals_state.set_force_disable(false);
				Command::none()
			},
			Message::PlannedMealSelect(date) => {
				self.select_planned_meal(date);
				self.planned.ingredients_state.snap_to_absolute(0.0);

				let recipe = &self.database.as_ref().unwrap().meals_database.planned_meal_mapping[&date].recipe;
				if has_image(&recipe.name) {
					self.planned.image = image::Handle::from_path(
						format!("{}/data/meals-images/{}.png", constants::get_directory(), recipe.name)
					);
				} else {
					self.planned.image = image::Handle::from_path(
						format!("{}/data/meals-images/placeholder.png", constants::get_directory())
					);
				}

				Command::none()
			},
			Message::PlannerDaySelect(index) => {
				self.planner.day_index = Some(index);
				self.transition_planner_state(PlannerState::MealSelect);
				Command::none()
			},
			Message::PlannerMonthSelect(year, month) => {
				self.select_month(year, month);
				Command::none()
			},
			Message::PlannerRecipeScroll => {
				self.last_interaction = Some(Instant::now());
				self.planner.ingredients_state.set_force_disable(false);
				Command::none()
			},
			Message::PlannerRecipeSelect(index) => {
				self.planner.recipe_index = Some(index);
				self.planner.ingredients_state.snap_to_absolute(0.0);

				let recipe = &self.database.as_ref().unwrap().meals_database.recipes[index];
				if has_image(&recipe.name) {
					self.planner.image = image::Handle::from_path(
						format!("{}/data/meals-images/{}.png", constants::get_directory(), recipe.name)
					);
				} else {
					self.planner.image = image::Handle::from_path(
						format!("{}/data/meals-images/placeholder.png", constants::get_directory())
					);
				}

				Command::none()
			},
			Message::RecipesScroll(scroll) => {
				self.last_interaction = Some(Instant::now());
				self.planner.recipes_position = scroll;
				self.planner.recipes_state.set_force_disable(false);
				Command::none()
			},
			Message::SwitchToPlanned => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;

				self.showing_planner = false;

				self.transition_planner_state(PlannerState::DaySelect);

				Command::none()
			},
			Message::SwitchToPlanner => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;
				
				self.showing_planner = true;

				self.transition_planner_state(PlannerState::DaySelect);

				Command::none()
			},
			Message::Tick => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				if self.last_interaction.is_some() {
					if Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(2)
						&& self.planned.meals_position < size
					{
						self.planned.meals_state.snap_to_absolute(size);
						self.planned.meals_position = size;
					}

					if Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(2)
						&& self.planner.recipes_position < size
					{
						self.planner.recipes_state.snap_to_absolute(size);
						self.planner.recipes_position = size;
					}

					if Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(4) {
						self.planned.ingredients_state.set_force_disable(true);
						self.planned.meals_state.set_force_disable(true);

						self.planner.ingredients_state.set_force_disable(true);
						self.planner.recipes_state.set_force_disable(true);
					}
				}

				Command::none()
			},
			Message::Update(io) => {
				self.database = io;

				self.planner.recipe_button_states.clear();
				for _ in 0..self.database.as_ref().unwrap().meals_database.recipes.len() {
					self.planner.recipe_button_states.push(button::State::new());
				}

				self.planned.meal_button_states.clear();
				for _ in 0..self.database.as_ref().unwrap().meals_database.planned_meal_mapping.len() {
					self.planned.meal_button_states.push(button::State::new());
				}

				self.planned.mapping.clear();

				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		if self.showing_planner {
			self.get_meal_planner().into()
		} else {
			self.get_meal_planned().into()
		}
	}
}
