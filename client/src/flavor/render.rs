use iced::{ Container, Element, Length, image };

use crate::constants::get_directory;

#[derive(Debug)]
pub struct View {
	image: image::Handle,
	image_state: image::viewer::State,
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl View {
	pub fn new() -> Self {
		View {
			image: image::Handle::from_path(format!(
				"{}/data/pictures/gerbil.png",
				get_directory()
			)),
			image_state: image::viewer::State::new(),
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		Container::new(
			image::Viewer::new(&mut self.image_state, self.image.clone())
				.width(Length::Units(260))
				.height(Length::Units(215))
				.min_scale(1.0)
		)
			.padding([5, 0, 0, 5])
			.into()
	}
}
