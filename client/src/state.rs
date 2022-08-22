use bansheelong_shared_ui::constants::WindowState;

pub const WINDOW_STATE: WindowState = WindowState {
	width: 1480,
	height: 320,
};

pub(crate) static VALID_STARTING_CHARACTERS: [char; 4] = ['-', '!', '%', 'z'];
