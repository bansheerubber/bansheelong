mod http;
mod types;
mod ws;

use bansheelong_types::{
	IO,
	get_todos_https_cert,
	get_todos_https_key,
	get_todos_secret,
	get_todos_server_port,
};
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::ws::Message;

use futures::future;

use crate::http::{
	add_planned_meals,
	add_todos,
	add_recipes,
	get_database,
	remove_planned_meals,
	set_database
};
use crate::ws::Users;

#[tokio::main]
async fn main() {
	let users = Users::default();
	let borrowed = users.clone();
	let users_filter = warp::any().map(move || borrowed.clone());

	// communication between HTTP and WS
	let (tx, rx) = mpsc::unbounded_channel::<types::WSCommand>();
	let tx = Arc::new(Mutex::new(tx));
	let mut rx = UnboundedReceiverStream::new(rx);
	
	// set up warp routes
	let io = Arc::new(Mutex::new(IO::default()));
	let routes = add_todos::build_add_todos(tx.clone(), io.clone())
		.or(set_database::build_set_database(tx.clone(), io.clone()))
		.or(get_database::build_get_database(io.clone()))
		.or(add_recipes::build_add_recipes(tx.clone(), io.clone()))
		.or(add_planned_meals::build_add_planned_meals(tx.clone(), io.clone()))
		.or(remove_planned_meals::build_remove_planned_meals(tx.clone(), io.clone()))
		.or( // set up websocket
			warp::path("websocket")
				.and(
					warp::header::<String>("secret")
						.map(|token: String| {
							token == get_todos_secret()
						})
				)
				.and(warp::ws())
				.and(users_filter)
				.and_then(ws::handler)
		);
	
	// http server async block
	let http_server: Pin<Box<dyn Future<Output = ()>>> = Box::pin(async move {
		println!("Running HTTP server");
		warp::serve(routes)
			.tls()
			.cert_path(get_todos_https_cert())
			.key_path(get_todos_https_key())
			.run(([0, 0, 0, 0], get_todos_server_port())).await;
	});

	// ws message handler async block
	let ws_message_handler = Box::pin(async move { // send refresh messages to listeners
		while let Some(message) = rx.next().await {
			match message {
				types::WSCommand::Ping => {
					let users = users.write().await;
					for user in users.iter() {
						if let Err(error) = user.channel.send(Message::ping([])) {
							eprintln!("WS Error {:?}", error);
						}
					}
				},
				types::WSCommand::Refresh => {
					let users = users.write().await;
					for user in users.iter() {
						if let Err(error) = user.channel.send(Message::text("refresh")) {
							eprintln!("WS Error {:?}", error);
						}
					}
				}
			}
		}
	});

	// ws keep alive async block
	let ws_keep_alive = Box::pin(async {
		loop {
			tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
			if let Err(error) = tx.lock().await.send(types::WSCommand::Ping) {
				panic!("{:?}", error);	
			}
		}
	});

	future::join_all([http_server, ws_message_handler, ws_keep_alive]).await;
}
