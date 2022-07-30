use std::env;

use iced::{ Font };

pub static WINDOW_WIDTH: u16 = 1480;
pub static WINDOW_HEIGHT: u16 = 320;

pub const NOTOSANS_BOLD: Font = Font::External {
	name: "NotoSans Bold",
	bytes: include_bytes!("../data/fonts/NotoSans-Bold.ttf"),
};

// pub const NOTOSANS: Font = Font::External {
// 	name: "NotoSans Regular",
// 	bytes: include_bytes!("../data/fonts/NotoSans-Medium.ttf"),
// };

pub const NOTOSANS_THIN: Font = Font::External {
	name: "NotoSans Thin",
	bytes: include_bytes!("../data/fonts/NotoSans-Regular.ttf"),
};

pub const ICONS: Font = Font::External {
	name: "Material Icons",
	bytes: include_bytes!("../data/fonts/MaterialIcons-Regular.ttf"),
};

pub fn get_directory() -> String {
	match env::var("BANSHEELONG_DIR") {
		Ok(path) => path,
		Err(_) => String::from("/home/me/Projects/bansheelong"),
	}
}

pub fn get_api_key() -> String {
	match env::var("BANSHEELONG_OPEN_WEATHER_KEY") {
		Ok(key) => key,
		Err(_) => String::from(""),
	}
}
