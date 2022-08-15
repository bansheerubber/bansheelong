use std::sync::{ Arc };
use std::convert::Infallible;

use futures::{ TryStreamExt };
use hyper::service::{ make_service_fn, service_fn };
use hyper::{ Body, Method, Request, Response, Server, StatusCode };

use tokio::sync::{ Mutex, mpsc };
use tokio;

use crate::types;
use bansheelong_types::{ Date, Dirty, IO, Item, MealsDatabase, TodosDatabase, get_todos_port, get_todos_secret };

// TODO move this under warp framework
async fn service(
	request: Request<Body>,
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>,
	io: Arc<Mutex<IO>>
) -> Result<Response<Body>, Infallible> {
	println!("{} {}", request.method(), request.uri().to_string());
	let (parts, body) = request.into_parts();

	let secret = parts.headers.get("Secret");
	if secret.is_none() || secret.unwrap().to_str().is_err() || secret.unwrap().to_str().unwrap() != get_todos_secret().as_str() {
		eprintln!(" -> Secret not in request");
		return Ok(
			Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body("404".into())
				.unwrap()
		);
	}

	match (parts.method, parts.uri.path()) {
		(Method::GET, "/get-database/") | (Method::GET, "/get-database") => {
			let mut guard = io.lock().await;
			let result = guard.read_database().await;
			if let Err(error) = result {
				eprintln!(" -> Error on request, {:?}", error);
				return Ok(
					Response::builder()
						.status(StatusCode::INTERNAL_SERVER_ERROR)
						.body(format!("{:?}", error).into())
						.unwrap()
				);
			}

			println!(" -> Valid request, reading todos from file and sending...");

			Ok(
				Response::builder()
					.status(StatusCode::OK)
					.body(serde_json::to_string(&result.unwrap()).unwrap().into())
					.unwrap()
			)
		},
		(Method::POST, "/add-todos/") | (Method::POST, "/add-todos") => {			
			let json = match String::from_utf8(
				body.try_fold(Vec::new(), |mut data, chunk| async move {
					data.extend_from_slice(&chunk);
					Ok(data)
				})
				.await.unwrap()
			) { // make sure we convert utf8 correctly
				Ok(string) => string,
				Err(error) => {
					eprintln!(" -> Error on request, {:?}", error);
					return Ok(
						Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(format!("{:?}", error).into())
							.unwrap()
					);
				}
			};

			match serde_json::from_str::<Vec<(Item, Option<Date>)>>(&json) { // parse JSON
				Ok(items) => {
					let mut guard = io.lock().await;

					for (item, date) in items { // add items to database
						let result = guard.add_to_todos_database(item, date);
						if let Err(error) = result {
							eprintln!(" -> Error on request, {:?}", error);
							return Ok(
								Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(format!("{:?}", error).into())
									.unwrap()
							);
						}
					}

					if let Err(error) = guard.sync().await { // sync
						eprintln!(" -> Error on request, {:?}", error);
						return Ok(
							Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(format!("{:?}", error).into())
								.unwrap()
						);
					}
				},
				Err(error) => {
					eprintln!(" -> Error on request, {:?}", error);
					return Ok(
						Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(format!("{:?}", error).into())
							.unwrap()
					);
				}
			}

			println!(" -> Valid request, adding todos and syncing...");
			if let Err(error) = tx.lock().await.send(types::WSCommand::Refresh) {
				eprintln!("WS Could not send refresh through http -> ws channel {:?}", error);
			}

			Ok(
				Response::builder()
					.status(StatusCode::OK)
					.body("Ok".into())
					.unwrap()
			)
		},
		(Method::POST, "/set-database/") | (Method::POST, "/set-todos") => {
			let json = match String::from_utf8(
				body.try_fold(Vec::new(), |mut data, chunk| async move {
					data.extend_from_slice(&chunk);
					Ok(data)
				})
				.await.unwrap()
			) { // make sure we convert utf8 correctly
				Ok(string) => string,
				Err(error) => {
					eprintln!(" -> Error on request, {:?}", error);
					return Ok(
						Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(format!("{:?}", error).into())
							.unwrap()
					);
				}
			};

			match serde_json::from_str::<(TodosDatabase, MealsDatabase)>(&json) { // parse JSON
				Ok(databases) => {
					let mut guard = io.lock().await;
					guard.meals_database = databases.1;
					guard.todos_database = databases.0;
					guard.dirty = Dirty::Write;
					if let Err(error) = guard.sync().await {
						eprintln!(" -> Error on request, {:?}", error);
						return Ok(
							Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(format!("{:?}", error).into())
								.unwrap()
						);
					}
				},
				Err(error) => {
					eprintln!(" -> Error on request, {:?}", error);
					return Ok(
						Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(format!("{:?}", error).into())
							.unwrap()
					);
				}
			}

			println!(" -> Valid request, set todo database and syncing...");
			if let Err(error) = tx.lock().await.send(types::WSCommand::Refresh) {
				eprintln!("WS Could not send refresh through http -> ws channel {:?}", error);
			}

			Ok(
				Response::builder()
					.status(StatusCode::OK)
					.body("Ok".into())
					.unwrap()
			)
		},
		_ => {
			Ok(
				Response::builder()
					.status(StatusCode::NOT_FOUND)
					.body("404".into())
					.unwrap()
			)
		}
	}
}

pub async fn host(tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>, io: Arc<Mutex<IO>>) -> hyper::Result<()> {
	let make_svc = make_service_fn(|_conn| {
		let io = io.clone();
		let tx = tx.clone();
		async { Ok::<_, Infallible>(service_fn(move |request| {
			service(request, tx.clone(), io.clone())
		})) }
	});

	let addr = ([0, 0, 0, 0], get_todos_port()).into();
	Server::bind(&addr).serve(make_svc).await
}
