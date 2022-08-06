use std::sync::Arc;

use iced::alignment;
use iced::scrollable;
use iced::{ Column, Command, Container, Element, Length, Row, Scrollable, Space, Text };

use chrono::{ Datelike, Local, TimeZone, Utc, Weekday };

use bansheelong_types::IO;

use crate::constants;
use crate::todos::Date;
use crate::style;

#[derive(Debug)]
pub struct View {
	scrollable_state: scrollable::State,
	todos: Option<Arc<IO>>,
}

#[derive(Debug, Clone)]
pub enum Message {
	Update(Option<Arc<IO>>),
}

impl View {
	pub fn new() -> Self {
		View {
			scrollable_state: scrollable::State::new(),
			todos: None,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Update(io) => {
				self.todos = io;
				Command::none()
			}
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
		if let None = self.todos {
			return Container::new(
				Container::new(Text::new(""))
					.width(Length::Units(width - 20))
					.height(Length::Units(250))
					.style(style::BlankWeatherContainer)
			)
				.width(Length::Units(width))
				.height(Length::Units(constants::WINDOW_HEIGHT))
				.style(style::WeatherContainer)
				.align_y(alignment::Vertical::Center)
				.into()
		}

		let mut scrollable = Scrollable::new(&mut self.scrollable_state)
			.width(Length::Units(width))
			.height(Length::Fill)
			.padding([20, 15, 20, 0])
			.style(style::TodoScrollable);

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

				Date::new(format!("{}/{}/{}({}):", d.month, d.day, d.year, abbreviation))
					.font(constants::NOTOSANS_THIN)
			} else {
				Date::new("General list")
			}
		};

		let has_time_day = |item: &bansheelong_types::Item| {
			item.time.is_some() && item.time.unwrap().day.is_some()
		};

		for (_, day) in self.todos.as_ref().unwrap().database.mapping.iter() {
			// find the last valid index in the list
			let mut last_index = 0;
			let mut index = 0;
			for item in day.items.iter() {
				if item.description != "" && !has_time_day(&item) {
					last_index = index;
				}
				index += 1;
			}

			if last_index == 0 {
				continue;
			}
			
			index = 0;
			let mut color_index = 0;
			scrollable = scrollable.push(
				Container::new(
					Container::new(
							day.items.iter().fold(
								Column::new()
									.push(
										date_to_ui(day.date)
									)
									.push(
										Space::new(Length::Units(0), Length::Units(5))
									)
									.width(Length::Fill),
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
												.width(Length::Units(10))
												.align_x(alignment::Horizontal::Center)
												.padding([7, 4, 0, 0])
										} else if item.description != "" {
											Container::new(Text::new("-"))
												.width(Length::Units(10))
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
													Text::new(format!("{} ", item.description.clone().replace("- ", "")))
														.font(constants::NOTOSANS_THIN)
														.width(Length::Fill)
												)
										)
									}
								}
							)
					)
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
