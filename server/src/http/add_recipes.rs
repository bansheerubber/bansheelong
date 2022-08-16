use std::sync::Arc;
use std::convert::Infallible;
use tokio::sync::{ Mutex, mpsc };
use warp::Filter;

use crate::http::{ Response, failed_secret };
use crate::types;

use bansheelong_types::{ IO, Recipe, get_todos_secret };

async fn add_recipes_endpoint(
	secret: bool,
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>,
	items: Vec<Recipe>
) -> Result<impl warp::Reply, Infallible> {
	println!("POST /add-recipes/");
	
	if !secret {
		return Ok(failed_secret());
	}
	
	let mut guard = io.lock().await;

	for recipe in items { // add recipes to database
		let result = guard.add_recipe(recipe);
		if let Err(error) = result {
			eprintln!(" -> Error on request, {:?}", error);
			return Ok(warp::reply::with_status(
				warp::reply::json(&Response {
					error: format!("{:?}", error).into(),
					success: false,
				}),
				warp::http::StatusCode::INTERNAL_SERVER_ERROR
			));
		}
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
		eprintln!("WS Could not send refresh through http -> ws channel {:?}", error);
	}
	
	Ok(warp::reply::with_status(
		warp::reply::json(&Response {
			error: None,
			success: true,
		}),
		warp::http::StatusCode::OK
	))
}

pub(crate) fn build_add_recipes(
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path("add-recipes"))
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
		.and_then(add_recipes_endpoint)
}
