use iced::{ Column, Command, Container, Element, Length, Row, Space, Text };

use bansheelong_types::JobFlags;

use crate::style;
use super::Data;

#[derive(Debug)]
pub struct View {
	data: Option<Data>,
	ellipses: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
	Received(Option<Data>),
	Tick,
}

impl View {
	pub fn new() -> Self {
		View {
			data: None,
			ellipses: 0,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Received(data) => {
				self.data = data;
				Command::none()
			},
			Message::Tick => {
				self.ellipses = (self.ellipses + 1) % 5;
				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let ellipses: String = std::iter::repeat(".").take(std::cmp::min((self.ellipses + 1) as usize, 4)).collect();
		
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
								String::from("Error")
							} else {
								if self.data.as_ref().unwrap().job_flags.contains(JobFlags::CREATING_MONTHLY) {
									String::from("Creating monthly backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobFlags::CREATING_WEEKLY) {
									String::from("Creating weekly backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobFlags::DOWNLOADING_DAILY) {
									String::from("Downloading daily backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobFlags::SYNCING_GITHUB) {
									String::from("Syncing GitHub to backup") + &ellipses
								} else {
									String::from("Idle")
								}
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
