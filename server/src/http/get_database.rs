use std::sync::Arc;
use std::convert::Infallible;
use tokio::sync::Mutex;
use warp::Filter;

use crate::http::{ Response, failed_secret };

use bansheelong_types::{ IO, get_todos_secret };

async fn get_database_endpoint(
	secret: bool,
	io: Arc<Mutex<IO>>
) -> Result<impl warp::Reply, Infallible> {
	println!("GET /get-database/");
	
	if !secret {
		return Ok(failed_secret());
	}
	
	let mut guard = io.lock().await;
	let result = guard.read_database().await;
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

	println!(" -> Valid request, reading todos from file and sending...");

	Ok(warp::reply::with_status(
		warp::reply::json(&result.unwrap()),
		warp::http::StatusCode::OK
	))
}

pub(crate) fn build_get_database(
	io: Arc<Mutex<IO>>
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path("get-database"))
		.and(
			warp::header::<String>("secret")
				.map(|token: String| {
					token == get_todos_secret()
				})
		)
		.and(warp::any().map(move || io.clone()))
		.and_then(get_database_endpoint)
}
