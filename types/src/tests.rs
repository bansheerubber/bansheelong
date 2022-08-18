#[cfg(test)]
mod tests {
	use rand::{ Rng, SeedableRng };
	use rand::rngs::StdRng;

	use crate::{ Date, IO, Item, Resource, Time };

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
				if let Err(error) = io.add_to_todos_database(
					Item {
						description: String::from(""),
						time: Some(Time {
							day: None,
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

			if let Err(error) = io.add_to_todos_database(
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
		for (_, day) in io.todos_database.mapping.iter() {
			println!("{:?} >= {:?}", day.date, last_date);
			assert!(day.date >= last_date);
			last_date = day.date;
		}
	}
}
