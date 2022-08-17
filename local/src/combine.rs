use image::imageops::overlay;

pub fn combine(background_path: String, todo_list_path: String, time_sheet_path: String, output_path: String) {
	// let mut background = image::open(&background_path);
	let mut background = match image::open(&background_path) {
		Ok(image) => image.to_rgba8(),
		Err(_) => {
			eprintln!("Could not find background at '{}'", background_path);
			return;
		},
	};

	let background_width = background.width();

	let todo_list = match image::open(&todo_list_path) {
		Ok(image) => image.to_rgba8(),
		Err(_) => {
			eprintln!("Could not find todo-list at '{}'", todo_list_path);
			return;
		},
	};

	let time_sheet = match image::open(&time_sheet_path) {
		Ok(image) => image.to_rgba8(),
		Err(_) => {
			eprintln!("Could not find time sheet at '{}'", time_sheet_path);
			return;
		},
	};
	
	overlay(&mut background, &todo_list, 15, 15 + 27);
	overlay(&mut background, &time_sheet, background_width as i64 - time_sheet.width() as i64 - 15, 15 + 27);

	if let Err(error) = background.save(output_path) {
		eprintln!("Error saving background image: {:?}", error);
	}
}
