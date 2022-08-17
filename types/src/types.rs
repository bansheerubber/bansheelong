use std::collections::{ BTreeMap, HashMap };
use std::cmp::Ordering;
use std::string::ToString;

use serde::{ Serialize, Deserialize };
use serde_with::serde_as;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Weekday {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Time {
	pub day: Option<Weekday>,
	pub start_hour: u8,
	pub start_minute: u8,
	pub end_hour: u8,
	pub end_minute: u8,
}

impl Ord for Time {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.start_hour < other.start_hour {
			Ordering::Less
		} else if self.start_hour > other.start_hour {
			Ordering::Greater
		} else if self.start_minute < other.start_minute {
			Ordering::Less
		} else if self.start_minute > other.start_minute {
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
		let time = if
			parameters.contains_key("start_hour")
			&& parameters.contains_key("start_minute")
			&& parameters.contains_key("end_hour")
			&& parameters.contains_key("end_minute")
		{
			Some(Time {
				day: None,
				start_hour: parameters.get("start_hour").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
				start_minute: parameters.get("start_minute").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
				end_hour: parameters.get("end_hour").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
				end_minute: parameters.get("end_minute").unwrap().parse::<u8>().unwrap_or_else(|_| { 0 }),
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TodosDatabase {
	#[serde_as(as = "Vec<(_, _)>")]
	pub mapping: BTreeMap<Option<Date>, Day>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ErrorTag {
	#[default]
	Generic,
	CouldNotFindFile,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Error {
	pub message: String,
	pub tag: ErrorTag,
}

#[derive(Clone, Debug)]
pub enum Dirty {
	None,
	Read,
	Write,
}

#[derive(Clone, Debug)]
pub struct Resource {
	pub reference: String,
}

pub type PlannedMealsRemoveLog = Vec<Date>;
pub type PlannedMealsWriteLog = Vec<PlannedMeal>;
pub type TodosWriteLog = Vec<(Item, Option<Date>)>;

#[derive(Clone, Debug)]
pub struct IO {
	pub count: i32,
	pub dirty: Dirty,
	pub meals_database: MealsDatabase,
	pub resource: Resource,
	pub todos_database: TodosDatabase,

	pub planned_meals_remove_log: PlannedMealsRemoveLog,
	pub planned_meals_write_log: PlannedMealsWriteLog,
	pub todos_write_log: TodosWriteLog,
}

impl Default for IO {
	fn default() -> Self {
		IO {
			count: 0,
			dirty: Dirty::Read,
			meals_database: MealsDatabase::default(),
			planned_meals_remove_log: Vec::new(),
			planned_meals_write_log: Vec::new(),
			resource: Resource {
				reference: String::from("todos")
			},
			todos_database: TodosDatabase::default(),
			todos_write_log: Vec::new(),
		}
	}
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MealsDatabase {
	pub recipes: Vec<Recipe>,
	#[serde_as(as = "Vec<(_, _)>")]
	pub planned_meal_mapping: BTreeMap<Date, PlannedMeal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ingredient {
	pub name: String,
}

impl Ingredient {
	pub fn new(name: String) -> Self {
		Ingredient {
			name,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Recipe {
	pub cooking_steps: Vec<String>,
	pub ingredients: Vec<Ingredient>,
	pub name: String,
	pub preparation_steps: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlannedIngredient {
	pub acquired: bool,
	pub ingredient: Ingredient,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlannedMeal {
	pub date: Date,
	pub ingredients: Vec<PlannedIngredient>,
	pub recipe: Recipe,
}

impl PlannedMeal {
	pub fn new(date: Date, recipe: Recipe) -> Self {
		PlannedMeal {
			date,
			ingredients: recipe.ingredients.iter()
				.map(|x| {
					PlannedIngredient {
						acquired: false,
						ingredient: x.clone(),
					}
				})
				.collect(),
			recipe,
		}
	}
}

pub enum WriteDatabase<'a> {
	Full {
		meals: &'a MealsDatabase,
		todos: &'a TodosDatabase,
	},
	Partial {
		planned_meals_remove_log: &'a PlannedMealsRemoveLog,
		planned_meals_write_log: &'a PlannedMealsWriteLog,
		todos_write_log: &'a TodosWriteLog,
	},
}
