use std::sync::Arc;

use iced::alignment;
use iced::scrollable;
use iced::{ Column, Command, Container, Element, Length, Scrollable, Space, Text };

use chrono::{ Datelike, TimeZone, Utc, Weekday };

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
		let width = 400;
		if let None = self.todos {
			return Container::new(
				Container::new(Text::new(""))
					.width(Length::Units(width))
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
			.width(Length::Units(400))
			.height(Length::Fill)
			.padding([20, 20, 20, 0])
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

		for (_, day) in self.todos.as_ref().unwrap().database.mapping.iter() {
			// find the last valid index in the list
			let mut last_index = 0;
			let mut index = 0;
			for item in day.items.iter() {
				if item.description != "" {
					last_index = index;
				}
				index += 1;
			}
			
			index = 0;
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
									if index - 1 > last_index {
										acc
									} else {
										acc.push(
											Text::new(format!("{} ", item.description))
												.font(constants::NOTOSANS_THIN)
												.width(Length::Fill)
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
