use bansheelong_types::IO;
use chrono::{ Datelike, TimeZone, Utc, Weekday };
use image::RgbaImage;
use image::imageops::crop;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::constants:: { BACKGROUND_COLOR, CHARACTERS_PER_ROW, FONT_HEIGHT, FONT_WIDTH, TEXT_COLOR, };
use crate::util::draw_todo_line;

pub fn draw_todo_list(database: &IO, file_name: String) {
	let mut image = RgbaImage::new(FONT_WIDTH * CHARACTERS_PER_ROW, 1000);
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(FONT_WIDTH * CHARACTERS_PER_ROW, 1000), *BACKGROUND_COLOR);

	let mut row = 0.5; // keep track of where we're drawing text

	for (date, day) in database.todos_database.mapping.iter() {
		if let Some(date) = date {
			let abbreviation = match Utc.ymd(2000 + date.year as i32, date.month as u32, date.day as u32)
				.and_hms(0, 0, 0).date().weekday()
			{
				Weekday::Mon => "m",
				Weekday::Tue => "t",
				Weekday::Wed => "w",
				Weekday::Thu => "th",
				Weekday::Fri => "f",
				Weekday::Sat => "s",
				Weekday::Sun => "su",
			};

			let date_string = format!("{}/{}/{}({}):", date.month, date.day, date.year, abbreviation);

			draw_filled_rect_mut(
				&mut image,
				Rect::at(FONT_WIDTH as i32, (FONT_HEIGHT as f32 * (row + 1.0)) as i32)
					.of_size(FONT_WIDTH * date_string.len() as u32, 1),
				*TEXT_COLOR
			);

			row = draw_todo_line(&mut image, date_string.clone(), row);
			row += 0.4; // padding for dates
		}

		for item in day.items.iter() {
			if item.time.is_some() && date.is_none() {
				continue;
			}

			if item.description.len() == 0 { // for separations, add half a row instead of a full one
				row += 0.6
			} else {
				row = draw_todo_line(&mut image, item.description.clone(), row);
			}
		}
	}

	image = crop(&mut image, 0, 0, FONT_WIDTH * CHARACTERS_PER_ROW, (row * FONT_HEIGHT as f32) as u32).to_image();

	if let Err(error) = image.save(file_name) {
		eprintln!("Error saving todo list image: {:?}", error);
	}
}
