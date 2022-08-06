use iced::{ Column, Container, Element, Length, Row, Space, Text };

use crate::style;

#[derive(Debug)]
pub struct View {
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl View {
	pub fn new() -> Self {
		View {}
	}

	pub fn view(&mut self) -> Element<Message> {
		Container::new(
			Container::new(
				Column::new()
					.push(
						Row::new()
							.push(
								Text::new("5 dailies")
							)
							.push(Space::new(Length::Units(20), Length::Units(0)))
							.push(
								Text::new("3 weeklies")
							)
							.push(Space::new(Length::Units(20), Length::Units(0)))
							.push(
								Text::new("10T/12T")
							)
							.width(Length::Fill)
					)
					.push(
						Text::new("Idle")
					)
					.width(Length::Units(240))
			)
				.padding(10)
				.style(style::TodoItem)
		)
			.padding([20, 0, 0, 5])
			.into()
	}
}
