use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };
use warp::Filter;

use crate::types;
use crate::http::{ add_planned_meal, add_todos, add_recipes, get_database, set_database };

use bansheelong_types::{ IO, get_todos_port };

pub async fn host(tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>, io: Arc<Mutex<IO>>) {
	let routes = add_todos::build_add_todos(tx.clone(), io.clone())
		.or(set_database::build_set_database(tx.clone(), io.clone()))
		.or(get_database::build_get_database(io.clone()))
		.or(add_recipes::build_add_recipes(tx.clone(), io.clone()))
		.or(add_planned_meal::build_add_planned_meal(tx.clone(), io.clone()));

	warp::serve(routes).run(([0, 0, 0 ,0], get_todos_port())).await;
}
