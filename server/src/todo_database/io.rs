use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

use serde::{ Serialize, Deserialize };

use crate::todo_database::{ Database, Date, Day, Dirty, Error, IO, Item };

impl IO {
	pub fn read_database(&mut self) -> Result<&Database, Error> {
		self.count += 1;
		
		let mut file = match File::open("todos") {
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

	pub fn add_to_database(&mut self, item: Item, date: Option<Date>) -> Result<&Database, Error> {
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
		let mut serializer = flexbuffers::FlexbufferSerializer::new();
		if let Err(error) = self.database.serialize(&mut serializer) {
			return Err(Error {
				message: format!("{:?}", error),
			});
		}

		if let Err(error) = std::fs::write("todos", serializer.view()) {
			Err(Error {
				message: format!("{:?}", error),
			})
		} else {
			self.dirty = Dirty::None;
			Ok(())
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

	use crate::todo_database::Time;

	fn setup() -> IO {
		let mut io = IO {
			file_name: String::from("/tmp/todos"),
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
