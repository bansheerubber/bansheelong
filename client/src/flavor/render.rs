use iced::{ Button, Command, Element, Length, button, image };

use bansheelong_shared_ui::{ constants, style };

#[derive(Debug)]
pub struct View {
	button_state: button::State,
	image: image::Handle,
	image_state: image::viewer::State,
	paths: Vec<String>,
	paths_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
	SwitchImage,
}

impl View {
	pub fn new() -> Self {
		let mut view = View {
			button_state: button::State::new(),
			image: image::Handle::from_path(format!(
				"{}/data/pictures/gerbil.png",
				constants::get_directory()
			)),
			image_state: image::viewer::State::new(),
			paths: Vec::new(),
			paths_index: 0,
		};

		view.read_images();

		return view;
	}

	fn read_images(&mut self) {
		let paths = std::fs::read_dir(format!("{}/data/pictures", constants::get_directory()));
		if let Ok(paths) = paths {
			self.paths = paths.map(|x| x.unwrap().path().display().to_string()).collect();
			self.paths_index = 0;

			self.image = image::Handle::from_path(self.paths[0].clone());
		}
	}

	pub fn update(&mut self, _message: Message) -> Command<Message> {
		self.paths_index = (self.paths_index + 1) % self.paths.len();
		self.image = image::Handle::from_path(self.paths[self.paths_index].clone());

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		Button::new(
			&mut self.button_state,
			image::Viewer::new(&mut self.image_state, self.image.clone())
				.width(Length::Units(260))
				.height(Length::Units(215))
				.min_scale(1.0)
		)
			.on_press(Message::SwitchImage)
			.padding([5, 0, 0, 5])
			.style(style::WeatherButton)
			.into()
	}
}
