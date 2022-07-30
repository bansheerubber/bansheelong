mod constants;
mod style;
mod todos;
mod weather;

use iced::alignment;
use iced::executor;
use iced::{ Application, Command, Container, Element, Length, Row, Settings, Subscription, Text };

use bansheelong_types::{ Resource, read_database };

struct Window {
	todos: todos::render::View,
	weather: weather::render::View,
}

#[derive(Debug)]
enum Message {
	Redraw,
	Refresh,
	TodoMessage(todos::render::Message),
	WeatherMessage(weather::render::Message),
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let todos_resource = Resource {
			reference: String::from("http://localhost:3000")
		};

		(
			Window {
				todos: todos::render::View::new(todos_resource.clone()),
				weather: weather::render::View::new(),
			},
			Command::batch([
				Command::perform(weather::api::dial(), move |result| {
					Self::Message::WeatherMessage(weather::render::Message::Fetched(result))
				}),
				Command::perform(read_database(todos_resource), move |result| {
					Self::Message::TodoMessage(todos::render::Message::Fetched(result))
				}),
			])
		)
	}

	fn title(&self) -> String {
		String::from("bansheelong")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		Subscription::batch([
			iced::time::every(std::time::Duration::from_millis(16)).map(|_| { // force redraw for rpi4
				Self::Message::Redraw
			}),
			iced::time::every(std::time::Duration::from_secs(300)).map(|_| Self::Message::Refresh), // refresh weather/todos
			iced::time::every(std::time::Duration::from_secs(1)).map(|_| { // tick weather widget so it can detect absense of user interaction, etc
				Self::Message::WeatherMessage(
					weather::render::Message::Tick
				)
			})
		])
	}

	fn update(&mut self, _message: Message) -> Command<Self::Message> {
		match _message {
			Self::Message::Redraw => {},
			Self::Message::Refresh => {
				return Command::batch([
					self.todos.update(todos::render::Message::Refresh).map(move |message| {
						Self::Message::TodoMessage(message)
					}),
					self.weather.update(weather::render::Message::Refresh).map(move |message| {
						Self::Message::WeatherMessage(message)
					}),
				]);
			}
			Self::Message::TodoMessage(message) => {
				return self.todos.update(message).map(move |message| {
					Self::Message::TodoMessage(message)
				});
			},
			Self::Message::WeatherMessage(message) => {
				return self.weather.update(message).map(move |message| {
					Self::Message::WeatherMessage(message)
				});
			},
		};

		Command::none()
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
