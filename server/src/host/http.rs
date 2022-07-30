use std::sync::{ Arc };
use std::collections::HashMap;
use std::convert::Infallible;

use hyper::service::{ make_service_fn, service_fn };
use hyper::{ Body, Method, Request, Response, Server, StatusCode };

use tokio::sync::Mutex;

use bansheelong_types::{ Date, IO, Item };

async fn service(request: Request<Body>, io: Arc<Mutex<IO>>) -> Result<Response<Body>, Infallible> {
	println!("{}", request.uri().to_string());
	match (request.method(), request.uri().path()) {
		(&Method::GET, "/get-todos/") | (&Method::GET, "/get-todos") => {
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
		(&Method::GET, "/add-todo/") | (&Method::GET, "/add-todo") => {
			let parameters = request.uri().query()
				.map(|v| {
					url::form_urlencoded::parse(v.as_bytes())
						.into_owned()
						.collect()
				})
				.unwrap_or_else(HashMap::new);

			// test for required parameters
			if !parameters.contains_key("description") {
				eprintln!(" -> Error on request, no description");
				return Ok(
					Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body("404".into())
						.unwrap()
				)	;
			}
			
			let mut guard = io.lock().await;
			let result = guard.add_to_database_sync(Item::new(&parameters), Date::new(&parameters)).await;
			if let Err(error) = result {
				eprintln!(" -> Error on request, {:?}", error);
				return Ok(
					Response::builder()
						.status(StatusCode::INTERNAL_SERVER_ERROR)
						.body(format!("{:?}", error).into())
						.unwrap()
				);
			}
			
			println!(" -> Valid request, adding todo and syncing...");

			Ok(
				Response::builder()
					.status(StatusCode::OK)
					.body(serde_json::to_string(&result.unwrap()).unwrap().into())
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

	let addr = ([127, 0, 0, 1], 3000).into();
	Server::bind(&addr).serve(make_svc).await
}
