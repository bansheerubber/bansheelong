use std::env;

use iced::Font;

#[derive(Debug)]
pub struct WindowState {
	pub width: u16,
	pub height: u16,
}

pub const NOTOSANS_BOLD: Font = Font::External {
	name: "NotoSans Bold",
	bytes: include_bytes!("../data/fonts/NotoSans-Bold.ttf"),
};

pub const NOTOSANS: Font = Font::External {
	name: "NotoSans Regular",
	bytes: include_bytes!("../data/fonts/NotoSans-Medium.ttf"),
};

pub const NOTOSANS_THIN: Font = Font::External {
	name: "NotoSans Thin",
	bytes: include_bytes!("../data/fonts/NotoSans-Regular.ttf"),
};

pub const ICONS: Font = Font::External {
	name: "Material Icons",
	bytes: include_bytes!("../data/fonts/MaterialIcons-Regular.ttf"),
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Menu {
	Meals,
	Todos,
}

#[derive(Clone, Debug)]
pub struct MenuState {
	pub buttons: Vec<(String, Menu)>,
	pub button_count: u16,
	pub button_height: u16,
	pub button_spacing: u16,
	pub width: u16,
}

impl MenuState {
	pub fn get_area_size(&self) -> u16 {
		let spacing_amount = if self.button_count >= 2 {
			self.button_count - 2
		} else {
			0
		};

		let button_count = if self.button_count >= 1 {
			self.button_count - 1
		} else {
			0
		};
		
		self.button_height * button_count + self.button_spacing * spacing_amount + 20
	}
}

pub fn get_directory() -> String {
	match env::var("BANSHEELONG_DIR") {
		Ok(path) => path,
		Err(_) => String::from(""),
	}
}

pub fn get_api_key() -> String {
	match env::var("BANSHEELONG_OPEN_WEATHER_KEY") {
		Ok(key) => key,
		Err(_) => String::from(""),
	}
}
