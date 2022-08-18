use std::sync::Arc;

use bansheelong_types::{ Date, IO, PlannedMeal };
use chrono::{ Datelike, Local, NaiveDate };
use iced::{ Alignment, Button, Column, Container, Length, Row, Scrollable, Space, Text, alignment, button, image, scrollable };

use crate::constants;
use crate::meals::{ Message, PlannerState };
use crate::menu::BUTTON_HEIGHT;
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

const WEEK_SELECT_WIDTH: u16 = 7 * 35 + 6 * 4;
const WEEK_SELECT_DAY_SIZE: Length = Length::Units(35);
const WEEK_SELECT_DAY_SPACING: Length = Length::Units(4);

fn get_current_month() -> u32 {
	Local::now().month() as u32 - 1
}

fn get_current_year() -> u8 {
	(Local::now().year() - 2000) as u8
}

// returns right panel container and the remaining width for the left panel
pub(crate) fn get_planner_right_panel<'a, I>(
	state: PlannerState,
	year_index: u8,
	month_index: u32,
	day_buttons: I,
	previous_month_state: &'a mut button::State,
	next_month_state: &'a mut button::State,
	meal_add_state: &'a mut button::State,
	ingredients_state: &'a mut scrollable::State,
	recipe_index: Option<usize>,
	selected_date: Option<Date>,
	image: &'a image::Handle,
	image_state: &'a mut image::viewer::State,
	database: Arc<IO>
) -> (Container<'a, Message>, u16)
	where
		I: Iterator<Item = &'a mut button::State>
{
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

	// get previous indices
	let previous_month_index = if month_index == 0 {
		11
	} else {
		month_index - 1
	};

	let previous_year_index = if month_index == 0 {
		year_index - 1
	} else {
		year_index
	};

	// get next indices
	let next_month_index = if month_index == 11 {
		0
	} else {
		month_index + 1
	};

	let next_year_index = if month_index == 11 {
		year_index + 1
	} else {
		year_index
	};
	
	match state {
		PlannerState::DaySelect => {
			let mut month = Column::new()
				.push(
					Container::new(
						Row::new()
							.align_items(Alignment::Center)
							.push(
								Button::new(
									previous_month_state,
									Text::new(
										if month_index > get_current_month() || year_index != get_current_year() {
											"\u{e408}"
										} else {
											""
										}
									)
										.width(Length::Units(20))
										.horizontal_alignment(alignment::Horizontal::Center)
										.vertical_alignment(alignment::Vertical::Center)
										.size(25)
										.font(constants::ICONS)
								)
									.padding([0, 10])
									.style(style::DarkButton)
									.on_press(Message::PlannerMonthSelect(previous_year_index, previous_month_index))
							)
							.push(
								Underline::new(format!("{}", MONTH[month_index as usize]))
									.size(25)
									.font(constants::NOTOSANS_BOLD)
							)
							.push(
								Button::new(
									next_month_state,
									Text::new("\u{e409}")
										.width(Length::Units(20))
										.horizontal_alignment(alignment::Horizontal::Center)
										.vertical_alignment(alignment::Vertical::Center)
										.size(25)
										.font(constants::ICONS)
								)
									.padding([0, 10])
									.style(style::DarkButton)
									.on_press(Message::PlannerMonthSelect(next_year_index, next_month_index))
							)
					)
						.width(Length::Fill)
						.align_x(alignment::Horizontal::Center)
				)
				.push(Space::new(Length::Units(0), Length::Units(if weeks < 6 { 10 } else { 5 })))
				.width(Length::Units(WEEK_SELECT_WIDTH));

			let mut day: i8 = match NaiveDate::from_ymd(2000 + year_index as i32, month_index + 1, 1).weekday() {
				chrono::Weekday::Sun => 1,
				chrono::Weekday::Mon => 0,
				chrono::Weekday::Tue => -1,
				chrono::Weekday::Wed => -2,
				chrono::Weekday::Thu => -3,
				chrono::Weekday::Fri => -4,
				chrono::Weekday::Sat => -5,
			};

			let mut day_in_week = 0;
			let mut week = Row::new();

			for state in day_buttons {
				if day_in_week == 7 {
					month = month.push(week)
						.push(Space::new(Length::Units(0), WEEK_SELECT_DAY_SPACING));

					week = Row::new();
					day_in_week = 0;
				}

				let mut button = Button::new(
					state,
					Container::new(
						if day >= 1 && day <= DAY_COUNT[month_index as usize] {
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
					.style(style::DarkButton)
					.padding(0);
				
				if day >= 1 && day <= DAY_COUNT[month_index as usize] {
					button = button.on_press(Message::PlannerDaySelect(day));
				}

				week = week
					.push(button)
					.push(Space::new(WEEK_SELECT_DAY_SPACING, Length::Units(0)));

				day += 1;
				day_in_week += 1;
			}

			month = month.push(week)
				.push(Space::new(Length::Units(0), WEEK_SELECT_DAY_SPACING))
				.push(Space::new(Length::Units(0), Length::Units(10)));

			let container = Container::new(
				Container::new(month)
					.height(Length::Units(constants::WINDOW_HEIGHT - 40))
					.width(Length::Units(WEEK_SELECT_WIDTH + 40))
					.padding([0, 20])
					.align_y(alignment::Vertical::Center)
					.style(style::MealsCalendarContainer)
			)
				.width(Length::Units(WEEK_SELECT_WIDTH + 40 + 35))
				.height(Length::Units(constants::WINDOW_HEIGHT))
				.padding([20, 20, 20, 5]);

			return (container, constants::MENU_WIDTH - (WEEK_SELECT_WIDTH + 40 + 25));
		},
		PlannerState::MealSelect => {
			let mut information_column = Column::new();
			if recipe_index.is_none() {
				information_column = information_column.push(
					Space::new(Length::Units(0), Length::Units(constants::WINDOW_HEIGHT - 40 - 20))
				);
			} else {
				let selected_recipe = &database.meals_database.recipes[recipe_index.unwrap()];

				// construct information column that lets us select which ingredients we have
				information_column = information_column
					.push(
						image::Viewer::new(image_state, image.clone())
							.width(Length::Units(415))
					)
					.push(
						Space::new(Length::Units(0), Length::Units(5))
					)
					.push(
						Text::new(selected_recipe.name.clone())
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
				information_column = selected_recipe.ingredients.iter()
					.fold(information_column, |information_column, ingredient| {
						information_column.push(
							Row::new()
								.push(
									Text::new("-")
								)
								.push(
									Space::new(Length::Units(6), Length::Units(0))
								)
								.push(
									Text::new(ingredient.name.clone())
								)
								.padding([10, 0, 0, 0])
						)
					});
				
				if selected_recipe.preparation_steps.len() > 0 {
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
					information_column = selected_recipe.preparation_steps.iter()
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
				
				if selected_recipe.cooking_steps.len() > 0 {
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
					information_column = selected_recipe.cooking_steps.iter()
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
				
				information_column = information_column
					.push(
						Space::new(Length::Units(0), Length::Units(15))
					)
					.push(
						Button::new(
							meal_add_state,
							Text::new("Add meal to schedule")
								.width(Length::Fill)
								.horizontal_alignment(alignment::Horizontal::Center)
						)
							.style(style::TodoMenuButton)
							.width(Length::Fill)
							.height(Length::Units(BUTTON_HEIGHT))
							.on_press(Message::APIAddPlannedMeal(PlannedMeal::new(selected_date.unwrap(), selected_recipe.clone())))
					);
			}

			let scrollable = Scrollable::new(ingredients_state)
				.push(	
					Container::new(
						information_column
					)
						.width(Length::Fill)
						.padding(10)
						.style(style::TodoItem)
				)
				.on_scroll_absolute(move |_| Message::PlannerRecipeScroll)
				.width(Length::Units(435))
				.height(Length::Fill)
				.padding([20, 15, 20, 5])
				.style(style::TodoScrollable);

			(Container::new(scrollable), constants::MENU_WIDTH - 435)
		},
	}
}
