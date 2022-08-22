use bansheelong_types::{ Date, IO };
use chrono::{ Datelike, Local, TimeZone, Utc, Weekday };
use image::RgbaImage;
use image::imageops::crop;
use imageproc::drawing::{ draw_filled_circle_mut, draw_filled_rect_mut };
use imageproc::rect::Rect;

use crate::constants:: { BACKGROUND_COLOR, CHARACTERS_PER_ROW, FONT_HEIGHT, FONT_WIDTH, TIMESHEET_COLORS, TODO_LIST_TEXT_COLOR, };
use crate::util::{ draw_todo_line, row_to_y };

pub fn draw_todo_list(database: &IO, file_name: String) {
	let mut image = RgbaImage::new(FONT_WIDTH * CHARACTERS_PER_ROW, 1000);
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(FONT_WIDTH * CHARACTERS_PER_ROW, 1000), BACKGROUND_COLOR);

	let mut row = 0.5; // keep track of where we're drawing text

	let time = Local::now();
	let current_date = Some(Date {
		day: time.day() as u8,
		month: time.month() as u8,
		year: (time.year() % 100) as u8,
	});

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
				TODO_LIST_TEXT_COLOR
			);

			row = draw_todo_line(&mut image, date_string.clone(), row);
			row += 0.4; // padding for dates
		}

		let mut color_index = 0;
		for item in day.items.iter() {
			if item.time.is_some() && date.is_none() { // do not display recurring events
				continue;
			}

			if item.description.len() == 0 { // for separations, add half a row instead of a full one
				row += 0.6
			} else {
				let description = if date == &current_date && item.time.is_some() { // get item description
					format!(" {}", item.description.split("-").skip(1).map(|x| x.to_string()).collect::<Vec<String>>().join("-"))
				} else {
					item.description.clone()
				};

				if date == &current_date && item.time.is_some() { // draw circle in place of hyphen
					draw_filled_circle_mut(
						&mut image,
						(FONT_WIDTH as i32 + FONT_WIDTH as i32 / 2, row_to_y(row) + FONT_HEIGHT as i32 / 2),
						4,
						TIMESHEET_COLORS[color_index]
					);
					color_index += 1;
				}
				
				row = draw_todo_line(&mut image, description, row);
			}
		}
	}

	image = crop(&mut image, 0, 0, FONT_WIDTH * CHARACTERS_PER_ROW, (row * FONT_HEIGHT as f32) as u32).to_image();

	if let Err(error) = image.save(file_name) {
		eprintln!("Error saving todo list image: {:?}", error);
	}
}
