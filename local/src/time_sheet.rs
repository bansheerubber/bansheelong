use bansheelong_types::{ Date, IO, Item, Weekday };
use chrono::{ Datelike, Local };
use image::RgbaImage;
use imageproc::drawing::{ draw_filled_rect_mut, draw_text_mut };
use imageproc::rect::Rect;

use crate::constants:: {
	BACKGROUND_COLOR,
	CHARACTERS_PER_ROW,
	FONT,
	FONT_SCALE,
	FONT_WIDTH,
	TODO_LIST_TEXT_COLOR,
	TIMESHEET_COLORS,
	TIMESHEET_HEIGHT,
	TIMESHEET_HEIGHT_PADDING,
	TIMESHEET_HOUR_HEIGHT,
	TIMESHEET_WIDTH_PADDING
};

use crate::util::draw_timesheet_line;

type HourMinute = u32;

const START_TIME: HourMinute = 6; // time range: 1-24 hours
const END_TIME: HourMinute = 23;

fn time_index_to_hour(index: HourMinute) -> String {
	if index == 24 {
		String::from("12")
	} else if index > 12 {
		(index % 12).to_string()
	} else {
		index.to_string()
	}
}

fn time_to_position(hour: u8, minute: u8) -> i32 {
	if hour == 0 {
		(TIMESHEET_HOUR_HEIGHT as f32 * (24 - START_TIME) as f32 + TIMESHEET_HOUR_HEIGHT as f32 * (minute as f32 / 60.0)) as i32 + TIMESHEET_HEIGHT_PADDING as i32
	} else {
		(TIMESHEET_HOUR_HEIGHT as f32 * (hour as u32 - START_TIME) as f32 + TIMESHEET_HOUR_HEIGHT as f32 * (minute as f32 / 60.0)) as i32 + TIMESHEET_HEIGHT_PADDING as i32
	}
}

fn draw_item(image: &mut RgbaImage, item: &Item, color_index: &mut usize) {
	let time = item.time.unwrap();

	let start_time = time_to_position(time.start_hour, time.start_minute);
	let end_time = time_to_position(time.end_hour, time.end_minute);

	draw_filled_rect_mut(
		image,
		Rect::at(TIMESHEET_WIDTH_PADDING as i32, start_time)
			.of_size(FONT_WIDTH * CHARACTERS_PER_ROW, (end_time - start_time) as u32),
		TIMESHEET_COLORS[*color_index]
	);

	draw_timesheet_line(image, item.description.clone(), start_time);

	*color_index = (*color_index + 1) % TIMESHEET_COLORS.len();
}

pub fn draw_time_sheet(database: &IO, file_name: String) {
	let width = FONT_WIDTH * CHARACTERS_PER_ROW + TIMESHEET_WIDTH_PADDING + (TIMESHEET_WIDTH_PADDING / 2 as u32);
	let height = TIMESHEET_HEIGHT + TIMESHEET_HEIGHT_PADDING - TIMESHEET_HOUR_HEIGHT * (23 - (END_TIME - START_TIME));
	let mut image = RgbaImage::new(width, height);

	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), BACKGROUND_COLOR);

	// plot time
	for i in START_TIME..=END_TIME {
		draw_text_mut(
			&mut image,
			TODO_LIST_TEXT_COLOR,
			(FONT_WIDTH / 2) as i32,
			((i - START_TIME) as f32 / 24.0 * TIMESHEET_HEIGHT as f32 + 0.5) as i32 + TIMESHEET_HEIGHT_PADDING as i32,
			FONT_SCALE,
			&FONT,
			time_index_to_hour(i).as_str()
		);
	}

	let mut color_index = 0;
	let time = Local::now();

	let weekday = match time.weekday() {
		chrono::Weekday::Mon => { Weekday::Monday },
		chrono::Weekday::Tue => { Weekday::Tuesday },
		chrono::Weekday::Wed => { Weekday::Wednesday },
		chrono::Weekday::Thu => { Weekday::Thursday },
		chrono::Weekday::Fri => { Weekday::Friday },
		chrono::Weekday::Sat => { Weekday::Saturday },
		chrono::Weekday::Sun => { Weekday::Sunday },
	};

	let current_date = Some(Date {
		day: time.day() as u8,
		month: time.month() as u8,
		year: (time.year() % 100) as u8,
	});

	// draw day-to-day events
	for (date, day) in database.todos_database.mapping.iter() {
		if date != &current_date {
			continue;
		}
		
		for item in day.items.iter() {
			if let None = item.time {
				continue;
			}

			draw_item(&mut image, item, &mut color_index);
		}
	}

	// draw recurring events
	if database.todos_database.mapping.contains_key(&None) {
		for item in database.todos_database.mapping[&None].items.iter() {
			if let None = item.time {
				continue;
			}

			if item.time.unwrap().day == Some(weekday) {
				draw_item(&mut image, item, &mut color_index);
			}
		}
	}

	if let Err(error) = image.save(file_name) {
		eprintln!("Error saving todo list image: {:?}", error);
	}
}
