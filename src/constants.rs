use iced::{ Font };

pub static WINDOW_WIDTH: u16 = 1480;
pub static WINDOW_HEIGHT: u16 = 320;

pub const NOTOSANS_BOLD: Font = Font::External {
	name: "NotoSans Bold",
	bytes: include_bytes!("../data/fonts/NotoSans-Bold.ttf"),
};

pub const NOTOSANS: Font = Font::External {
	name: "NotoSans Regular",
	bytes: include_bytes!("../data/fonts/NotoSans-Medium.ttf"),
};

pub const ICONS: Font = Font::External {
	name: "Material Icons",
	bytes: include_bytes!("../data/fonts/MaterialIcons-Regular.ttf"),
};
