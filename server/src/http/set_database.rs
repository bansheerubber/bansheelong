use std::sync::Arc;
use std::convert::Infallible;
use tokio::sync::{ Mutex, mpsc };
use warp::Filter;

use crate::http::{ Response, failed_secret };
use crate::types;

use bansheelong_types::{ Dirty, IO, MealsDatabase, TodosDatabase, get_todos_secret };

async fn set_database_endpoint(
	secret: bool,
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>,
	databases: (TodosDatabase, MealsDatabase)
) -> Result<impl warp::Reply, Infallible> {
	println!("POST /set-database/");
	
	if !secret {
		return Ok(failed_secret());
	}
	
	let mut guard = io.lock().await;
	let old_planned_meals = guard.meals_database.planned_meal_mapping.clone();

	guard.meals_database = databases.1;

	if guard.meals_database.planned_meal_mapping.len() == 0 {
		guard.meals_database.planned_meal_mapping = old_planned_meals;
	}

	guard.todos_database = databases.0;
	guard.dirty = Dirty::Write;
	if let Err(error) = guard.sync().await {
		eprintln!(" -> Error on request, {:?}", error);
		return Ok(warp::reply::with_status(
			warp::reply::json(&Response {
				error: format!("{:?}", error).into(),
				success: false,
			}),
			warp::http::StatusCode::INTERNAL_SERVER_ERROR
		));
	}

	println!(" -> Valid request, set database and syncing...");
	if let Err(error) = tx.lock().await.send(types::WSCommand::Refresh) {
		eprintln!("WS could not send refresh through http -> ws channel {:?}", error);
	}

	Ok(warp::reply::with_status(
		warp::reply::json(&Response {
			error: None,
			success: true,
		}),
		warp::http::StatusCode::OK
	))
}

pub(crate) fn build_set_database(
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path("set-database"))
		.and(warp::body::content_length_limit(1024 * 100))
		.and(
			warp::header::<String>("secret")
				.map(|token: String| {
					token == get_todos_secret()
				})
		)
		.and(warp::any().map(move || tx.clone()))
		.and(warp::any().map(move || io.clone()))
		.and(warp::body::json())
		.and_then(set_database_endpoint)
}
