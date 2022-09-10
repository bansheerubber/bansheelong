use lazy_static::lazy_static;
use regex::Regex;

use crate::{
	Date,
	Day,
	Dirty,
	Error,
	ErrorTag,
	IO,
	Ingredient,
	Item,
	MealsDatabase,
	PlannedMeal,
	PlannedMealsRemoveLog,
	PlannedMealsWriteLog,
	Recipe,
	RecipeStep,
	Time,
	TodosDatabase,
	Weekday,
	WriteDatabase,
	read_database,
	write_database,
};

impl IO {
	pub async fn read_database(&mut self) -> Result<(&TodosDatabase, &MealsDatabase), Error> {
		match read_database(self.resource.clone()).await {
			Ok(database) => {
				self.todos_database = database.0;
				self.meals_database = database.1;
				self.dirty = Dirty::None;
				Ok((&self.todos_database, &self.meals_database))
			},
			Err(error) => {
				if error.tag == ErrorTag::CouldNotFindFile {
					self.write_database().await?;
					return Ok((&self.todos_database, &self.meals_database));
				}
				Err(error)
			},
		}
	}

	pub fn add_recipe(&mut self, recipe: Recipe) -> Result<&MealsDatabase, Error> {
		self.dirty = Dirty::Write;
		self.meals_database.recipes.push(recipe);
		self.meals_database.recipes.sort();
		Ok(&self.meals_database)
	}

	pub fn add_planned_meal(&mut self, meal: PlannedMeal) -> Result<&MealsDatabase, Error> {
		self.dirty = Dirty::Write;
		self.meals_database.planned_meal_mapping.insert(meal.date.clone(), meal);
		Ok(&self.meals_database)
	}

	pub fn add_planned_meal_log(&self, meal: PlannedMeal) -> PlannedMealsWriteLog {
		let mut log = self.planned_meals_write_log.clone();
		log.push(meal);
		return log;
	}

	pub fn remove_planned_meal(&mut self, date: Date) -> Result<&MealsDatabase, Error> {
		self.dirty = Dirty::Write;
		self.meals_database.planned_meal_mapping.remove(&date);
		Ok(&self.meals_database)
	}

	pub fn remove_planned_meal_log(&self, date: Date) -> PlannedMealsRemoveLog {
		let mut log = self.planned_meals_remove_log.clone();
		log.push(date);
		return log;
	}

	pub fn add_to_todos_database(&mut self, item: Item, date: Option<Date>) -> Result<&TodosDatabase, Error> {
		self.todos_write_log.push((item.clone(), date.clone()));
		self.dirty = Dirty::Write;
		if self.todos_database.mapping.contains_key(&date) {
			self.todos_database.mapping.get_mut(&date).unwrap().items.push(item);
		} else {
			self.todos_database.mapping.insert(date, Day {
				items: vec![item],
				date,
			});
		}

		Ok(&self.todos_database)
	}

