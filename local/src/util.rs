use image::RgbaImage;
use imageproc::drawing::draw_text_mut;

use crate::constants:: { CHARACTERS_PER_ROW, FONT, FONT_HEIGHT, FONT_SCALE, TEXT_COLOR, };

pub(crate) fn draw_todo_line(image: &mut RgbaImage, text: String, mut row: f32) -> f32 {
	// we need to split it up, pad it with space at the front and back
	let split = text.split(" ");
	let mut buffer = Vec::new();
	let mut buffer_character_count = 0;

	for word in split {
		if buffer_character_count + word.trim().len() + buffer.len() >= (CHARACTERS_PER_ROW - 2) as usize {
			let joined = format!(" {} ", buffer.join(" "));
			draw_text_mut(image, *TEXT_COLOR, 0, (row * FONT_HEIGHT as f32) as i32, *FONT_SCALE, &FONT, joined.as_str());
			row += 1.0;

			buffer.clear();
			buffer_character_count = 0;
		}

		buffer.push(word.trim());
		buffer_character_count += word.trim().len();
	}

	if buffer.len() > 0 {
		let joined = format!(" {} ", buffer.join(" "));
		draw_text_mut(image, *TEXT_COLOR, 0, (row * FONT_HEIGHT as f32) as i32, *FONT_SCALE, &FONT, joined.as_str());
		row += 1.0;
	}

	return row;
}
