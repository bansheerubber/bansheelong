pub(crate) mod add_planned_meals;
pub(crate) mod add_todos;
pub(crate) mod add_recipes;
pub(crate) mod failed_secret;
pub(crate) mod get_database;
pub(crate) mod host;
pub(crate) mod remove_planned_meals;
pub(crate) mod set_database;

pub(crate) use failed_secret::failed_secret;
pub use host::host;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response {
	error: Option<String>,
	success: bool,
}
