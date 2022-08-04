mod calendar;
mod constants;
mod style;
mod todos;
mod weather;

use std::sync::Arc;

use iced::alignment;
use iced::executor;
use iced::{ Application, Command, Container, Element, Length, Row, Settings, Subscription, Text };

use bansheelong_types::{ Database, Error, IO, Resource, get_todos_host, get_todos_port, read_database };

struct Window {
	calendar: calendar::View,
	todos: todos::View,
	weather: weather::View,

	io: Arc<IO>,
}

#[derive(Debug)]
enum Message {
	CalendarMessage(calendar::Message),
	FetchedTodos(Result<Database, Error>),
	Redraw,
	Refresh,
	RefreshTodos,
	TodoMessage(todos::Message),
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
				calendar: calendar::View::new(),
				todos: todos::View::new(),
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
					todos::Event::Error(_) => Self::Message::TodoMessage(todos::Message::Update(None)),
					todos::Event::InvalidateState => Self::Message::TodoMessage(todos::Message::Update(None)),
					todos::Event::Refresh => Self::Message::RefreshTodos,
				}
			})
		])
	}

	fn update(&mut self, _message: Message) -> Command<Self::Message> {
		match _message {
			Self::Message::CalendarMessage(message) => {
				self.calendar.update(message).map(move |message| {
					Self::Message::CalendarMessage(message)
				})
			},
			Self::Message::FetchedTodos(result) => {
				if let Err(error) = result {
					println!("{:?}", error);
				} else if let Ok(database) = result {
					self.io = Arc::new(IO { // TODO clean this up
						database,
						resource: self.io.resource.clone(),
						..IO::default()
					});
				}

				self.todos.update(todos::Message::Update(Some(self.io.clone()))).map(move |message| {
					self::Message::TodoMessage(message)
				})
			},
			Self::Message::Redraw => { Command::none() },
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
			Self::Message::TodoMessage(message) => {
				self.todos.update(message).map(move |message| {
					Self::Message::TodoMessage(message)
				})
			},
			Self::Message::Tick => {
				Command::batch([
					self.calendar.update(calendar::Message::Tick).map(move |message| {
						Self::Message::CalendarMessage(message)
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
				.push( // todo list
					self.todos.view().map(move |_message| {
						Self::Message::Redraw
					})
				)
				.push( // calendar bar
					self.calendar.view().map(move |message| {
						Self::Message::CalendarMessage(message)
					})
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
