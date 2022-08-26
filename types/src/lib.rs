use std::env;
use bitflags::bitflags;

pub(crate) mod io;
pub(crate) mod read_write;
pub(crate) mod tests;
pub(crate) mod types;

pub use types::Date;
pub use types::Day;
pub use types::Dirty;
pub use types::Error;
pub use types::ErrorTag;
pub use types::IO;
pub use types::Ingredient;
pub use types::Item;
pub use types::MealsDatabase;
pub use types::PlannedIngredient;
pub use types::PlannedMeal;
pub use types::PlannedMealsRemoveLog;
pub use types::PlannedMealsWriteLog;
pub use types::Recipe;
pub use types::RecipeStep;
pub use types::Resource;
pub use types::Time;
pub use types::TodosDatabase;
pub use types::Weekday;
pub use types::WriteDatabase;

pub use read_write::read_database;
pub use read_write::write_database;

pub fn get_todos_server_port() -> u16 {
	match env::var("BANSHEELONG_TODOS_SERVER_PORT") {
		Ok(port) => port.parse().unwrap(),
		Err(_) => 0,
	}
}

pub fn get_todos_port() -> String {
	match env::var("BANSHEELONG_TODOS_PORT") {
		Ok(port) => {
			if let Ok(_) = port.parse::<u16>() {
				format!(":{}", port)
			} else {
				String::from("")
			}
		},
		Err(_) => String::from(""),
	}
}

pub fn get_todos_host() -> String {
	match env::var("BANSHEELONG_TODOS_HOST") {
		Ok(host) => host,
		Err(_) => String::from("localhost"),
	}
}

pub fn get_todos_path() -> String {
	match env::var("BANSHEELONG_TODOS_PATH") {
		Ok(path) => path,
		Err(_) => String::from(""),
	}
}

pub fn get_todos_secret() -> String {
	match env::var("BANSHEELONG_TODOS_SECRET") {
		Ok(secret) => secret,
		Err(_) => String::from(""),
	}
}

pub fn get_todos_https_cert() -> String {
	match env::var("BANSHEELONG_TODOS_HTTPS_CERT") {
		Ok(cert) => cert,
		Err(_) => String::from(""),
	}
}

pub fn get_todos_https_key() -> String {
	match env::var("BANSHEELONG_TODOS_HTTPS_KEY") {
		Ok(key) => key,
		Err(_) => String::from(""),
	}
}

pub fn get_storage_port() -> u16 {
	match env::var("BANSHEELONG_STORAGE_PORT") {
		Ok(port) => port.parse().unwrap(),
		Err(_) => 0,
	}
}

pub fn get_storage_host() -> String {
	match env::var("BANSHEELONG_STORAGE_HOST") {
		Ok(host) => host,
		Err(_) => String::from("localhost"),
	}
}

pub fn get_static_path() -> Option<String> {
	match env::var("BANSHEELONG_TODOS_HTTP_ROOT") {
		Ok(root) => Some(root),
		Err(_) => None,
	}
}

// how many words we send to clients using the storage server
pub const STORAGE_MESSAGE_COUNT: u8 = 5;

bitflags! {
	pub struct JobStatusFlags: u64 {
		const IDLE              = 0b00000000;
		const ERROR             = 0b00000001;
		const DOWNLOADING_DAILY = 0b00000010;
		const CREATING_WEEKLY   = 0b00000100;
		const CREATING_MONTHLY  = 0b00001000;
		const SYNCING_GITHUB    = 0b00010000;
		const REMOVING_DAILY    = 0b00100000;
		const REMOVING_WEEKLY   = 0b01000000;
	}
}
