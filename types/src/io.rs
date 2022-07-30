use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

use futures::executor;
use serde::{ Serialize, Deserialize };

use crate::{ Database, Date, Day, Dirty, Error, IO, Item };

impl IO {
	pub fn read_database(&mut self) -> Result<&Database, Error> {
		if self.resource.contains("http") {
			let client = reqwest::Client::new();
			let response_result = executor::block_on(
				client.get(format!("{}/get-todos", self.resource))
					.header(reqwest::header::CONTENT_TYPE, "application/json")
					.header(reqwest::header::ACCEPT, "application/json")
					.send()
			);

			if let Err(error) = response_result {
				return Err(Error {
					message: format!("{:?}", error),
				});
			}
			
			Ok(&self.database)
		} else {
			self.count += 1;
			
			let mut file = match File::open(&self.resource) {
				Ok(file) => file,
				Err(error) => {
					if error.kind() == ErrorKind::NotFound {
						self.write_database()?;
						return Ok(&self.database);
					}
					
					return Err(Error {
						message: format!("{:?}", error),
					});
				}
			};

			let mut buffer = Vec::new();
			if let Err(error) = file.read_to_end(&mut buffer) {
				return Err(Error {
					message: format!("{:?}", error),
				});
			}

			let root = match flexbuffers::Reader::get_root(buffer.as_slice()) {
				Ok(root) => root,
				Err(error) => return Err(Error {
					message: format!("{:?}", error),
				}),
			};

			match Database::deserialize(root) {
				Ok(database) => {
					self.database = database;
					self.dirty = Dirty::None;
					Ok(&self.database)
				},
				Err(error) => Err(Error {
					message: format!("{:?}", error),
				}),
			}
		}
	}

	pub fn add_to_database(&mut self, item: Item, date: Option<Date>) -> Result<&Database, Error> {
		self.write_log.push((item.clone(), date.clone()));
		self.dirty = Dirty::Write;
		if self.database.mapping.contains_key(&date) {
			let items = &mut self.database.mapping.get_mut(&date).unwrap().items;
			let pos = items.binary_search(&item).unwrap_or_else(|e| e);
			items.insert(pos, item);
		} else {
			self.database.mapping.insert(date, Day {
				items: vec![item],
				date,
			});
		}

		Ok(&self.database)
	}

	pub fn add_to_database_sync(&mut self, item: Item, date: Option<Date>) -> Result<&Database, Error> {
		self.sync()?;
		
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

		self.sync()?;

		Ok(&self.database)
	}

	pub fn write_database(&mut self) -> Result<(), Error> {
		if self.resource.contains("http") {
			println!("{:?}", self.write_log);
			
			let client = reqwest::Client::new();
			let response_result = executor::block_on(
				client.post(format!("{}/add-todo", self.resource))
					.form(&[("todos", serde_json::to_string(&self.write_log).unwrap())])
					.send()
			);

			if let Err(error) = response_result {
				return Err(Error {
					message: format!("{:?}", error),
				});
			}

			self.write_log.clear();
			self.dirty = Dirty::None;
			Ok(())
		} else {
			self.write_log.clear();
			let mut serializer = flexbuffers::FlexbufferSerializer::new();
			if let Err(error) = self.database.serialize(&mut serializer) {
				return Err(Error {
					message: format!("{:?}", error),
				});
			}

			if let Err(error) = std::fs::write(&self.resource, serializer.view()) {
				Err(Error {
					message: format!("{:?}", error),
				})
			} else {
				self.dirty = Dirty::None;
				Ok(())
			}
		}
	}

	pub fn sync(&mut self) -> Result<(), Error> {
		match &self.dirty {
			Dirty::Read => {
				self.read_database()?;
			},
			Dirty::Write => {
				self.write_database()?;
			},
			_ => {},
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rand::{ Rng, SeedableRng };
	use rand::rngs::StdRng;

	use crate::Time;

	fn setup() -> IO {
		let mut io = IO {
			resource: String::from("/tmp/todos"),
			..IO::default()
		};

		if let Err(error) = io.write_database() {
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
							hour: generator.gen_range(0, 20),
							minute: generator.gen_range(0, 20),
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
			let mut last_time = None;
			for item in day.items.iter() {
				println!("{:?} >= {:?}", item.time, last_time);
				assert!(item.time >= last_time);
				last_time = item.time;
			}

			println!("{:?} >= {:?}", day.date, last_date);
			assert!(day.date >= last_date);
			last_date = day.date;
		}
	}
}
