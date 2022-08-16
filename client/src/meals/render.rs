use std::sync::Arc;
use std::time::{ Duration, Instant };

use bansheelong_types::{ Date, IO, Ingredient, PlannedMeal, Recipe };
use chrono::{ Datelike, Local, NaiveDate };
use iced::{ Button, Column, Command, Container, Element, Length, Row, Scrollable, Space, Text, alignment, button, image, scrollable };

use crate::constants;
use crate::menu::{ Menu, BUTTONS, BUTTON_AREA_SIZE, BUTTON_COUNT, BUTTON_HEIGHT, BUTTON_SPACING };
use crate::shared::Underline;
use crate::style;

const MONTH: [&'static str; 12] = [
	"January",
	"February",
	"March",
	"April",
	"May",
	"June",
	"July",
	"August",
	"September",
	"October",
	"November",
	"December",
];

const DAY: [&'static str; 7] = [
	"Sunday",
	"Monday",
	"Tuesday",
	"Wednesday",
	"Thursday",
	"Friday",
	"Saturday",
];

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

const WEEK_SELECT_WIDTH: u16 = 7 * 35 + 6 * 4;
const WEEK_SELECT_DAY_SIZE: Length = Length::Units(35);
const WEEK_SELECT_DAY_SPACING: Length = Length::Units(4);

fn get_current_month() -> usize {
	Local::now().month() as usize - 1
}

fn get_planner_right_panel<'a, I>(state: PlannerState, week_select_buttons: I, day_buttons: I) -> Column<'a, Message>
	where
		I: Iterator<Item = &'a mut button::State>
	{
		let time = Local::now();
		let current_month = time.month();
		let current_year = time.year();

		let start_of_month = NaiveDate::from_ymd(current_year, current_month, 1);
		let end_of_month = NaiveDate::from_ymd(current_year, current_month, DAY_COUNT[current_month as usize - 1] as u32);
		let weeks = (end_of_month.iso_week().week0() - start_of_month.iso_week().week0() + 1) as usize;
		
		match state {
			PlannerState::WeekSelect => {
				let mut month = Column::new()
					.push(
						Container::new(
							Underline::new(format!(" {} ", MONTH[current_month as usize - 1]))
								.size(25)
								.font(constants::NOTOSANS_BOLD)
						)
							.width(Length::Fill)
							.align_x(alignment::Horizontal::Center)
					)
					.push(Space::new(Length::Units(0), Length::Units(if weeks < 6 { 10 } else { 5 })))
					.width(Length::Units(WEEK_SELECT_WIDTH));

				let mut day: i8 = match start_of_month.weekday() {
					chrono::Weekday::Sun => 1,
					chrono::Weekday::Mon => 0,
					chrono::Weekday::Tue => -1,
					chrono::Weekday::Wed => -2,
					chrono::Weekday::Thu => -3,
					chrono::Weekday::Fri => -4,
					chrono::Weekday::Sat => -5,
				};

				month = week_select_buttons
					.enumerate()
					.fold(
						month,
						|month, (week_index, state)| {
							let mut week = Row::new();
							for _ in 0..7 {
								week = week.push(
									Container::new(
										if day >= 1 && day <= DAY_COUNT[current_month as usize - 1] {
											Text::new(day.to_string())
												.size(18)
										} else {
											Text::new("")
										}
									)
										.padding([0, 0, 0, 3])
										.width(WEEK_SELECT_DAY_SIZE)
										.height(WEEK_SELECT_DAY_SIZE)
										.style(style::MealsDayContainer)
								)
								.push(Space::new(WEEK_SELECT_DAY_SPACING, Length::Units(0)));

								day += 1;
							}

							month.push(
									Button::new(
										state,
										week
									)
										.on_press(Message::PlannerWeekSelect(week_index as u8))
										.style(style::DarkButton)
										.padding(0)
								)
								.push(Space::new(Length::Units(0), WEEK_SELECT_DAY_SPACING))
						}
					);

				month = month.push(Space::new(Length::Units(0), Length::Units(10)));

				return month;
			},
			PlannerState::DaySelect => {
				DAY.iter()
					.zip(day_buttons)
					.enumerate()
					.fold(
						Column::new()
							.spacing(BUTTON_SPACING / 2),
						|week, (index, (day, state))| {
							week.push(
								Button::new(
									state,
									Text::new(day.clone())
										.width(Length::Fill)
										.horizontal_alignment(alignment::Horizontal::Center)
								)
									.style(style::TodoMenuButton)
									.width(Length::Fill)
									.height(Length::Units(BUTTON_HEIGHT))
									.on_press(Message::PlannerDaySelect(index as u8))
							)
						}
					)
			},
			PlannerState::MealSelect => {
				Column::new()
			},
		}
	}

#[derive(Clone, Copy, Debug)]
enum PlannerState {
	WeekSelect,
	DaySelect,
	MealSelect,
}

