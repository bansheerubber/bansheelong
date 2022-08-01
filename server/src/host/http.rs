use std::sync::{ Arc };
use std::convert::Infallible;

use futures::{ TryStreamExt };
use hyper::service::{ make_service_fn, service_fn };
use hyper::{ Body, Method, Request, Response, Server, StatusCode };

use tokio::sync::Mutex;
use tokio;

use bansheelong_types::{ Date, Database, Dirty, IO, Item };

async fn service(request: Request<Body>, io: Arc<Mutex<IO>>) -> Result<Response<Body>, Infallible> {
	println!("{} {}", request.method(), request.uri().to_string());
	let (parts, body) = request.into_parts();
	match (parts.method, parts.uri.path()) {
		(Method::GET, "/get-todos/") | (Method::GET, "/get-todos") => {
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
						let result = guard.add_to_database(item, date);
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

			Ok(
				Response::builder()
					.status(StatusCode::OK)
					.body("Ok".into())
					.unwrap()
			)
		},
		(Method::POST, "/set-todos/") | (Method::POST, "/set-todos") => {
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

			match serde_json::from_str::<Database>(&json) { // parse JSON
				Ok(database) => {
					let mut guard = io.lock().await;
					guard.database = database;
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

pub async fn host(io: Arc<Mutex<IO>>) -> hyper::Result<()> {
	let make_svc = make_service_fn(|_conn| {
		let io = io.clone();
		async { Ok::<_, Infallible>(service_fn(move |request| {
			let io = io.clone();
			service(request, io)
		})) }
	});

	let addr = ([192, 168, 0, 83], 3000).into();
	Server::bind(&addr).serve(make_svc).await
}
