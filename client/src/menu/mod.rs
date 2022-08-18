pub mod render;

use bansheelong_shared_ui::constants;
use lazy_static::lazy_static;

pub use render::Message;
pub use render::View;

lazy_static! {
	pub static ref MENU_STATE: constants::MenuState = constants::MenuState {
		buttons: vec![
			(String::from("Meal manager"), constants::Menu::Meals),
			(String::from("Todo manager"), constants::Menu::Todos),
		],
		button_count: 2,
		button_height: 30,
		button_spacing: 15,
		width: 740,
	};
}
