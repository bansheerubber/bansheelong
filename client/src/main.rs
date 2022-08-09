mod calendar;
mod constants;
mod flavor;
mod meals;
mod menu;
mod shared;
mod storage;
mod style;
mod todos;
mod weather;

use std::sync::Arc;

use iced::alignment;
use iced::executor;
use iced::{ Application, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text };

use bansheelong_types::{ Database, Error, IO, Resource, get_todos_host, get_todos_port, read_database };

struct Window {
	flavor: flavor::View,
	menu: menu::View,
	storage: storage::View,
	weather: weather::View,

	io: Arc<IO>,
}

#[derive(Debug)]
enum Message {
	FetchedTodos(Result<Database, Error>),
	FlavorMessage(flavor::Message),
	Refresh,
	RefreshTodos,
	StorageMessage(storage::Message),
	MenuMessage(menu::Message),
	Tick,
	WeatherMessage(weather::Message),
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let resource = Resource {
			reference: format!("http://{}:{}", get_todos_host(), get_todos_port()),
		};

		(
			Window {
				flavor: flavor::View::new(),
				menu: menu::View::new(),
				storage: storage::View::new(),
				weather: weather::View::new(),

				io: Arc::new(IO {
					resource: resource.clone(),
					..IO::default()
				}),
			},
			Command::batch([
				Command::perform(weather::api::dial(), move |result| {
					Self::Message::WeatherMessage(weather::Message::Fetched(result))
				}),
				Command::perform(read_database(resource), Self::Message::FetchedTodos),
			])
		)
	}

	fn title(&self) -> String {
		String::from("bansheelong")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		Subscription::batch([
			iced::time::every(std::time::Duration::from_secs(300)).map(|_| Self::Message::Refresh), // refresh weather/todos
			iced::time::every(std::time::Duration::from_secs(1)).map(|_| { // tick weather widget so it can detect absense of user interaction, etc
				Self::Message::Tick
			}),
			todos::connect().map(|event| {
				match event {
					todos::Event::Error(_) => Self::Message::MenuMessage(menu::Message::TodosMessage(
						todos::Message::Update(None)
					)),
					todos::Event::InvalidateState => Self::Message::MenuMessage(menu::Message::TodosMessage(
						todos::Message::Update(None)
					)),
					todos::Event::Refresh => Self::Message::RefreshTodos,
				}
			})
		])
	}

	fn update(&mut self, _message: Message) -> Command<Self::Message> {
		match _message {
			Self::Message::FetchedTodos(result) => {
				if let Err(error) = result {
					println!("{:?}", error);
					
					Command::batch([
						self.menu.update(menu::Message::CalendarMessage(
							calendar::Message::Update(None)
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
						self.menu.update(menu::Message::TodosMessage(
							todos::Message::Update(None)
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
					])
				} else {
					self.io = Arc::new(IO { // TODO clean this up
						database: result.unwrap(),
						resource: self.io.resource.clone(),
						..IO::default()
					});

					Command::batch([
						self.menu.update(menu::Message::CalendarMessage(
							calendar::Message::Update(Some(self.io.clone()))
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
						self.menu.update(menu::Message::TodosMessage(
							todos::Message::Update(Some(self.io.clone()))
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
					])
				}
			},
			Self::Message::FlavorMessage(message) => {
				println!("try to update the thing");
				self.flavor.update(message).map(move |message| {
					Self::Message::FlavorMessage(message)
				})
			},
			Self::Message::MenuMessage(message) => {
				self.menu.update(message).map(move |message| {
					Self::Message::MenuMessage(message)
				})
			},
			Self::Message::Refresh => {
				Command::batch([
					Command::perform(read_database(self.io.resource.clone()), Self::Message::FetchedTodos),
					self.weather.update(weather::Message::Refresh).map(move |message| {
						Self::Message::WeatherMessage(message)
					}),
				])
			},
			Self::Message::RefreshTodos => {
				Command::perform(read_database(self.io.resource.clone()), Self::Message::FetchedTodos)
			},
			Self::Message::StorageMessage(_) => { Command::none() },
			Self::Message::Tick => {
				Command::batch([
					self.menu.update(menu::Message::Tick).map(move |message| {
						Self::Message::MenuMessage(message)
					}),
					self.weather.update(weather::Message::Tick).map(move |message| {
						Self::Message::WeatherMessage(message)
					}),
				])
			},
			Self::Message::WeatherMessage(message) => {
				self.weather.update(message).map(move |message| {
					Self::Message::WeatherMessage(message)
				})
			},
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		Container::new(
			Row::new()
				.push( // weather
					self.weather.view().map(move |message| {
						Self::Message::WeatherMessage(message)
					})
				)
				.push( // vertical rule
					Container::new(
						Container::new(Text::new(""))
							.style(style::VerticalRule)
							.width(Length::Units(2))
							.height(Length::Units(constants::WINDOW_HEIGHT - 50))
					)
						.height(Length::Fill)
						.padding([0, 25])
						.align_y(alignment::Vertical::Center)
				)
				.push(
					self.menu.view().map(move |message| {
						Self::Message::MenuMessage(message)
					})
				)
				.push( // storage thing & neat picture
					Column::new()
						.push(
							self.storage.view().map(move |message| {
								Self::Message::StorageMessage(message)
							})
						)
						.push(
							self.flavor.view().map(move |message| {
								Self::Message::FlavorMessage(message)
							})
						)
				)
		)
			.width(Length::Fill)
			.style(style::Container)
			.into()
	}
}

#[tokio::main]
async fn main() -> iced::Result {
	Window::run(Settings {
		antialiasing: false,
		default_font: Some(include_bytes!("../data/fonts/NotoSans-Medium.ttf")),
		text_multithreading: true,
		window: iced::window::Settings {
			size: (constants::WINDOW_WIDTH as u32, constants::WINDOW_HEIGHT as u32),
			resizable: false,
			decorations: false,
			..iced::window::Settings::default()
		},
		..Settings::default()
	})
}
