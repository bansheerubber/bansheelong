use image::RgbaImage;
use imageproc::drawing::draw_text_mut;

use crate::constants::{
	CHARACTERS_PER_ROW,
	FONT,
	FONT_HEIGHT,
	FONT_SCALE,
	TODO_LIST_TEXT_COLOR,
	TIMESHEET_TEXT_COLOR,
	TIMESHEET_WIDTH_PADDING,
	VALID_STARTING_CHARACTERS,
};

pub(crate) fn row_to_y(row: f32) -> i32 {
	(row * FONT_HEIGHT as f32) as i32
}

pub(crate) fn draw_todo_line(image: &mut RgbaImage, text: String, mut row: f32) -> f32 {
	// we need to split it up, pad it with space at the front and back
	let split = text.split(" ");
	let mut buffer = Vec::new();
	let mut buffer_character_count = 0;

	for word in split {
		if buffer_character_count + word.trim().len() + buffer.len() >= (CHARACTERS_PER_ROW - 2) as usize {
			let joined = format!(" {} ", buffer.join(" "));
			draw_text_mut(image, TODO_LIST_TEXT_COLOR, 0, row_to_y(row), FONT_SCALE, &FONT, joined.as_str());
			row += 1.0;

			buffer.clear();
			buffer_character_count = 0;
		}

		buffer.push(word.trim());
		buffer_character_count += word.trim().len();
	}

	if buffer.len() > 0 {
		let joined = format!(" {} ", buffer.join(" "));
		draw_text_mut(image, TODO_LIST_TEXT_COLOR, 0, row_to_y(row), FONT_SCALE, &FONT, joined.as_str());
		row += 1.0;
	}

	return row;
}

pub(crate) fn draw_timesheet_line(image: &mut RgbaImage, text: String, mut y_offset: i32) {
	let text = if VALID_STARTING_CHARACTERS.contains(&text.chars().nth(0).unwrap())
		&& text.chars().nth(1).unwrap() == ' '
	{
		String::from(&text[2..])
	} else {
		text
	};
	
	// we need to split it up, pad it with space at the front and back
	let split = text.split(" ");
	let mut buffer = Vec::new();
	let mut buffer_character_count = 0;

	for word in split {
		if buffer_character_count + word.trim().len() + buffer.len() >= (CHARACTERS_PER_ROW - 2) as usize {
			let joined = format!("{}", buffer.join(" "));
			draw_text_mut(image, TIMESHEET_TEXT_COLOR, TIMESHEET_WIDTH_PADDING as i32 + 4, y_offset + 3, FONT_SCALE, &FONT, joined.as_str());
			y_offset += FONT_HEIGHT as i32;

			buffer.clear();
			buffer_character_count = 0;
		}

		buffer.push(word.trim());
		buffer_character_count += word.trim().len();
	}

	if buffer.len() > 0 {
		let joined = format!("{}", buffer.join(" "));
		draw_text_mut(image, TIMESHEET_TEXT_COLOR, TIMESHEET_WIDTH_PADDING as i32 + 4, y_offset + 3, FONT_SCALE, &FONT, joined.as_str());
	}
}
