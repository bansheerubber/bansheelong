use std::env;
use std::cmp;

use iced::alignment;
use iced::button;
use iced::{ Alignment, Button, Column, Command, Container, Element, Length, Row, Svg, Text };

use crate::constants;
use crate::style;

#[derive(Debug)]
struct Instant {
	time: u16,
	temperature: u16,
}

impl Instant {
	fn get_temperature(&self) -> String {
		format!("{}°", self.temperature)
	}

	fn get_time(&self) -> String {
		if self.time > 12 {
			format!("{} PM", self.time - 12)
		} else if self.time == 0 {
			String::from("12 AM")
		} else {
			format!("{} AM", self.time)
		}
	}
}

#[derive(Debug)]
struct Status {
	current: Instant,
	day: String,
	humidity: u16,
	icon: String,
	times: [Instant; 3],
	uv: u16,
	wind: u16,
}

#[derive(Debug)]
pub struct View {
	current_status: isize,
	detail_toggle: bool,
	max_statuses: isize,
	statuses: [Status; 5],

	next_day_state: button::State,
	previous_day_state: button::State,
	toggle_detail_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
	Next,
	Previous,
	Refresh,
	ToggleDetail,
}

impl View {
	pub fn new() -> Self {
		View {
			current_status: 0,
			detail_toggle: false,
			max_statuses: 5,
			statuses: [
				Status {
					current: Instant {
						time: 0,
						temperature: 99,
					},
					day: String::from("Sunday"),
					humidity: 47,
					icon: String::from("clear-day.svg"),
					times: [
						Instant {
							time: (5 + 12),
							temperature: 98,
						},
						Instant {
							time: (9 + 12),
							temperature: 91,
						},
						Instant {
							time: 8,
							temperature: 86,
						},
					],
					uv: 10,
					wind: 12,
				},
				Status {
					current: Instant {
						time: 0,
						temperature: 100,
					},
					day: String::from("Monday"),
					humidity: 45,
					icon: String::from("clear-day.svg"),
					times: [
						Instant {
							time: (5 + 12),
							temperature: 100,
						},
						Instant {
							time: (9 + 12),
							temperature: 100,
						},
						Instant {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 10,
				},
				Status {
					current: Instant {
						time: 0,
						temperature: 100,
					},
					day: String::from("Teusday"),
					humidity: 45,
					icon: String::from("clear-day.svg"),
					times: [
						Instant {
							time: (5 + 12),
							temperature: 100,
						},
						Instant {
							time: (9 + 12),
							temperature: 100,
						},
						Instant {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 8,
				},
				Status {
					current: Instant {
						time: 0,
						temperature: 100,
					},
					day: String::from("Wednesday"),
					humidity: 45,
					icon: String::from("clear-day.svg"),
					times: [
						Instant {
							time: (5 + 12),
							temperature: 100,
						},
						Instant {
							time: (9 + 12),
							temperature: 100,
						},
						Instant {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 8,
				},
				Status {
					current: Instant {
						time: 0,
						temperature: 100,
					},
					day: String::from("Thursday"),
					humidity: 45,
					icon: String::from("clear-day.svg"),
					times: [
						Instant {
							time: (5 + 12),
							temperature: 100,
						},
						Instant {
							time: (9 + 12),
							temperature: 100,
						},
						Instant {
							time: 8,
							temperature: 100,
						},
					],
					uv: 11,
					wind: 7,
				},
			],
			next_day_state: button::State::new(),
			previous_day_state: button::State::new(),
			toggle_detail_state: button::State::new(),
		}
	}
	
	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Next => {
				self.current_status = cmp::min(self.current_status + 1, self.max_statuses - 1);
			},
			Message::Previous => {
				self.current_status = cmp::max(self.current_status - 1, 0);
			},
			Message::Refresh => {
				println!("refresh from api");
			},
			Message::ToggleDetail => {
				if self.current_status == 0 {
					self.detail_toggle = !self.detail_toggle;
				}
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let width = 402;

		let create_temp_time = |status: &Instant| {
			Column::new()
				.width(Length::Units(width / 3))
				.align_items(Alignment::Center)
				.push(
					Text::new(status.get_temperature())
						.size(50)
				)
				.push(
					Text::new(status.get_time())
						.size(20)
				)
		};

		let previous_day = Button::new(
			&mut self.previous_day_state,
			Text::new(if self.current_status != 0 { "\u{e408}" } else { "" })
				.width(Length::Units(40))
				.height(Length::Fill)
				.vertical_alignment(alignment::Vertical::Center)
				.size(60)
				.font(constants::ICONS)
		)
			.on_press(Message::Previous)
			.width(Length::Units(40))
			.height(Length::Fill)
			.padding(0)
			.style(style::WeatherButton);
	
		let next_day = Button::new(
			&mut self.next_day_state,
			Text::new(if self.current_status != self.max_statuses - 1 { "\u{e409}" } else { "" } )
				.width(Length::Units(40))
				.height(Length::Fill)
				.vertical_alignment(alignment::Vertical::Center)
				.size(60)
				.font(constants::ICONS)
		)
			.on_press(Message::Next)
			.width(Length::Units(40))
			.height(Length::Fill)
			.padding(0)
			.style(style::WeatherButton);
		
		let temperature_or_detailed = if self.current_status == 0 && !self.detail_toggle {
			Button::new(
				&mut self.toggle_detail_state,
				Row::new()
					.padding(0)
					.align_items(Alignment::Center)
					.width(Length::Fill)
					.push(
						create_temp_time(&self.statuses[self.current_status as usize].times[0])
					)
					.push(
						create_temp_time(&self.statuses[self.current_status as usize].times[1])
					)
					.push(
						create_temp_time(&self.statuses[self.current_status as usize].times[2])
					)
			)
				.on_press(Message::ToggleDetail)
				.padding(0)
				.style(style::WeatherButton)
		} else {
			Button::new(
				&mut self.toggle_detail_state,
				Row::new()
					.padding(0)
					.align_items(Alignment::Center)
					.width(Length::Fill)
					.push(
						Container::new(
							Svg::from_path(
								format!(
									"{}/data/uv-index-{}.svg",
									env::var("BANSHEELONG_DIR").unwrap(),
									self.statuses[self.current_status as usize].uv
								)
							)
								.width(Length::Units(70))
						)
							.width(Length::Units(width / 3))
							.align_x(iced::alignment::Horizontal::Center)
					)
					.push(
						Row::new()
							.width(Length::Units(width / 3))
							.align_items(Alignment::Center)
							.push(
								Svg::from_path(
									format!(
										"{}/data/raindrop.svg",
										env::var("BANSHEELONG_DIR").unwrap()
									)
								)
									.width(Length::Units(50))
									.height(Length::Units(70))
							)
							.push(
								Text::new(format!("{}%", self.statuses[self.current_status as usize].humidity))
									.size(40)
							)
					)
					.push(
						Row::new()
							.width(Length::Units(width / 3))
							.align_items(Alignment::Center)
							.push(
								Svg::from_path(
									format!(
										"{}/data/wind.svg",
										env::var("BANSHEELONG_DIR").unwrap()
									)
								)
									.width(Length::Units(70))
							)
							.push(
								Text::new(format!("{}", self.statuses[self.current_status as usize].wind))
									.size(40)
							)
					)
			)
				.on_press(Message::ToggleDetail)
				.padding(0)
				.style(style::WeatherButton)
		};
		
		Container::new(
			Column::new()
				.push( // weather icon & temperature & day & back/next buttons
					Row::new()
						.padding(0)
						.align_items(Alignment::Center)
						.push( // chevron left
							previous_day
						)
						.push( // weather icon
							Svg::from_path(
								format!(
									"{}/data/{}",
									env::var("BANSHEELONG_DIR").unwrap(),
									self.statuses[self.current_status as usize].icon
								)
							)
								.width(Length::Units(200))
								.height(Length::Units(200))
						)
						.push( // temperature & day
							Column::new()
								.padding(0)
								.align_items(Alignment::Start)
								.push(
									Text::new(self.statuses[self.current_status as usize].current.get_temperature())
										.size(70)
										.font(constants::NOTOSANS_BOLD)
								)
								.push(
									Container::new(
										Text::new(self.statuses[self.current_status as usize].day.to_string())
											.size(25)
									)
										.padding([0, 7])
								)
						)
						.push( // chevron right
							next_day
						)
						.height(Length::Units(200))
				)
				.push( // temperatures at times
					temperature_or_detailed
				)
		)
			.width(Length::Units(width))
			.height(Length::Units(constants::WINDOW_HEIGHT))
			.padding(0)
			.into()
	}
}
