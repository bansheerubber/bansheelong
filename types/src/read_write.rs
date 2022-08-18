use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

use serde::{ Serialize, Deserialize };

use crate::{
	Day,
	Error,
	ErrorTag,
	MealsDatabase,
	Resource,
	TodosDatabase,
	WriteDatabase,
	get_todos_secret
};

pub async fn read_database(resource: Resource) -> Result<(TodosDatabase, MealsDatabase), Error> {
	if resource.reference.contains("http") {
		let client = reqwest::Client::new();
		let response_result = client.get(format!("{}/get-database/", resource.reference))
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

		match response.json::<(TodosDatabase, MealsDatabase)>().await {
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

		match <(TodosDatabase, MealsDatabase)>::deserialize(root) {
			Ok(database) => Ok(database),
			Err(error) => Err(Error {
				message: format!("{:?}", error),
				..Error::default()
			}),
		}
	}
}

pub async fn write_database<'a>(
	data: WriteDatabase<'a>,
	resource: Resource
) -> Result<(), Error> {
	if resource.reference.contains("http") {
		let client = reqwest::Client::new();
		let response_result = match data {
    	WriteDatabase::Full {
				meals,
				todos,
			} => {
				Some(
					client.post(format!("{}/set-database/", resource.reference))
						.header("Secret", get_todos_secret())
						.body(serde_json::to_string(&(todos, meals)).unwrap())
						.send()
						.await
				)
			},
    	WriteDatabase::Partial {
				planned_meals_remove_log,
				planned_meals_write_log,
				todos_write_log
			} => {
				let mut result = None;
				
				if planned_meals_remove_log.len() > 0 {
					result = Some(
						client.post(format!("{}/remove-planned-meals/", resource.reference))
							.header("Secret", get_todos_secret())
							.body(serde_json::to_string(planned_meals_remove_log).unwrap())
							.send()
							.await
					);
				}

				if planned_meals_write_log.len() > 0 {
					result = Some(
						client.post(format!("{}/add-planned-meals/", resource.reference))
							.header("Secret", get_todos_secret())
							.body(serde_json::to_string(planned_meals_write_log).unwrap())
							.send()
							.await
					);
				}
				
				if todos_write_log.len() > 0 {
					result = Some(
						client.post(format!("{}/add-todos/", resource.reference))
							.header("Secret", get_todos_secret())
							.body(serde_json::to_string(todos_write_log).unwrap())
							.send()
							.await
					);
				}

				result
			},
		};

		if let Some(response_result) = response_result.as_ref() {
			if let Err(error) = response_result {
				return Err(Error {
					message: format!("{:?}", error),
					..Error::default()
				});
			}
		}

		if let Some(response_result) = response_result {
			if response_result.as_ref().unwrap().status() != reqwest::StatusCode::OK {
				return Err(Error {
					message: response_result.unwrap().text().await.unwrap().clone(),
					..Error::default()
				});
			}
		}

		Ok(())
	} else {
		let mut serializer = flexbuffers::FlexbufferSerializer::new();

		let mut read_databases: (Option<TodosDatabase>, Option<MealsDatabase>);

		let databases = match data {
    	WriteDatabase::Full {
				meals,
				todos,
			} => {
				(todos, meals)
			},
    	WriteDatabase::Partial {
				planned_meals_remove_log,
				planned_meals_write_log,
				todos_write_log
			} => {
				let databases = read_database(resource.clone()).await?;
				read_databases = (Some(databases.0), Some(databases.1));

				for date in planned_meals_remove_log {
					read_databases.1.as_mut().unwrap().planned_meal_mapping.remove(date);
				}

				for planned_meal in planned_meals_write_log {
					read_databases.1.as_mut().unwrap().planned_meal_mapping.insert(
						planned_meal.date,
						planned_meal.clone()
					);
				}
				
				for (item, date) in todos_write_log {
					if read_databases.0.as_mut().unwrap().mapping.contains_key(&date) {
						read_databases.0.as_mut().unwrap().mapping.get_mut(&date).unwrap().items.push(item.clone());
					} else {
						read_databases.0.as_mut().unwrap().mapping.insert(date.clone(), Day {
							items: vec![item.clone()],
							date: date.clone(),
						});
					}
				}

				(read_databases.0.as_ref().unwrap(), read_databases.1.as_ref().unwrap())
			},
		};

		if let Err(error) = databases.serialize(&mut serializer) {
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
