use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

use lazy_static::lazy_static;

use regex::Regex;
use serde::{ Serialize, Deserialize };

use crate::{ Database, Date, Day, Dirty, Error, ErrorTag, IO, Item, Resource, Time };

use crate::get_todos_secret;

pub async fn read_database(resource: Resource) -> Result<Database, Error> {
	if resource.reference.contains("http") {
		let client = reqwest::Client::new();
		let response_result = client.get(format!("{}/get-todos", resource.reference))
			.header(reqwest::header::CONTENT_TYPE, "application/json")
			.header(reqwest::header::ACCEPT, "application/json")
			.header("Secret", get_todos_secret())
			.send()
			.await;

		if let Err(error) = response_result {
			return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			});
		}
		let response = response_result.unwrap();

		match response.json::<Database>().await {
			Ok(result) => Ok(result),
			Err(error) => Err(Error {
				message: format!("Could not deserialize JSON: {:?}", error),
				..Error::default()
			}),
		}
	} else {
		let mut file = match File::open(&resource.reference) {
			Ok(file) => file,
			Err(error) => {
				if error.kind() == ErrorKind::NotFound {
					return Err(Error {
						message: String::from("Could not find file"),
						tag: ErrorTag::CouldNotFindFile,
					});
				}
				
				return Err(Error {
					message: format!("{:?}", error),
					..Error::default()
				});
			}
		};

		let mut buffer = Vec::new();
		if let Err(error) = file.read_to_end(&mut buffer) {
			return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			});
		}

		let root = match flexbuffers::Reader::get_root(buffer.as_slice()) {
			Ok(root) => root,
			Err(error) => return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			}),
		};

		match Database::deserialize(root) {
			Ok(database) => Ok(database),
			Err(error) => Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			}),
		}
	}
}

pub async fn write_database(
	database: &Database,
	write_log: Option<&Vec<(Item, Option<Date>)>>,
	resource: Resource
) -> Result<(), Error> {
	if resource.reference.contains("http") {
		let client = reqwest::Client::new();
		let response_result = if let Some(log) = write_log {
			client.post(format!("{}/add-todos", resource.reference))
				.header("Secret", get_todos_secret())
				.body(serde_json::to_string(&log).unwrap())
				.send()
				.await
		} else { // if no write log is specified, write the whole database
			client.post(format!("{}/set-todos", resource.reference))
				.header("Secret", get_todos_secret())
				.body(serde_json::to_string(&database).unwrap())
				.send()
				.await
		};

		if let Err(error) = response_result {
			return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			});
		}

		Ok(())
	} else {
		let mut serializer = flexbuffers::FlexbufferSerializer::new();
		if let Err(error) = database.serialize(&mut serializer) {
			return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			});
		}

		if let Err(error) = std::fs::write(&resource.reference, serializer.view()) {
			Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			})
		} else {
			Ok(())
		}
	}
}

impl IO {
	pub async fn read_database(&mut self) -> Result<&Database, Error> {
		match read_database(self.resource.clone()).await {
			Ok(database) => {
				self.database = database;
				self.dirty = Dirty::None;
				Ok(&self.database)
			},
			Err(error) => {
				if error.tag == ErrorTag::CouldNotFindFile {
					self.write_database().await?;
					return Ok(&self.database);
				}
				Err(error)
			},
		}
	}

	pub fn add_to_database(&mut self, item: Item, date: Option<Date>) -> Result<&Database, Error> {
		self.write_log.push((item.clone(), date.clone()));
		self.dirty = Dirty::Write;
		if self.database.mapping.contains_key(&date) {
			self.database.mapping.get_mut(&date).unwrap().items.push(item);
		} else {
			self.database.mapping.insert(date, Day {
				items: vec![item],
				date,
			});
		}

		Ok(&self.database)
	}

	pub async fn add_to_database_sync(&mut self, item: Item, date: Option<Date>) -> Result<&Database, Error> {
		self.sync().await?;
		
		self.write_log.push((item.clone(), date.clone()));
		self.dirty = Dirty::Write;
		if self.database.mapping.contains_key(&date) {
			self.database.mapping.get_mut(&date).unwrap().items.push(item);
		} else {
			self.database.mapping.insert(date, Day {
				items: vec![item],
				date,
			});
		}

		self.sync().await?;

		Ok(&self.database)
	}

	pub async fn write_database(&mut self) -> Result<(), Error> {
		match write_database(&self.database, Some(&self.write_log), self.resource.clone()).await {
			Ok(_) => {
				self.write_log.clear();
				self.dirty = Dirty::None;
				Ok(())
			},
			Err(error) => Err(error),
		}
	}

	pub async fn sync(&mut self) -> Result<(), Error> {
		match &self.dirty {
			Dirty::Read => {
				self.read_database().await?;
			},
			Dirty::Write => {
				self.write_database().await?;
			},
			_ => {},
		}

		Ok(())
	}

