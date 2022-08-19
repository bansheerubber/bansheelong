pub mod render;

use bansheelong_shared_ui::constants;
use lazy_static::lazy_static;

use crate::state::WINDOW_STATE;

pub use render::Message;
pub use render::View;

lazy_static! {
	pub static ref MENU_STATE: constants::MenuState = constants::MenuState {
		buttons: vec![
			(String::from("Meal manager"), constants::Menu::Meals),
		],
		button_count: 1,
		button_height: 36,
		button_spacing: 15,
		width: WINDOW_STATE.width - 5 - 20,
	};
}
