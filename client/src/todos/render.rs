use iced::scrollable;
use iced::{ Column, Command, Container, Element, Length, Scrollable, Space, Text };

use bansheelong_types::{ Database, Error, IO, Resource, read_database };

use crate::constants;
use crate::todos::date::Date;
use crate::style;

#[derive(Debug)]
pub struct View {
	scrollable_state: scrollable::State,
	todos: IO,
}

#[derive(Debug, Clone)]
pub enum Message {
	Fetched(Result<Database, Error>),
	Refresh,
}

impl View {
	pub fn new(resource: Resource) -> Self {
		View {
			scrollable_state: scrollable::State::new(),
			todos: IO {
				resource,
				..IO::default()
			},
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Fetched(result) => {
				if let Err(error) = result {
					println!("{:?}", error);
				} else if let Ok(database) = result {
					self.todos.database = database;
				}
				Command::none()
			},
			Message::Refresh => {
				Command::perform(read_database(self.todos.resource.clone()), Message::Fetched)
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scrollable_state)
			.width(Length::Units(400))
			.height(Length::Fill)
			.padding([20, 20, 20, 0])
			.style(style::TodoScrollable);

		let date_to_ui = |date: Option<bansheelong_types::Date>| {
			if let Some(d) = date {
				Date::new(format!("{}/{}/{}({}):", d.month, d.day, d.year, "fu"))
					.font(constants::NOTOSANS_THIN)
			} else {
				Date::new("General list")
			}
		};

		for (_, day) in self.todos.database.mapping.iter() {
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
					.padding([5, 0])
			);
		}

		return scrollable.into();
	}
}