	pub fn parse_from_human_readable(&mut self, file: String) -> Result<(), Error> {
		self.database = Database::default();
		
		let string = match std::fs::read_to_string(file) {
			Ok(string) => string,
			Err(error) => return Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			}),
		};

		lazy_static! {
			static ref DATE_REGEX: Regex = Regex::new(r"([0-9]+)/([0-9]+)/([0-9]+)").unwrap();
		}
		
		let lines: Vec<String> = string.split("\n").map(str::to_string).collect();
		let mut date: Option<Date> = None;
		for mut line in lines {
			if let Some(captures) = DATE_REGEX.captures(&line) {
				let month = String::from(captures.get(1).unwrap().as_str());
				let day = String::from(captures.get(2).unwrap().as_str());
				let year = String::from(captures.get(3).unwrap().as_str());

				date = Some(Date {
					day: day.parse().unwrap(),
					month: month.parse().unwrap(),
					year: year.parse().unwrap(),
				});
			} else {
				let time = get_time_from_line(line.clone());

				let time = if let Err(error) = time {
					eprintln!("Could not parse time for item: {:?}", error);
					None
				} else {
					time.unwrap()
				};

				if line.len() > 0 && line.chars().nth(0).unwrap() == '@' {
					line = line.split(" ").skip(1).collect::<Vec<&str>>().join(" ");
				}

				let item = Item {
					description: line,
					time,
				};

				self.add_to_database(item, date)?;
			}
		}

		Ok(())
	}
}

#[derive(Debug)]
enum TimeError {
	BadEndHours,
	BadStartHours,
	BadEndMinutes,
	BadStartMinutes,
}

fn get_time_from_line(line: String) -> Result<Option<Time>, TimeError> {
	lazy_static! {
		static ref TIME_REGEX: Regex = Regex::new(r"(\d{1,2})(:\d{2})?(am|pm)?(-|\+)(\d{0,2})(:\d{2})?(am|pm)?").unwrap();
	}
	
	if let Some(captures) = TIME_REGEX.captures(&line.as_str().to_lowercase()) {
		// decode start hours
		let mut start_hour = if let None = captures.get(1) {
			0
		} else {
			match String::from(captures.get(1).unwrap().as_str()).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					return Err(TimeError::BadStartHours);
				}
			}
		};

		// decode start minutes
		let start_minute = if let None = captures.get(2) {
			0
		} else {
			let string = captures.get(2).unwrap().as_str();
			match String::from(&string[1..string.len()]).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					return Err(TimeError::BadStartMinutes);
				}
			}
		};

		// decode end hours
		let mut end_hour = if let None = captures.get(5) {
			0
		} else {
			match String::from(captures.get(5).unwrap().as_str()).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					return Err(TimeError::BadEndHours);
				}
			}
		};

		// decode end minutes
		let mut end_minute = if let None = captures.get(6) {
			0
		} else {
			let string = captures.get(6).unwrap().as_str();
			match String::from(&string[1..string.len()]).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					println!("{:?}", captures.get(6).unwrap().as_str());
					return Err(TimeError::BadEndMinutes);
				}
			}
		};

		// decode am/pm
		let start_ampm = if let None = captures.get(3) {
			if start_hour < 8 || start_hour == 12 {
				String::from("pm")
			} else {
				String::from("am")
			}
		} else {
			String::from(captures.get(3).unwrap().as_str())
		};

		let end_ampm = if let None = captures.get(3) {
			if end_hour < 8 || end_hour == 12 {
				String::from("pm")
			} else {
				String::from("am")
			}
		} else {
			String::from(captures.get(3).unwrap().as_str())
		};

		// handle start hours conversions
		if start_ampm == "pm" && start_hour != 12 {
			start_hour += 12;
		}

		if start_ampm == "am" && start_hour == 12 {
			start_hour = 0;
		}

		// handle operators
		let operator = String::from(captures.get(4).unwrap().as_str());
		if end_ampm == "pm" && end_hour != 12 && operator != "+" {
			end_hour += 12;
		}

		if operator == "+" {
			end_hour = start_hour + end_hour;
			end_minute = start_minute + end_minute;

			if end_minute >= 60 {
				end_hour += 1;
				end_minute = end_minute % 60;
			}
		}

		Ok(Some(Time {
			start_hour,
			start_minute,
			end_hour,
			end_minute,
		}))
	} else {
		Ok(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rand::{ Rng, SeedableRng };
	use rand::rngs::StdRng;

	use crate::{ Resource, Time };

	fn setup() -> IO {
		let mut io = IO {
			resource: Resource {
				reference: String::from("/tmp/todos"),
			},
			..IO::default()
		};

		if let Err(error) = tokio_test::block_on(io.write_database()) {
			panic!("{:?}", error);
		}

		let mut generator = StdRng::seed_from_u64(1659117803);

		let mut dates = Vec::new();
		for _ in 0..100 {
			dates.push(Some(Date {
				day: generator.gen_range(0, 20),
				month: generator.gen_range(0, 20),
				year: generator.gen_range(0, 20),
			}));
		}
		dates.push(None);

		for date in dates {
			for _ in 0..100 {
				if let Err(error) = io.add_to_database(
					Item {
						description: String::from(""),
						time: Some(Time {
							start_hour: generator.gen_range(0, 20),
							start_minute: generator.gen_range(0, 20),
							end_hour: 0,
							end_minute: 0,
						}),
					},
					date
				) {
					panic!("{:?}", error);
				}
			}

			if let Err(error) = io.add_to_database(
				Item {
					description: String::from(""),
					time: None,
				},
				date
			) {
				panic!("{:?}", error);
			}
		}

		io
	}

	#[test]
	fn monotonically_increasing() {
		let io = setup();
		
		let mut last_date = None;
		for (_, day) in io.database.mapping.iter() {
			println!("{:?} >= {:?}", day.date, last_date);
			assert!(day.date >= last_date);
			last_date = day.date;
		}
	}
}
