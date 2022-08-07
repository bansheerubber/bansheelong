use lazy_static::lazy_static;

pub mod render;

pub use render::Message;
pub use render::View;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Menu {
	Meals,
	Todos,
}

pub const BUTTON_COUNT: u16 = 2;
pub const BUTTON_HEIGHT: u16 = 30;
pub const BUTTON_SPACING: u16 = 15;
pub const BUTTON_AREA_SIZE: u16 = BUTTON_HEIGHT * (BUTTON_COUNT - 1) + BUTTON_SPACING * (BUTTON_COUNT - 2) + 40;

lazy_static! {
	pub static ref BUTTONS: [(String, Menu); BUTTON_COUNT as usize] = [
		(String::from("Meal manager"), Menu::Meals),
		(String::from("Todo manager"), Menu::Todos),
	];
}
