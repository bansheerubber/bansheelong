use iced::{ Column, Command, Container, Element, Length, Row, Space, Text };

use crate::style;
use super::Data;

#[derive(Debug)]
pub struct View {
	data: Option<Data>,
}

#[derive(Debug, Clone)]
pub enum Message {
	Received(Option<Data>),
}

impl View {
	pub fn new() -> Self {
		View {
			data: None,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		let Message::Received(data) = message;
		self.data = data;

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		Container::new(
			Container::new(
				Column::new()
					.push(
						Row::new()
							.push(
								Text::new(
									format!(
										"{} dailies",
										if let None = self.data {
											0
										} else {
											self.data.as_ref().unwrap().dailies
										}
									)
								)
							)
							.push(Space::new(Length::Units(20), Length::Units(0)))
							.push(
								Text::new(
									format!(
										"{} weeklies",
										if let None = self.data {
											0
										} else {
											self.data.as_ref().unwrap().weeklies
										}
									)
								)
							)
							.push(Space::new(Length::Units(20), Length::Units(0)))
							.push(
								Text::new(
									format!(
										"{}T/{}T",
										if let None = self.data { // used size
											0
										} else {
											self.data.as_ref().unwrap().used_size / 1000000000000
										},
										if let None = self.data { // total size
											0
										} else {
											self.data.as_ref().unwrap().total_size / 1000000000000
										}
									)
								)
							)
							.width(Length::Fill)
					)
					.push(
						Text::new(
							if self.data.is_none() || self.data.as_ref().unwrap().has_zpool_error {
								"Error"
							} else {
								"Idle"
							}
						)
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
