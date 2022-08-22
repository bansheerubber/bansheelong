use std::collections::HashMap;
use std::time::{ Duration, Instant };

use bansheelong_types::Date;
use chrono::{ Datelike, NaiveDate };
use iced::{ Command, Container, Element, Length, Padding, Space, button, image, scrollable };

use crate::constants;
use crate::meals::{
	Props,
	Message,
	PlannedInfo,
	PlannerInfo,
	PlannerState,
	View,
	get_current_month,
	get_current_year,
	get_scroll_position,
	has_image
};
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

impl View {
	pub fn new<P: Into<Padding>>(
		props: Props,
		menu_state: constants::MenuState,
		window_state: constants::WindowState,
		empty_padding: P
	) -> Self {
		let scroll_position = get_scroll_position(&menu_state);

		let mut meals_state = scrollable::State::new();
		meals_state.snap_to_absolute(scroll_position);

		let mut recipes_state = scrollable::State::new();
		recipes_state.snap_to_absolute(scroll_position);

		let mut view = View {
			button_states: vec![button::State::new(); menu_state.button_count as usize],
			database: None,
			empty_padding: empty_padding.into(),
			last_interaction: None,
			menu_state,
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
			props,
			showing_planner: false,
			window_state,
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
				let size = get_scroll_position(&self.menu_state);
				
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
				let size = get_scroll_position(&self.menu_state);
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;

				self.showing_planner = false;

				self.transition_planner_state(PlannerState::DaySelect);

				Command::none()
			},
			Message::SwitchToPlanner => {
				let size = get_scroll_position(&self.menu_state);
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;
				
				self.showing_planner = true;

				self.transition_planner_state(PlannerState::DaySelect);

				Command::none()
			},
			Message::Tick => {
				let size = get_scroll_position(&self.menu_state);
				
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

				if self.database.is_some() {
					self.planner.recipe_button_states.clear();
					for _ in 0..self.database.as_ref().unwrap().meals_database.recipes.len() {
						self.planner.recipe_button_states.push(button::State::new());
					}

					self.planned.meal_button_states.clear();
					for _ in 0..self.database.as_ref().unwrap().meals_database.planned_meal_mapping.len() {
						self.planned.meal_button_states.push(button::State::new());
					}

					if self.planned.meal_index.is_some()
						&& !self.database.as_ref().unwrap().meals_database.planned_meal_mapping.contains_key(
							&self.planned.meal_index.unwrap()
						)
					{
						self.planned.meal_index = None;
					}

					self.planned.mapping.clear();
				}

				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		if let None = self.database {
			return Container::new(
				Container::new(
					Space::new(Length::Units(0), Length::Units(0))
				)
					.width(Length::Fill)
					.height(Length::Fill)
					.style(style::TodoItem)
			)
				.width(Length::Units(self.menu_state.width))
				.height(Length::Units(self.window_state.height))
				.padding(self.empty_padding)
				.into();
		}
		
		if self.showing_planner {
			self.get_meal_planner().into()
		} else {
			self.get_meal_planned().into()
		}
	}
}
