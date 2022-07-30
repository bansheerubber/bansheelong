use iced::scrollable;
use iced::{ Column, Container, Element, Length, Scrollable, Space, Text };

use crate::constants;
use crate::todos::date::Date;
use crate::todos::types;
use crate::style;

#[derive(Debug)]
pub struct View {
	scrollable_state: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
	
}

impl View {
	pub fn new() -> Self {
		View {
			scrollable_state: scrollable::State::new(),
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scrollable_state)
			.width(Length::Units(400))
			.height(Length::Fill)
			.padding([20, 20, 20, 0])
			.style(style::TodoScrollable);
		
		let test = vec![
			types::Day {
				items: vec![
					types::Item {
						description: String::from("do the laundry"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("eat the hamburger"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("jump up and down"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
				],
				day: 1,
				month: 5,
				year: 22,
				day_name: String::from("su"),
			},
			types::Day {
				items: vec![
					types::Item {
						description: String::from("do the laundry"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("eat the hamburger"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("jump up and down"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
				],
				day: 1,
				month: 5,
				year: 22,
				day_name: String::from("eg"),
			},
			types::Day {
				items: vec![
					types::Item {
						description: String::from("do the frogly"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("eat the hamburger"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
					types::Item {
						description: String::from("jump up and down"),
						time: types::Time {
							hour: 0,
							minute: 0,
						}
					},
				],
				day: 1,
				month: 5,
				year: 22,
				day_name: String::from("pu"),
			},
		];
		
		for day in test {
			scrollable = scrollable.push(
				Container::new(
					Container::new(
							day.items.iter().fold(
								Column::new()
									.push(
										Date::new(format!("{}/{}/{}({}):", day.month, day.day, day.year, day.day_name))
											.font(constants::NOTOSANS_THIN)
									)
									.push(
										Space::new(Length::Units(0), Length::Units(5))
									)
									.width(Length::Fill),
								|acc, item| acc.push(
									Text::new(format!("- {}", item.description))
										.font(constants::NOTOSANS_THIN)
										.width(Length::Fill)
								)
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
