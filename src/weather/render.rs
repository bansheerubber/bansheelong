use iced::alignment::Horizontal;
use iced::{ Alignment, Column, Command, Container, Element, Length, Row, Settings, Svg, Text };
use std::env;

use crate::constants;

#[derive(Debug)]
pub struct View {

}

#[derive(Debug, Clone)]
pub enum Message {

}

impl View {
	pub fn new() -> Self {
		View {

		}
	}

	pub fn view(&mut self) -> Element<()> {
		let width = 360;

		let create_temp_time = |temp: String, time: String| {
			Column::new()
				.width(Length::Units(width / 3))
				.align_items(Alignment::Center)
				.push(
					Text::new(format!("{}°", temp))
						.size(50)
				)
				.push(
					Text::new(time)
						.size(20)
				)
		};
		
		Container::new(
			Column::new()
				.push(
					Row::new()
						.padding(0)
						.align_items(Alignment::Center)
						.push(
							Svg::from_path(
								format!(
									"{}/data/clear-day.svg",
									env::var("BANSHEELONG_DIR").unwrap()
								)
							)
								.width(Length::Units(200))
								.height(Length::Units(200))
						)
						.push(
							Column::new()
								.padding(0)
								.align_items(Alignment::Start)
								.push(
									Text::new("100°")
										.size(70)
										.font(constants::NOTOSANS_BOLD)
								)
								.push(
									Container::new(
										Text::new("Monday")
											.size(25)
									)
										.padding([0, 7])
								)
						)
				)
				.push(
					Row::new()
						.padding(0)
						.align_items(Alignment::Center)
						.width(Length::Fill)
						.push(
							create_temp_time(String::from("100"), String::from("5 PM"))
						)
						.push(
							create_temp_time(String::from("100"), String::from("9 PM"))
						)
						.push(
							create_temp_time(String::from("100"), String::from("8 AM"))
						)
				)
		)
			.width(Length::Units(width))
			.height(Length::Units(constants::WINDOW_HEIGHT))
			.padding(0)
			.into()
	}
}
