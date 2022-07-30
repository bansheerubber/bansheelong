use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

use serde::{ Serialize, Deserialize };

use crate::{ Database, Date, Day, Dirty, Error, ErrorTag, IO, Item, Resource };

pub async fn read_database(resource: Resource) -> Result<Database, Error> {
	if resource.reference.contains("http") {
		let client = reqwest::Client::new();
		let response_result = client.get(format!("{}/get-todos", resource.reference))
			.header(reqwest::header::CONTENT_TYPE, "application/json")
			.header(reqwest::header::ACCEPT, "application/json")
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
		let response_result =  client.post(format!("{}/add-todo", resource.reference))
			.form(&[("todos", serde_json::to_string(&write_log).unwrap())])
			.send()
			.await;

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
