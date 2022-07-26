use std::cmp;
use std::time::{ Duration, Instant };

use iced::alignment;
use iced::button;
use iced::{ Alignment, Button, Column, Command, Container, Element, Length, Row, Svg, Text };

use crate::constants::{ ICONS, NOTOSANS_BOLD, WINDOW_HEIGHT, get_directory };
use crate::style;
use crate::weather::api;
use crate::weather::types::{ OneAPIError, OneAPIResponse };

#[derive(Debug)]
struct TemperatureDatum {
	time: u16,
	temperature: u16,
}

impl TemperatureDatum {
	fn get_temperature(&self) -> String {
		format!("{}°", self.temperature)
	}

	fn get_time(&self) -> String {
		if self.time == 12 {
			format!("{} PM", self.time)
		} else if self.time == 0 {
			String::from("12 AM")
		} else if self.time > 12 {
			format!("{} PM", self.time - 12)
		} else {
			format!("{} AM", self.time)
		}
	}
}

#[derive(Debug)]
struct DailyStatus {
	current: TemperatureDatum,
	day: String,
	humidity: u16,
	icon: String,
	times: [TemperatureDatum; 3],
	uv: u16,
	wind: u16,
}

#[derive(Debug)]
pub struct View {
	api_data: Option<OneAPIResponse>,
	current_status: isize,
	detail_toggle: bool,
	last_interaction: Instant,
	max_statuses: isize,
	statuses: [DailyStatus; 5],

	next_day_state: button::State,
	previous_day_state: button::State,
	toggle_detail_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
	Fetched(Result<OneAPIResponse, OneAPIError>),
	Next,
	Previous,
	Refresh,
	Tick,
	ToggleDetail,
}

