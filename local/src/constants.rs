use image::Rgba;
use lazy_static::lazy_static;
use rusttype::{ Font, Scale };

pub(crate) const CHARACTERS_PER_ROW: u32 = 40;
pub(crate) const FONT_WIDTH: u32 = 8;
pub(crate) const FONT_HEIGHT: u32 = 14;

// const TIMESHEET_HEIGHT: u32 = 1000;
// const TIMESHEET_PADDING: u32 = FONT_WIDTH * 3;

lazy_static! {
	pub(crate) static ref BACKGROUND_COLOR: Rgba<u8> = Rgba([30, 18, 30, 242]);
	pub(crate) static ref FONT: Font<'static> = Font::try_from_vec(Vec::from(include_bytes!("../fonts/Greybeard-16px.ttf") as &[u8])).unwrap();
	pub(crate) static ref FONT_SCALE: Scale = Scale {
		x: 16.0,
		y: 16.0
	};
	pub(crate) static ref TEXT_COLOR: Rgba<u8> = Rgba([183, 172, 183, 255]);
}
