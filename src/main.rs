mod constants;
mod style;
mod weather;

use iced::executor;
use iced::{ Application, Column, Command, Container, Element, Length, Settings, Subscription };

struct Window {
	weather: weather::render::View,
}

#[derive(Debug)]
enum Message {
	Redraw,
	WeatherMessage(weather::render::Message),
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		(
			Window {
				weather: weather::render::View::new(),
			},
			Command::none(),
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
			iced::time::every(std::time::Duration::from_secs(300)).map(|_| { // refersh weather info
				Self::Message::WeatherMessage(
					weather::render::Message::Refresh
				)
			})
		])
	}

	fn update(&mut self, _message: Message) -> Command<Self::Message> {
		match _message {
			Self::Message::Redraw => {},
			Self::Message::WeatherMessage(message) => {
				self.weather.update(message);
			},
		};

		Command::none()
	}

	fn view(&mut self) -> Element<Self::Message> {
		Container::new(
			Column::new()
				.push(self.weather.view().map(move |message| {
					Self::Message::WeatherMessage(message)
				}))
		)
			.width(Length::Fill)
			.padding([12, 0, 0, 20])
			.style(style::Container)
			.into()
	}
}

fn main() -> iced::Result {
	Window::run(Settings {
		antialiasing: true,
		default_font: Some(include_bytes!("../data/fonts/NotoSans-Medium.ttf")),
		window: iced::window::Settings {
			size: (constants::WINDOW_WIDTH as u32, constants::WINDOW_HEIGHT as u32),
			resizable: false,
			decorations: false,
			..iced::window::Settings::default()
		},
		..Settings::default()
	})
}