#[derive(Debug)]
struct PlannerInfo {
	day_button_states: [button::State; 7],
	day_index: Option<u8>,
	month_index: usize,
	recipe_button_states: Vec<button::State>,
	recipes_position: f32,
	recipes_state: scrollable::State,
	state: PlannerState,
	week_button_states: Vec<button::State>,
	week_index: Option<u8>,
}

#[derive(Debug)]
struct PlannedInfo {
	image: image::Handle,
	image_state: image::viewer::State,
	ingredient_button_states: Vec<button::State>,
	ingredients_state: scrollable::State,
	meal_button_states: Vec<button::State>,
	meal_index: Option<usize>,
	meals: Vec<PlannedMeal>,
	meals_position: f32,
	meals_state: scrollable::State,
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

#[derive(Debug, Clone)]
pub enum Message {
	MenuChange(Menu),
	PlannedIngredientsScroll,
	PlannedIngredientSelect(usize),
	PlannedMealsScroll(f32),
	PlannedMealSelect(usize),
	PlannerDaySelect(u8),
	PlannerWeekSelect(u8),
	RecipesScroll(f32),
	SwitchToPlanned,
	SwitchToPlanner,
	Tick,
	Update(Option<Arc<IO>>),
}

impl View {
	pub fn new() -> Self {
		let scroll_position = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;

		let recipe = Recipe {
			ingredients: vec![
				Ingredient::new(String::from("Box of orange chicken")),
				Ingredient::new(String::from("Rice")),
				Ingredient::new(String::from("Broccoli")),
			],
			name: String::from("Orange Chicken"),
		};

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
					"{}/data/meals/orange chicken.jpg",
					constants::get_directory()
				)),
				image_state: image::viewer::State::new(),
				ingredient_button_states: Vec::new(),
				meal_button_states: vec![button::State::new(); 3],
				meal_index: None,
				meals: vec![
					PlannedMeal::new(Date {
						day: 8,
						month: 7,
						year: 22,
					}, recipe.clone()),
					PlannedMeal::new(Date {
						day: 8,
						month: 8,
						year: 22,
					}, recipe.clone()),
					PlannedMeal::new(Date {
						day: 8,
						month: 9,
						year: 22,
					}, recipe.clone()),
				],
				meals_state,
				meals_position: scroll_position,
				ingredients_state: scrollable::State::new(),
				switch_planner_state: button::State::new(),
			},
			planner: PlannerInfo {
				day_index: None,
				day_button_states: [button::State::new(); 7],
				month_index: 0,
				recipe_button_states: Vec::new(),
				recipes_position: scroll_position,
				recipes_state,
				state: PlannerState::WeekSelect,
				week_button_states: Vec::new(),
				week_index: None,
			},
			showing_planner: false,
		};

		view.transition_planner_state(PlannerState::WeekSelect);
		view.select_month(get_current_month());

		return view;
	}

	fn transition_planner_state(&mut self, state: PlannerState) {
		match state {
			PlannerState::WeekSelect => {
				self.planner.day_index = None;
				self.planner.week_index = None;
			},
			PlannerState::DaySelect => {},
			PlannerState::MealSelect => {},
		}
	}

	fn select_planned_meal(&mut self, meal_index: usize) {
		self.planned.meal_index = Some(meal_index);

		self.planned.ingredient_button_states.clear();
		for _ in 0..self.planned.meals[self.planned.meal_index.unwrap()].ingredients.len() {
			self.planned.ingredient_button_states.push(button::State::new());
		}
	}

	fn select_month(&mut self, month_index: usize) {
		self.planner.month_index = month_index;

		let time = Local::now();
		let current_month = time.month();
		let current_year = time.year();

		let start_of_month = NaiveDate::from_ymd(current_year, current_month, 1);
		let end_of_month = NaiveDate::from_ymd(current_year, current_month, DAY_COUNT[current_month as usize - 1] as u32);
		let weeks = end_of_month.iso_week().week0() - start_of_month.iso_week().week0() + 1;

		self.planner.week_button_states.clear();
		for _ in 0..weeks {
			self.planner.week_button_states.push(button::State::new());
		}
	}

	fn toggle_ingredient_acquired(&mut self, ingredient_index: usize) {
		if let Some(meal_index) = self.planned.meal_index {
			if let Some(ingredient) = self.planned.meals[meal_index].ingredients.get_mut(ingredient_index) {
				ingredient.acquired = !ingredient.acquired;
			}
		}
	}

	fn get_meal_planner(&mut self) -> Row<Message> {
		let remaining_width = 740 - (WEEK_SELECT_WIDTH + 40 + 25);

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
			.fold(scrollable, |prev, (index, (x, state))| {
				prev.push(
					Button::new(
						state,
						Container::new(
							Row::new()
								.width(Length::Fill)
								.push(
									Text::new(x.name.clone())
										.width(Length::Fill)
								)
								.push(
									Text::new(format!(
										"{} ingredient{}",
										x.ingredients.len(),
										if x.ingredients.len() != 1 {
											"s"
										} else {
											""
										}
									))
								)
						)
							.width(Length::Fill)
							.padding(10)
					)
						.on_press(Message::PlannedMealSelect(index))
						.style(style::DarkButton)
						.padding(0)
				)
				.push(Space::new(Length::Units(0), Length::Units(10)))
			});
		
		let week_buttons = self.planner.week_button_states.iter_mut();
		let day_buttons = self.planner.day_button_states.iter_mut();
		let right_panel = get_planner_right_panel(self.planner.state, week_buttons, day_buttons);

		Row::new()
			.push(scrollable)
			.push(
				Container::new(
					Container::new(right_panel)
						.height(Length::Units(constants::WINDOW_HEIGHT - 40))
						.width(Length::Units(WEEK_SELECT_WIDTH + 40))
						.padding([0, 20])
						.align_y(alignment::Vertical::Center)
						.style(style::MealsCalendarContainer)
				)
					.width(Length::Units(WEEK_SELECT_WIDTH + 40 + 35))
					.height(Length::Units(constants::WINDOW_HEIGHT))
					.padding([20, 20, 20, 5])
			)
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
		scrollable = self.planned.meals.iter()
			.zip(self.planned.meal_button_states.iter_mut())
			.enumerate()
			.fold(scrollable, |prev, (index, (x, state))| {
				prev.push(
					Button::new(
						state,
						Container::new(
							Row::new()
								.width(Length::Fill)
								.push(
									Text::new(format!("{}/{}/{}", x.date.day, x.date.month, x.date.year))
										.width(Length::Units(60))
								)
								.push(
									Text::new(x.recipe.name.clone())
										.width(Length::Fill)
								)
								.push(
									Text::new(
										x.ingredients.iter().fold("\u{e2e6}", |prev, y| {
											if prev == "\u{e836}" || y.acquired {
												prev
											} else {
												"\u{e836}"
											}
										})
									)
										.width(Length::Shrink)
										.font(constants::ICONS)
								)
						)
							.width(Length::Fill)
							.padding(10)
					)
						.on_press(Message::PlannedMealSelect(index))
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
			let selected_meal = &self.planned.meals[self.planned.meal_index.unwrap()];

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
					Text::new(selected_meal.recipe.name.clone())
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
				)
				.push(
					Space::new(Length::Units(0), Length::Units(0))
				);
			
			// put the ingredients into the information column
			information_column = selected_meal.ingredients.iter()
				.zip(self.planned.ingredient_button_states.iter_mut())
				.enumerate()
				.fold(information_column, |prev, (index, (x, state))| {
					prev.push(
						Button::new(
							state,
							Row::new()
								.push(
									Text::new(
										if x.acquired {
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
									Text::new(x.ingredient.name.clone())
								)
								.padding([10, 0, 0, 0])
						)
							.on_press(Message::PlannedIngredientSelect(index))
							.style(style::DarkButton)
							.padding(0)
					)
				});
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
			Message::MenuChange(_) => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;

				self.showing_planner = false;

				self.transition_planner_state(PlannerState::WeekSelect);

				Command::none()
			},
			Message::PlannedIngredientsScroll => {
				self.last_interaction = Some(Instant::now());
				self.planned.ingredients_state.set_force_disable(false);
				Command::none()
			},
			Message::PlannedIngredientSelect(index) => {
				self.toggle_ingredient_acquired(index);
				Command::none()
			},
			Message::PlannedMealsScroll(scroll) => {
				self.last_interaction = Some(Instant::now());
				self.planned.meals_position = scroll;
				self.planned.meals_state.set_force_disable(false);
				Command::none()
			},
			Message::PlannedMealSelect(index) => {
				self.select_planned_meal(index);
				self.planned.ingredients_state.snap_to_absolute(0.0);
				Command::none()
			},
			Message::PlannerDaySelect(index) => {
				self.planner.day_index = Some(index);
				self.transition_planner_state(PlannerState::MealSelect);
				Command::none()
			}
			Message::PlannerWeekSelect(index) => {
				self.planner.week_index = Some(index);
				self.transition_planner_state(PlannerState::DaySelect);
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

				self.transition_planner_state(PlannerState::WeekSelect);

				Command::none()
			},
			Message::SwitchToPlanner => {
				let size = (BUTTON_AREA_SIZE + BUTTON_HEIGHT + BUTTON_SPACING) as f32;
				
				self.planned.meals_state.snap_to_absolute(size);
				self.planned.meals_position = size;

				self.planner.recipes_state.snap_to_absolute(size);
				self.planner.recipes_position = size;
				
				self.showing_planner = true;

				self.transition_planner_state(PlannerState::WeekSelect);

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
						self.planned.meals_state.set_force_disable(true);
						self.planned.ingredients_state.set_force_disable(true);

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
