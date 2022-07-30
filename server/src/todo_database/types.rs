use std::collections::{ BTreeMap, HashMap };
use std::cmp::Ordering;
use std::string::ToString;

use serde::{ Serialize, Deserialize };
use serde_with::serde_as;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Time {
	pub hour: u8,
	pub minute: u8,
}

impl Ord for Time {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.hour < other.hour {
			Ordering::Less
		} else if self.hour > other.hour {
			Ordering::Greater
		} else if self.minute < other.minute {
			Ordering::Less
		} else if self.minute > other.minute {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl PartialOrd for Time {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Item {
	pub description: String,
	pub time: Option<Time>,
}

impl Ord for Item {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.time == other.time {
			Ordering::Equal
		} else if let None = self.time {
			Ordering::Less
		} else if let None = other.time {
			Ordering::Greater
		} else {
			self.time.unwrap().cmp(&other.time.unwrap())
		}
	}
}

impl PartialOrd for Item {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Item {
	pub fn new(parameters: &HashMap<String, String>) -> Item {
		let time = if parameters.contains_key("hour") && parameters.contains_key("minute") {
			Some(Time {
				hour: parameters.get("hour").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
				minute: parameters.get("minute").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
			})
		} else {
			None
		};
		
		Item {
			description: parameters.get("description").unwrap().to_string(),
			time,
		}
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Date {
	pub day: u8,
	pub month: u8,
	pub year: u8,
}

impl Ord for Date {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.year < other.year {
			Ordering::Less
		} else if self.year > other.year {
			Ordering::Greater
		} else if self.month < other.month {
			Ordering::Less
		} else if self.month > other.month {
			Ordering::Greater
		} else if self.day < other.day {
			Ordering::Less
		} else if self.day > other.day {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Date {
	pub fn new(parameters: &HashMap<String, String>) -> Option<Date> {
		if parameters.contains_key("day") && parameters.contains_key("month") && parameters.contains_key("year") {
			Some(Date {
				day: parameters.get("day").unwrap().parse::<u8>().unwrap(),
				month: parameters.get("month").unwrap().parse::<u8>().unwrap(),
				year: parameters.get("year").unwrap().parse::<u8>().unwrap(),
			})
		} else {
			None
		}
	}
}

impl ToString for Date {
	fn to_string(&self) -> String {
		format!("{}/{}/{}", self.month, self.day, self.year)
	}
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Day {
	pub items: Vec<Item>,
	pub date: Option<Date>,
}

impl Ord for Day {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.date == other.date {
			Ordering::Equal
		} else if let None = self.date {
			Ordering::Less
		} else if let None = other.date {
			Ordering::Greater
		} else {
			self.date.unwrap().cmp(&other.date.unwrap())
		}
	}
}

impl PartialOrd for Day {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[serde_as]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Database {
	#[serde_as(as = "Vec<(_, _)>")]
	pub mapping: BTreeMap<Option<Date>, Day>,
}

#[derive(Debug)]
pub struct Error {
	pub message: String,
}

pub enum Dirty {
	None,
	Read,
	Write,
}

pub struct IO {
	pub database: Database,
	pub file_name: String,
	pub count: i32,
	pub dirty: Dirty,
}

impl Default for IO {
	fn default() -> Self {
		IO {
			database: Database::default(),
			file_name: String::from("todos"),
			count: 0,
			dirty: Dirty::Read,
		}
	}
}
