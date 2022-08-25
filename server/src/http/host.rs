use bansheelong_types::{ IO, get_todos_port, get_todos_secret };
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::ws::Message;

use crate::types;
use crate::http::{
	add_planned_meals,
	add_todos,
	add_recipes,
	get_database,
	remove_planned_meals,
	set_database
};
use crate::ws::{ Users, handler };

pub fn host<'a>(
	rx: mpsc::UnboundedReceiver<types::WSCommand>,
	tx: Arc<Mutex<mpsc::UnboundedSender<types::WSCommand>>>
) -> (Pin<Box<dyn Future<Output=()> + 'a>>, Pin<Box<dyn Future<Output=()> + 'a>>) {
	let users = Users::default();
	let borrowed = users.clone();
	let users_filter = warp::any().map(move || borrowed.clone());
	
	let ws_route = warp::path("websocket")
		.and(
			warp::header::<String>("secret")
				.map(|token: String| {
					token == get_todos_secret()
				})
		)
		.and(warp::ws())
		.and(users_filter)
		.and_then(handler);
	
	let io = Arc::new(Mutex::new(IO::default()));
	let routes = add_todos::build_add_todos(tx.clone(), io.clone())
		.or(set_database::build_set_database(tx.clone(), io.clone()))
		.or(get_database::build_get_database(io.clone()))
		.or(add_recipes::build_add_recipes(tx.clone(), io.clone()))
		.or(add_planned_meals::build_add_planned_meals(tx.clone(), io.clone()))
		.or(remove_planned_meals::build_remove_planned_meals(tx.clone(), io.clone()))
		.or(ws_route);

	let mut rx = UnboundedReceiverStream::new(rx);
	(
		Box::pin(async move {
			println!("Running HTTP server");
			warp::serve(routes).run(([0, 0, 0 ,0], get_todos_port())).await;
		}),
		Box::pin(async move { // send refresh messages to listeners
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
		}),
	)
}
