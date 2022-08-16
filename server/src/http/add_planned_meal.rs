use std::sync::Arc;
use std::convert::Infallible;
use tokio::sync::{ Mutex, mpsc };
use warp::Filter;

use crate::http::{ Response, failed_secret };
use crate::types;

use bansheelong_types::{ Date, IO, Recipe, get_todos_secret };

async fn add_planned_meal_endpoint(
	secret: bool,
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>,
	info: (Recipe, Date)
) -> Result<impl warp::Reply, Infallible> {
	println!("POST /add-planned-meal/");
	
	if !secret {
		return Ok(failed_secret());
	}
	
	let mut guard = io.lock().await;

	let (recipe, date) = info;

	if let Err(error) = guard.add_planned_meal(recipe, date) { // add planned meal
		eprintln!(" -> Error on request, {:?}", error);
		return Ok(warp::reply::with_status(
			warp::reply::json(&Response {
				error: format!("{:?}", error).into(),
				success: false,
			}),
			warp::http::StatusCode::INTERNAL_SERVER_ERROR
		));
	}

	if let Err(error) = guard.sync().await { // sync
		eprintln!(" -> Error on request, {:?}", error);
		return Ok(warp::reply::with_status(
			warp::reply::json(&Response {
				error: format!("{:?}", error).into(),
				success: false,
			}),
			warp::http::StatusCode::INTERNAL_SERVER_ERROR
		));
	}

	println!(" -> Valid request, adding recipes and syncing...");
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

pub(crate) fn build_add_planned_meal(
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path("add-planned-meal"))
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
		.and_then(add_planned_meal_endpoint)
}
