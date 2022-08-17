use image::Rgba;
use lazy_static::lazy_static;
use rusttype::{ Font, Scale };

pub(crate) const CHARACTERS_PER_ROW: u32 = 40;

pub(crate) const BACKGROUND_COLOR: Rgba<u8> = Rgba([30, 18, 30, 242]);

pub(crate) const FONT_SCALE: Scale = Scale {
	x: 16.0,
	y: 16.0,
};
pub(crate) const FONT_HEIGHT: u32 = 14;
pub(crate) const FONT_WIDTH: u32 = 8;

pub(crate) const TIMESHEET_HEIGHT: u32 = 1008;
pub(crate) const TIMESHEET_HOUR_HEIGHT: u32 = TIMESHEET_HEIGHT / 24;
pub(crate) const TIMESHEET_HEIGHT_PADDING: u32 = 4;
pub(crate) const TIMESHEET_TEXT_COLOR: Rgba<u8> = Rgba([233, 217, 233, 255]);
pub(crate) const TIMESHEET_WIDTH_PADDING: u32 = FONT_WIDTH * 3;

pub(crate) const TIMESHEET_BLUE: Rgba<u8> = Rgba([138, 118, 224, 255]);
pub(crate) const TIMESHEET_MAGENTA: Rgba<u8> = Rgba([224, 89, 224, 255]);
pub(crate) const TIMESHEET_GREEN: Rgba<u8> = Rgba([44, 186, 96, 255]);
pub(crate) const TIMESHEET_CYAN: Rgba<u8> = Rgba([88, 183, 206, 255]);

pub(crate) static TIMESHEET_COLORS: [Rgba<u8>; 4] = [
	TIMESHEET_BLUE,
	TIMESHEET_MAGENTA,
	TIMESHEET_GREEN,
	TIMESHEET_CYAN,
];

pub(crate) const TODO_LIST_TEXT_COLOR: Rgba<u8> = Rgba([183, 172, 183, 255]);

lazy_static! {
	pub(crate) static ref FONT: Font<'static> = Font::try_from_vec(Vec::from(include_bytes!("../fonts/Greybeard-16px.ttf") as &[u8])).unwrap();
}