	pub async fn write_database(&mut self) -> Result<(), Error> {
		match write_database(
			WriteDatabase::Full {
				meals: &self.meals_database,
				todos: &self.todos_database,
			},
			self.resource.clone()
		).await {
			Ok(_) => {
				self.planned_meals_write_log.clear();
				self.todos_write_log.clear();
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

	pub fn parse_from_human_readable(&mut self, todo_list: String, recipe_list: String) -> Result<(), Error> {
		self.meals_database = MealsDatabase::default();
		self.todos_database = TodosDatabase::default();
	
		// read the todos
		{
			let string = match std::fs::read_to_string(todo_list) {
				Ok(string) => string,
				Err(error) => return Err(Error {
					message: format!("{:?}", error),
					..Error::default()
				}),
			};

			lazy_static! {
				// group 1: month
				// group 2: day
				// group 3: year
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

					self.add_to_todos_database(item, date)?;
				}
			}
		}

		// read the recipes
		{
			let string = match std::fs::read_to_string(recipe_list) {
				Ok(string) => string,
				Err(error) => return Err(Error {
					message: format!("{:?}", error),
					..Error::default()
				}),
			};

			lazy_static! {
				// group 1: recipe name

				// group 2 & 3 are optional and dependent on each other
				// group 2: url of picture
				// group 3: minutes it takes to complete meal
				static ref NAME_REGEX: Regex = Regex::new(
					r"^([a-zA-Z\s\-0-9,.()]+)(?:\s+?\[([a-zA-Z0-9\-._~:/?#\[\]@!$&'()*+,;=]+),\s*?([0-9]+)\])?:$"
				).unwrap();

				// group 1: markup character delineating ingredient/steps/etc
				// group 2: text description of ingredient/step/etc
				static ref INFO_REGEX: Regex = Regex::new(
					r"([$#-]) ([a-zA-Z\s\-0-9,.()]+)(?:\s+?\[([a-zA-Z0-9\-._~:/?#\[\]@!$&'()*+,;=\s]+)\])?$"
				).unwrap();
			}

			let lines: Vec<String> = string.split("\n").map(str::to_string).collect();

			let mut name = None;

			// cleared whenever we get a new name
			let mut cooking_steps = Vec::new();
			let mut image_url = None;
			let mut ingredients = Vec::new();
			let mut minutes = None;
			let mut preparation_steps = Vec::new();

			for line in lines {
				if let Some(captures) = NAME_REGEX.captures(&line) {
					if name != None {
						self.add_recipe(Recipe {
							cooking_steps: cooking_steps.clone(),
							image_url,
							ingredients: ingredients.clone(),
							minutes,
							name: name.unwrap(),
							preparation_steps: preparation_steps.clone(),
						})?;
					}

					cooking_steps.clear();
					ingredients.clear();
					preparation_steps.clear();
					
					name = Some(String::from(captures.get(1).unwrap().as_str()));
					
					image_url = if let Some(capture) = captures.get(2) {
						Some(capture.as_str().to_string())
					} else {
						None
					};

					minutes = if let Some(capture) = captures.get(3) {
						match capture.as_str().parse::<u32>() {
							Ok(minutes) => Some(minutes),
							Err(_) => None,
						}
					} else {
						None
					};
				} else if let Some(captures) = INFO_REGEX.captures(&line) {
					let first_character = captures.get(1).unwrap().as_str();
					let rest = String::from(captures.get(2).unwrap().as_str());
					let extra = match captures.get(3) {
						Some(m) => Some(String::from(m.as_str())),
						None => None,
					};

					match first_character {
						"-" => { // ingredient markup
							ingredients.push(Ingredient {
								name: rest,
								quantity: extra,
							});
						},
						"#" => { // preparation markup
							preparation_steps.push(RecipeStep {
								extra_information: extra,
								name: rest,
							});
						},
						"$" => { // cooking markup
							cooking_steps.push(RecipeStep {
								extra_information: extra,
								name: rest,
							});
						},
						_ => {},
					}
				}
			}

			if name != None {
				self.add_recipe(Recipe {
					cooking_steps,
					image_url,
					ingredients,
					minutes,
					name: name.unwrap(),
					preparation_steps,
				})?;
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
	InvalidEndAmPm,
	InvalidTimeRange,
	NeededEndHours,
	NeededEndTime,
}

fn get_time_from_line(line: String) -> Result<Option<Time>, TimeError> {
	lazy_static! {
		// group 1: day (used for recurring events)
		// group 2: start hours
		// group 3: start minutes (optional)
		// group 4: start time am/pm (optional, inferred if not specified)
		// group 5: either a - for specifying a time range, or a + for a calculated time range
		// group 6: end hours (optional, required for time range)
		// group 7: end minutes (optional)
		// group 8: end time am/pm (optional, only for time range)
		static ref TIME_REGEX: Regex = Regex::new(
			r"(m|t|w|th|f|s|su)?(\d{1,2})(:\d{2})?(am|pm)?(-|\+)(\d{0,2})(:\d{2})?(am|pm)?"
		).unwrap();
	}
	
	if let Some(captures) = TIME_REGEX.captures(&line.as_str().to_lowercase()) {
		// get operator
		let operator = String::from(captures.get(5).unwrap().as_str());

		if operator == "-" { // time range error checking
			if captures.get(6).unwrap().as_str() == "" {
				return Err(TimeError::NeededEndHours);
			}
		} else if captures.get(8).is_some() { // calculated time range error checking
			return Err(TimeError::InvalidEndAmPm);
		} else if captures.get(6).unwrap().as_str() == "" && captures.get(7).is_none() {
			return Err(TimeError::NeededEndTime);
		}
		
		// decode day
		let day = if let None = captures.get(1) {
			None
		} else {
			Some(match captures.get(1).unwrap().as_str() {
				"m" => Weekday::Monday,
				"t" => Weekday::Tuesday,
				"w" => Weekday::Wednesday,
				"th" => Weekday::Thursday,
				"f" => Weekday::Friday,
				"s" => Weekday::Saturday,
				"su" => Weekday::Sunday,
				&_ => Weekday::Monday,
			})
		};
		
		// decode start hours
		let mut start_hour = if let None = captures.get(2) {
			0
		} else {
			match String::from(captures.get(2).unwrap().as_str()).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					return Err(TimeError::BadStartHours);
				}
			}
		};

		// decode start minutes
		let start_minute = if let None = captures.get(3) {
			0
		} else {
			let string = captures.get(3).unwrap().as_str();
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
		let mut end_hour = if captures.get(6).unwrap().as_str() == "" {
			0
		} else {
			match String::from(captures.get(6).unwrap().as_str()).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					return Err(TimeError::BadEndHours);
				}
			}
		};

		// decode end minutes
		let mut end_minute = if let None = captures.get(7) {
			0
		} else {
			let string = captures.get(7).unwrap().as_str();
			match String::from(&string[1..string.len()]).parse::<u8>() {
			  Ok(number) => {
					number
				},
				Err(_) => {
					eprintln!("{:?}", captures.get(7).unwrap().as_str());
					return Err(TimeError::BadEndMinutes);
				}
			}
		};

		// decode am/pm
		let start_ampm = if let None = captures.get(4) {
			if start_hour < 8 || start_hour == 12 {
				String::from("pm")
			} else {
				String::from("am")
			}
		} else {
			String::from(captures.get(4).unwrap().as_str())
		};

		let end_ampm = if let None = captures.get(8) {
			if end_hour < 8 || end_hour == 12 {
				String::from("pm")
			} else {
				String::from("am")
			}
		} else {
			String::from(captures.get(8).unwrap().as_str())
		};

		// handle start hours conversions
		if start_ampm == "pm" && start_hour != 12 {
			start_hour += 12;
		}

		if start_ampm == "am" && start_hour == 12 {
			start_hour = 0;
		}

		// handle operators
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

		if start_hour > end_hour {
			return Err(TimeError::InvalidTimeRange);
		} else if start_hour == end_hour && start_minute >= end_minute {
			return Err(TimeError::InvalidTimeRange);
		}

		Ok(Some(Time {
			day,
			start_hour,
			start_minute,
			end_hour,
			end_minute,
		}))
	} else {
		Ok(None)
	}
}
