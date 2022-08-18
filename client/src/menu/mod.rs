pub mod render;

use bansheelong_shared_ui::constants::MenuState;
use lazy_static::lazy_static;

pub use render::Message;
pub use render::View;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Menu {
	Meals,
	Todos,
}

// pub const BUTTON_COUNT: u16 = 2;
// pub const BUTTON_HEIGHT: u16 = 30;
// pub const BUTTON_SPACING: u16 = 15;
// pub const BUTTON_AREA_SIZE: u16 = BUTTON_HEIGHT * (BUTTON_COUNT - 1) + BUTTON_SPACING * (BUTTON_COUNT - 2) + 20;

pub const MENU_STATE: MenuState = MenuState {
	button_count: 2,
	button_height: 30,
	button_spacing: 15,
	width: 740,
};

lazy_static! {
	pub static ref BUTTONS: [(String, Menu); MENU_STATE.button_count as usize] = [
		(String::from("Meal manager"), Menu::Meals),
		(String::from("Todo manager"), Menu::Todos),
	];
}
