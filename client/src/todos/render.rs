use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{ Duration, Instant };

use bansheelong_shared_ui::{ Underline, constants, style };
use bansheelong_types::{ Date, Day, IO, PlannedMeal };
use chrono::{ Datelike, Local, TimeZone, Utc, Weekday };
use iced::{ Button, Column, Command, Container, Element, Length, Row, Scrollable, Space, Text, alignment, button, scrollable };

use crate::menu::MENU_STATE;
use crate::state::{ VALID_STARTING_CHARACTERS, WINDOW_STATE };

#[derive(Debug)]
pub struct View {
	button_states: Vec<button::State>,
	database: Option<Arc<IO>>,
	last_interaction: Option<Instant>,
	scrollable_state: scrollable::State,
	scroll_position: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
	MenuChange(constants::Menu),
	Scroll(f32),
	Tick,
	Update(Option<Arc<IO>>),
}

impl View {
	pub fn new() -> Self {
		let scroll_position = MENU_STATE.get_area_size() as f32;
		
		let mut scrollable_state = scrollable::State::new();
		scrollable_state.snap_to_absolute(scroll_position);
		View {
			button_states: vec![button::State::new(); MENU_STATE.button_count as usize],
			database: None,
			last_interaction: None,
			scrollable_state,
			scroll_position,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::MenuChange(_) => {
				self.scrollable_state.snap_to_absolute(MENU_STATE.get_area_size() as f32);
				self.scroll_position = MENU_STATE.get_area_size() as f32;
				Command::none()
			},
			Message::Scroll(scroll) => {
				self.last_interaction = Some(Instant::now());
				self.scroll_position = scroll;
				self.scrollable_state.set_force_disable(false);
				Command::none()
			},
			Message::Tick => {
				if self.last_interaction.is_some() {
					if Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(2)
						&& self.scroll_position < MENU_STATE.get_area_size() as f32
					{
						self.scrollable_state.snap_to_absolute(MENU_STATE.get_area_size() as f32);
						self.scroll_position = MENU_STATE.get_area_size() as f32;
					}

					if Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(4) {
						self.scrollable_state.set_force_disable(true);
					}
				}

				Command::none()
			},
			Message::Update(io) => {
				self.database = io;
				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let color_amount = 4;
		let get_todo_circle = |index| -> Box<dyn iced::container::StyleSheet> {
			match index {
				0 => Box::new(style::TodoCircleBlue),
				1 => Box::new(style::TodoCircleMagenta),
				2 => Box::new(style::TodoCircleGreen),
				3 => Box::new(style::TodoCircleCyan),
				_ => Box::new(style::TodoCircleBlue),
			}
		};

		let time = Local::now();
		let current_date = Some(bansheelong_types::Date {
			day: time.day() as u8,
			month: time.month() as u8,
			year: (time.year() % 100) as u8,
		});
		
		let width = 385;
		if let None = self.database {
			return Container::new(
				Container::new(Text::new(""))
					.width(Length::Units(width - 15))
					.height(Length::Units(250))
					.style(style::BlankWeatherContainer)
			)
				.width(Length::Units(width))
				.height(Length::Units(WINDOW_STATE.height))
				.style(style::WeatherContainer)
				.align_y(alignment::Vertical::Center)
				.into()
		}

		let mut scrollable = Scrollable::new(&mut self.scrollable_state)
			.width(Length::Units(width))
			.height(Length::Fill)
			.padding([20, 15, 20, 0])
			.style(style::TodoScrollable)
			.on_scroll_absolute(move |offset| Message::Scroll(offset))
			.min_height((MENU_STATE.get_area_size() + WINDOW_STATE.height) as u32);

		// add buttons to top button menu thing
		scrollable = scrollable.push(
			self.button_states
			.iter_mut()
			.zip(MENU_STATE.buttons.iter())
			.fold(
				Column::new()
					.spacing(MENU_STATE.button_spacing)
					.padding([0, 0, 20, 0]),
				|button_column, (state, (name, menu_type))| {
					if menu_type != &constants::Menu::Todos {
						button_column.push(
							Button::new(
								state,
								Text::new(name.clone())
									.width(Length::Fill)
									.horizontal_alignment(alignment::Horizontal::Center)
							)
								.style(style::TodoMenuButton)
								.width(Length::Fill)
								.height(Length::Units(MENU_STATE.button_height))
								.on_press(Message::MenuChange(menu_type.clone()))
						)
					} else {
						button_column
					}
				}
			)
		);

		let date_to_ui = |date: Option<bansheelong_types::Date>| {
			if let Some(d) = date {
				let abbreviation = match Utc.ymd(2000 + d.year as i32, d.month as u32, d.day as u32)
					.and_hms(0, 0, 0).date().weekday()
				{
					Weekday::Mon => "m",
					Weekday::Tue => "t",
					Weekday::Wed => "w",
					Weekday::Thu => "th",
					Weekday::Fri => "f",
					Weekday::Sat => "s",
					Weekday::Sun => "su",
				};

				Underline::new(format!("{}/{}/{}({}):", d.month, d.day, d.year, abbreviation))
					.font(constants::NOTOSANS_THIN)
			} else {
				Underline::new("General list")
			}
		};

		let has_time_day = |item: &bansheelong_types::Item| {
			item.time.is_some() && item.time.unwrap().day.is_some()
		};

		let mut new_mapping: BTreeMap<Option<Date>, (Option<&Day>, Option<&PlannedMeal>)> = BTreeMap::new();
		for (date, day) in self.database.as_ref().unwrap().todos_database.mapping.iter() {
			new_mapping.insert(date.clone(), (Some(day), None));
		}

		for (date, meal) in self.database.as_ref().unwrap().meals_database.planned_meal_mapping.iter() {
			if let Some(entry) = new_mapping.get_mut(&Some(date.clone())) {
				entry.1 = Some(meal);
			} else {
				new_mapping.insert(Some(date.clone()), (None, Some(meal)));
			}
		}

		for (date, (day, meal)) in new_mapping.iter() {
			let mut column = Column::new()
				.push(
					date_to_ui(*date)
				)
				.push(
					Space::new(Length::Units(0), Length::Units(5))
				)
				.width(Length::Fill);
			
			if let Some(day) = day {
				// find the last valid index in the list
				let mut last_index = -1;
				let mut index = 0;
				for item in day.items.iter() {
					if item.description != "" && !has_time_day(&item) {
						last_index = index;
					}
					index += 1;
				}

				if last_index == -1 && meal.is_none() {
					continue;
				}

				index = 0;
				let mut color_index = 0;

				column = day.items.iter().fold(
					column,
					|acc, item| {
						index += 1;
						if index - 1 > last_index || has_time_day(&item) {
							acc
						} else {
							let circle_or_dash = if item.time.is_some() && day.date == current_date {
								color_index += 1;
								Container::new(
									Container::new(Space::new(Length::Units(0), Length::Units(0)))
										.style(get_todo_circle((color_index - 1) % color_amount))
										.width(Length::Units(7))
										.height(Length::Units(7))
								)
									.width(Length::Units(18))
									.align_x(alignment::Horizontal::Center)
									.padding([7, 4, 0, 0])
							} else if item.description != "" {
								let start_character = if VALID_STARTING_CHARACTERS.contains(&item.description.chars().nth(0).unwrap()) {
									String::from(item.description.chars().nth(0).unwrap())
								} else {
									String::from("-")
								};
								
								Container::new(Text::new(start_character))
									.width(Length::Units(18))
									.align_x(alignment::Horizontal::Center)
									.padding([0, 4, 0, 0])
							} else {
								Container::new(Space::new(Length::Units(0), Length::Units(0)))
							};

							acc.push(
								Row::new()
									.push(
										circle_or_dash
									)
									.push(
										Text::new(format!(
											"{} ",
											if VALID_STARTING_CHARACTERS.contains(&item.description.chars().nth(0).unwrap())
												&& item.description.chars().nth(1).unwrap() == ' '
											{
												String::from(&item.description[2..])
											} else {
												item.description.clone()
											}
										))
											.font(constants::NOTOSANS_THIN)
											.width(Length::Fill)
									)
							)
						}
					}
				);
			}

			if let Some(meal) = meal {
				let acquired_ingredient_count = meal.ingredients.iter()
					.fold(0, |prev, planned_ingredient| {
						if planned_ingredient.acquired {
							prev + 1
						} else {
							prev
						}
					});

				let character = if acquired_ingredient_count == meal.ingredients.len() {
					"#"
				} else {
					"!"
				};

				// add meal if there is one
				column = column.push(
					Row::new()
						.push(
							Container::new(Text::new(character))
								.width(Length::Units(18))
								.align_x(alignment::Horizontal::Center)
								.padding([0, 4, 0, 0])
						)
						.push(
							Text::new(format!("{} ", meal.recipe.name.clone()))
								.font(constants::NOTOSANS_THIN)
								.width(Length::Fill)
						)
				)
			}

			scrollable = scrollable.push(
				Container::new(
					Container::new(column)
						.width(Length::Fill)
						.style(style::TodoItem)
						.padding(10)
				)
					.width(Length::Fill)
					.style(style::TodoItemContainer)
					.padding([0, 0, 10, 0])
			);
		}

		return scrollable.into();
	}
}