impl View {
	pub fn new() -> Self {
		View {
			api_data: None,
			current_status: 0,
			detail_toggle: false,
			last_interaction: Instant::now(),
			max_statuses: 5,
			statuses: [
				DailyStatus {
					current: TemperatureDatum {
						time: 0,
						temperature: 99,
					},
					day: String::from("Sunday"),
					humidity: 47,
					icon: String::from("clear-day"),
					times: [
						TemperatureDatum {
							time: (5 + 12),
							temperature: 98,
						},
						TemperatureDatum {
							time: (9 + 12),
							temperature: 91,
						},
						TemperatureDatum {
							time: 8,
							temperature: 86,
						},
					],
					uv: 10,
					wind: 12,
				},
				DailyStatus {
					current: TemperatureDatum {
						time: 0,
						temperature: 100,
					},
					day: String::from("Monday"),
					humidity: 45,
					icon: String::from("clear-day"),
					times: [
						TemperatureDatum {
							time: (5 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: (9 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 10,
				},
				DailyStatus {
					current: TemperatureDatum {
						time: 0,
						temperature: 100,
					},
					day: String::from("Teusday"),
					humidity: 45,
					icon: String::from("clear-day"),
					times: [
						TemperatureDatum {
							time: (5 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: (9 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 8,
				},
				DailyStatus {
					current: TemperatureDatum {
						time: 0,
						temperature: 100,
					},
					day: String::from("Wednesday"),
					humidity: 45,
					icon: String::from("clear-day"),
					times: [
						TemperatureDatum {
							time: (5 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: (9 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: 8,
							temperature: 100,
						},
					],
					uv: 10,
					wind: 8,
				},
				DailyStatus {
					current: TemperatureDatum {
						time: 0,
						temperature: 100,
					},
					day: String::from("Thursday"),
					humidity: 45,
					icon: String::from("clear-day"),
					times: [
						TemperatureDatum {
							time: (5 + 12),
							temperature: 100,
						},
						TemperatureDatum {
							time: (9 + 12),
							temperature: 100,
						},
						TemperatureDatum {
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
	
	pub fn update_from_api(&mut self) {
		if let Some(data) = &self.api_data {
			// data.current.temp
			self.statuses[0].current.temperature = data.current.temp as u16;
			self.statuses[0].current.time = 0;
			self.statuses[0].day = api::convert_to_time(data.current.dt).format("%A").to_string();
			self.statuses[0].humidity = data.current.humidity;
			self.statuses[0].icon = api::decode_icon(data.current.weather[0].id, data.current.clouds > 50, true);
			self.statuses[0].uv = std::cmp::max(std::cmp::min(data.current.uvi as u16, 11), 1);
			self.statuses[0].wind = data.current.wind_speed as u16;

			let mut next_time: u16 = (api::convert_to_time(data.current.dt).format("%H").to_string().parse::<u16>().unwrap() + 1) % 24;
			let mut index = 0;
			let mut status_index = 0;
			while status_index < 3 {
				let time: u16 = api::convert_to_time(data.hourly[index].dt).format("%H").to_string().parse().unwrap();
				if time == next_time {
					self.statuses[0].times[status_index].temperature = data.hourly[index].temp as u16;
					self.statuses[0].times[status_index].time = time;
					next_time = (time + 3) % 24;
					status_index += 1;
				}
				index += 1;
			}

			for i in 1..5 {
				self.statuses[i].current.temperature = data.daily[i].temp.max as u16;
				self.statuses[i].current.time = 0;
				self.statuses[i].day = api::convert_to_time(data.daily[i].dt).format("%A").to_string();
				self.statuses[i].humidity = data.daily[i].humidity;
				self.statuses[i].icon = api::decode_icon(data.daily[i].weather[0].id, data.daily[i].clouds > 50, true);
				self.statuses[i].uv = std::cmp::max(std::cmp::min(data.daily[i].uvi as u16, 11), 1);
				self.statuses[i].wind = data.daily[i].wind_speed as u16;

				self.statuses[i].times[0].temperature = data.daily[i].temp.morn as u16;
				self.statuses[i].times[0].time = 8;

				self.statuses[i].times[1].temperature = data.daily[i].temp.day as u16;
				self.statuses[i].times[1].time = 12;

				self.statuses[i].times[2].temperature = data.daily[i].temp.night as u16;
				self.statuses[i].times[2].time = 12 + 8;
			}
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Fetched(result) => {
				if let Ok(data) = result {
					self.api_data = Some(data);
				} else {
					self.api_data = None;
					println!("{:?}", result.err());
				}
				self.update_from_api();
			},
			Message::Next => {
				self.current_status = cmp::min(self.current_status + 1, self.max_statuses - 1);
				self.last_interaction = Instant::now();
			},
			Message::Previous => {
				self.current_status = cmp::max(self.current_status - 1, 0);
				self.last_interaction = Instant::now();
			},
			Message::Refresh => {
				return Command::perform(api::dial(), Message::Fetched);
			},
			Message::Tick => {
				if Instant::now() - self.last_interaction > Duration::from_secs(120) {
					if self.detail_toggle { // reset detail panel if i forgot to close it for today
						self.detail_toggle = false;
					}
				}
				
				if Instant::now() - self.last_interaction > Duration::from_secs(300) {
					if self.current_status != 0 { // reset to current day if i forgot to switch back to it
						self.current_status = 0;
					}
				}
			},
			Message::ToggleDetail => {
				self.detail_toggle = !self.detail_toggle;
				self.last_interaction = Instant::now();
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let width = 402;

		if let None = self.api_data {
			return Container::new(
				Container::new(Text::new(""))
					.width(Length::Units(width - 40))
					.height(Length::Units(250))	
					.style(style::BlankWeatherContainer)
			)
				.width(Length::Units(width))
				.height(Length::Units(WINDOW_HEIGHT))
				.padding([0, 0, 0, 40])
				.style(style::WeatherContainer)
				.align_y(alignment::Vertical::Center)
				.into()
		}

		let create_temp_time = |status: &TemperatureDatum| {
			Column::new()
				.width(Length::Units(width / 3))
				.align_items(Alignment::Center)
				.push(
					Text::new(status.get_temperature())
						.size(55)
				)
				.push(
					Text::new(status.get_time())
						.size(25)
				)
		};

		let previous_day = Button::new(
			&mut self.previous_day_state,
			Text::new(if self.current_status != 0 { "\u{e408}" } else { "" })
				.width(Length::Units(40))
				.height(Length::Fill)
				.vertical_alignment(alignment::Vertical::Center)
				.size(60)
				.font(ICONS)
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
				.font(ICONS)
		)
			.on_press(Message::Next)
			.width(Length::Units(40))
			.height(Length::Fill)
			.padding(0)
			.style(style::WeatherButton);
		
		let temperature_or_detailed = if self.detail_toggle {
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
									get_directory(),
									self.statuses[self.current_status as usize].uv
								)
							)
								.width(Length::Units(80))
						)
							.width(Length::Units((width as f32 * 0.2) as u16))
							.align_x(iced::alignment::Horizontal::Center)
					)
					.push(
						Container::new(
							Row::new()
								.align_items(Alignment::Center)
								.push(
									Svg::from_path(
										format!(
											"{}/data/raindrop.svg",
											get_directory()
										)
									)
										.width(Length::Units(60))
										.height(Length::Units(80))
								)
								.push(
									Text::new(format!("{}%", self.statuses[self.current_status as usize].humidity))
										.size(55)
								)
						)
							.width(Length::Units((width as f32 * 0.4) as u16))
							.align_x(alignment::Horizontal::Center)
					)
					.push(
						Container::new(
							Row::new()
								.align_items(Alignment::Center)
								.push(
									Svg::from_path(
										format!(
											"{}/data/wind.svg",
											get_directory()
										)
									)
										.width(Length::Units(80))
								)
								.push(
									Text::new(format!("{}", self.statuses[self.current_status as usize].wind))
										.size(55)
								)
						)
							.width(Length::Units((width as f32 * 0.4) as u16))
							.align_x(alignment::Horizontal::Center)
					)
			)
				.on_press(Message::ToggleDetail)
				.padding([6, 0, 0, 0])
				.style(style::WeatherButton)
		} else {
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
									"{}/data/{}.svg",
									get_directory(),
									self.statuses[self.current_status as usize].icon
								)
							)
								.width(Length::Units(180))
						)
						.push( // temperature & day
							Column::new()
								.padding(0)
								.align_items(Alignment::Start)
								.push(
									Text::new(self.statuses[self.current_status as usize].current.get_temperature())
										.size(70)
										.font(NOTOSANS_BOLD)
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
						.height(Length::Units(180))
				)
				.push( // temperatures at times
					temperature_or_detailed
				)
		)
			.width(Length::Units(width))
			.height(Length::Units(WINDOW_HEIGHT))
			.padding([8, 0, 0, 20])
			.style(style::WeatherContainer)
			.into()
	}
}
