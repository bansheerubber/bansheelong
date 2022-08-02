use std::env;

pub(crate) mod io;
pub(crate) mod types;

pub use types::Database;
pub use types::Date;
pub use types::Day;
pub use types::Dirty;
pub use types::Error;
pub use types::ErrorTag;
pub use types::IO;
pub use types::Item;
pub use types::Resource;
pub use types::Time;

pub use io::read_database;
pub use io::write_database;

pub fn get_todos_port() -> u16 {
	match env::var("BANSHEELONG_TODOS_PORT") {
		Ok(port) => port.parse().unwrap(),
		Err(_) => 80,
	}
}

pub fn get_todos_host() -> String {
	match env::var("BANSHEELONG_TODOS_HOST") {
		Ok(host) => host,
		Err(_) => String::from("localhost"),
	}
}

pub fn get_todos_secret() -> String {
	match env::var("BANSHEELONG_TODOS_SECRET") {
		Ok(secret) => secret,
		Err(_) => String::from(""),
	}
}
