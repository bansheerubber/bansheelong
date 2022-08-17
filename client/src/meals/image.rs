use std::fs;
use std::path::Path;

use crate::constants::get_directory;

pub fn download_image(url: &String, file_name: &String) {
	let file_path = format!("{}/data/meals-images/{}", get_directory(), file_name);
	
	let request = reqwest::blocking::get(url);
	if let Err(error) = request {
		eprintln!("Could not create request for '{}' with error {:?}", url, error);
		return;
	}

  let bytes = request.unwrap().bytes();
	if let Ok(bytes) = bytes {
		if let Err(error) = fs::write(&file_path, bytes) {
			eprintln!("Could not write image to '{}' with error {:?}", file_path, error);
		}
	} else {
		eprintln!("Could not complete request for '{}' with error {:?}", url, bytes.err().unwrap());
	}
}

pub fn has_image(file_name: &String) -> bool {
	Path::new(&format!("{}/data/meals-images/{}", get_directory(), file_name)).exists()
}

pub fn is_valid_image_url(url: &String) -> bool {
	if &url[0..4] == "http" {
		return true;
	} else {
		return false;
	}
}
