mod constants;
mod style;
mod weather;

use iced::executor;
use iced::{ Application, Column, Command, Container, Element, Length, Settings, Text };

struct Window {
	weather: weather::render::View,
}

impl Application for Window {
	type Message = ();
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<()>) {
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

	fn update(&mut self, _message: ()) -> Command<()> {
		Command::none()
	}

	fn view(&mut self) -> Element<()> {
		Container::new(
			Column::new()
				.push(self.weather.view())
		)
			.width(Length::Fill)
			.padding(20)
			.style(style::Container)
			.into()
	}
}

fn main() -> iced::Result {
	Window::run(Settings {
		antialiasing: false,
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
