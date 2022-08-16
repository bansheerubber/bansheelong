use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };
use warp::Filter;

use crate::types;
use crate::http::{ add_planned_meals, add_todos, add_recipes, get_database, remove_planned_meals, set_database };

use bansheelong_types::{ IO, get_todos_port };

pub async fn host(tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>, io: Arc<Mutex<IO>>) {
	let routes = add_todos::build_add_todos(tx.clone(), io.clone())
		.or(set_database::build_set_database(tx.clone(), io.clone()))
		.or(get_database::build_get_database(io.clone()))
		.or(add_recipes::build_add_recipes(tx.clone(), io.clone()))
		.or(add_planned_meals::build_add_planned_meals(tx.clone(), io.clone()))
		.or(remove_planned_meals::build_remove_planned_meals(tx.clone(), io.clone()));

	warp::serve(routes).run(([0, 0, 0 ,0], get_todos_port())).await;
}
