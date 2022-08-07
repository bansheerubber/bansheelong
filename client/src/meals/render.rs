use std::time::{ Duration, Instant };

use iced::{ Button, Column, Command, Container, Element, Length, Row, Scrollable, Space, Text, alignment, button, scrollable };

use chrono::{ Datelike, Local, NaiveDate };

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

#[derive(Debug)]
pub struct View {
	button_states: [button::State; BUTTON_COUNT as usize],
	current_meals_state: scrollable::State,
	current_meals_position: f32,
	last_interaction: Option<Instant>,
	meal_list_state: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
	CurrentMealsScroll(f32),
	MenuChange(Menu),
	Tick,
}

impl View {
	pub fn new() -> Self {
		let scroll_position = BUTTON_AREA_SIZE as f32;
		
		let mut scrollable_state = scrollable::State::new();
		scrollable_state.snap_to_absolute(scroll_position);
		View {
			button_states: [button::State::new(); BUTTON_COUNT as usize],
			current_meals_state: scrollable_state,
			current_meals_position: scroll_position,
			last_interaction: None,
			meal_list_state: scrollable::State::new(),
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::CurrentMealsScroll(scroll) => {
				self.last_interaction = Some(Instant::now());
				self.current_meals_position = scroll;
				Command::none()
			},
			Message::MenuChange(_) => {
				self.current_meals_state.snap_to_absolute(BUTTON_AREA_SIZE as f32);
				self.current_meals_position = BUTTON_AREA_SIZE as f32;
				Command::none()
			},
			Message::Tick => {
				if self.last_interaction.is_some()
					&& Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(2)
					&& self.current_meals_position < BUTTON_AREA_SIZE as f32
				{
					self.current_meals_state.snap_to_absolute(BUTTON_AREA_SIZE as f32);
					self.current_meals_position = BUTTON_AREA_SIZE as f32;
				}

				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		// let size = 35;
		// let spacing = 4;
		// let width = 7 * size + 6 * spacing;

		// let size = Length::Units(size);
		// let spacing = Length::Units(spacing);

		// let time = Local::now();
		// let current_month = time.month();
		// let current_year = time.year();

		// let start_of_month = NaiveDate::from_ymd(current_year, current_month, 1);
		// let end_of_month = NaiveDate::from_ymd(current_year, current_month, DAY_COUNT[current_month as usize - 1] as u32);
		// let weeks = end_of_month.iso_week().week0() - start_of_month.iso_week().week0() + 1;

		// // day picker
		// let mut month = Column::new()
		// 	.push(
		// 		Container::new(
		// 			Underline::new(format!(" {} ", MONTH[current_month as usize - 1]))
		// 				.size(25)
		// 				.font(constants::NOTOSANS_BOLD)
		// 		)
		// 			.width(Length::Fill)
		// 			.align_x(alignment::Horizontal::Center)
		// 	)
		// 	.push(Space::new(Length::Units(0), Length::Units(if weeks < 6 { 10 } else { 5 })))
		// 	.width(Length::Units(width));

		// let mut day: i8 = match start_of_month.weekday() {
		// 	chrono::Weekday::Sun => 1,
		// 	chrono::Weekday::Mon => 0,
		// 	chrono::Weekday::Tue => -1,
		// 	chrono::Weekday::Wed => -2,
		// 	chrono::Weekday::Thu => -3,
		// 	chrono::Weekday::Fri => -4,
		// 	chrono::Weekday::Sat => -5,
		// };

		// for _ in 0..weeks {
		// 	let mut week = Row::new();
		// 	for _ in 0..7 {
		// 		week = week.push(
		// 			Container::new(
		// 				if day >= 1 && day <= DAY_COUNT[current_month as usize - 1] {
		// 					Text::new(day.to_string())
		// 						.size(18)
		// 				} else {
		// 					Text::new("")
		// 				}
		// 			)
		// 				.padding([0, 0, 0, 3])
		// 				.width(size)
		// 				.height(size)
		// 				.style(style::MealsDayContainer)
		// 		)
		// 		.push(Space::new(spacing, Length::Units(0)));

		// 		day += 1;
		// 	}

		// 	month = month
		// 		.push(week)
		// 		.push(Space::new(Length::Units(0), spacing));
		// }

		// month = month.push(Space::new(Length::Units(0), Length::Units(10)));

		// let get_meal_entry = || {
		// 	Container::new(
		// 		Text::new("orange chicken")
		// 			.size(30)
		// 	)
		// 		.padding([5, 0])
		// };

		// // meal list
		// let meal_list = Column::new()
		// 	.push(
		// 		Underline::new("Meal list")
		// 	)
		// 	.push(Space::new(Length::Units(0), Length::Units(5)))
		// 	.push(
		// 		Scrollable::new(&mut self.meal_list_state)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.push(
		// 				get_meal_entry()
		// 			)
		// 			.width(Length::Fill)
		// 			.style(style::TodoScrollable)
		// 	);

		// let remaining_width = 740 - (width + 40 + 35);

		// Row::new()
		// 	.push(
		// 		Container::new(
		// 			Container::new(meal_list)
		// 				.height(Length::Units(constants::WINDOW_HEIGHT - 40))
		// 				.width(Length::Units(remaining_width))
		// 				.padding([15, 10, 10, 20])
		// 				.style(style::MealsCalendarContainer)
		// 		)
		// 			.width(Length::Units(remaining_width))
		// 			.height(Length::Units(constants::WINDOW_HEIGHT))
		// 			.padding([20, 0])
		// 	)
		// 	.push(
		// 		Container::new(
		// 			Container::new(month)
		// 				.height(Length::Units(constants::WINDOW_HEIGHT - 40))
		// 				.width(Length::Units(width + 40))
		// 				.padding([0, 20])
		// 				.align_y(alignment::Vertical::Center)
		// 				.style(style::MealsCalendarContainer)
		// 		)
		// 			.width(Length::Units(width + 40 + 35))
		// 			.height(Length::Units(constants::WINDOW_HEIGHT))
		// 			.padding([20, 15, 20, 20])
		// 	)
		// 	.into()

		let mut scrollable = Scrollable::new(&mut self.current_meals_state)
			.width(Length::Units(300))
			.height(Length::Fill)
			.padding([20, 15, 20, 0])
			.style(style::TodoScrollable)
			.on_scroll_absolute(move |offset| Message::CurrentMealsScroll(offset))
			.min_height((BUTTON_AREA_SIZE + constants::WINDOW_HEIGHT) as u32);

		// add buttons to top button menu thing
		scrollable = scrollable.push(
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
		);

		scrollable = scrollable.push(
			Container::new(
				Container::new(
					Row::new()
						.width(Length::Fill)
						.push(
							Text::new("8/7/22")
								.width(Length::Units(60))
						)
						.push(
							Text::new("Orange chicken")
								.width(Length::Fill)
						)
						.push(
							Text::new("\u{e2e6}")
								.width(Length::Shrink)
								.font(constants::ICONS)
						)
				)
					.width(Length::Fill)
					.padding(10)
					.style(style::TodoItem)
			)
				.padding([0, 0, 10, 0])
		);

		scrollable = scrollable.push(
			Container::new(
				Container::new(
					Row::new()
						.width(Length::Fill)
						.push(
							Text::new("8/7/22")
								.width(Length::Units(60))
						)
						.push(
							Text::new("Orange chicken")
								.width(Length::Fill)
						)
						.push(
							Text::new("\u{e2e6}")
								.width(Length::Shrink)
								.font(constants::ICONS)
						)
				)
					.width(Length::Fill)
					.padding(10)
					.style(style::TodoItem)
			)
				.padding([0, 0, 10, 0])
		);

		Row::new()
			.push(
				scrollable
			)
			.push(
				Space::new(Length::Units(5), Length::Units(0))
			)
			.push(
				Container::new(
					Container::new(Text::new("the ingredients"))
						.width(Length::Fill)
						.height(Length::Fill)
						.style(style::TodoItem)
				)
					.width(Length::Units(435))
					.height(Length::Fill)
					.padding([20, 15, 20, 0])
			)
			.into()
	}
}
