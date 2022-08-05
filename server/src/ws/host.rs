use std::sync::atomic::{ AtomicUsize, Ordering };
use std::sync::Arc;

use futures::{ StreamExt, SinkExt, TryFutureExt, future };
use warp::ws::{ Message, WebSocket };
use warp::Filter;
use warp::reply::Reply;
use warp::reject::Rejection;

use tokio::sync::{ RwLock, mpsc };
use tokio_stream::wrappers::UnboundedReceiverStream;
use bansheelong_types::{ get_todos_port, get_todos_secret };

use crate::types;

struct User {
	id: usize,
	channel: mpsc::UnboundedSender<Message>,
}

type Users = Arc<RwLock<Vec<User>>>;
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(0);

async fn user_connected(ws: WebSocket, users: Users) {
	let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

	println!("WS User {} connected", id);

	let (mut user_ws_tx, mut user_ws_rx) = ws.split();

	let (tx, rx) = mpsc::unbounded_channel();
	let mut rx = UnboundedReceiverStream::new(rx);

	tokio::task::spawn(async move {
		while let Some(message) = rx.next().await {
			user_ws_tx
				.send(message)
				.unwrap_or_else(|e| {
					eprintln!("websocket send error: {}", e);
				})
				.await;
		}
	});

	users.write().await.push(User {
		id,
		channel: tx,
	});

	loop {
		if let None = user_ws_rx.next().await { // ignore user communication until they disconnect
			break;
		}
	}

	user_disconnect(id, users).await;
}

async fn user_disconnect(user_id: usize, users: Users) {
	let index = users.read().await.iter().position(|u| u.id == user_id);
	if let None = index {
		eprintln!("WS Could not remove user {}", user_id);
	}

	println!("WS User {} disconnected", user_id);
	users.write().await.remove(index.unwrap());
}

async fn handler(correct: bool, ws: warp::ws::Ws, users_filter: Arc<RwLock<Vec<User>>>) -> Result<impl Reply, Rejection> {
	if correct {
		Ok(ws.on_upgrade(move |socket| user_connected(socket, users_filter)))
	} else {
		Err(warp::reject::not_found())
	}
}

pub async fn host(rx: mpsc::UnboundedReceiver<types::WSCommand>) {
	let mut rx = UnboundedReceiverStream::new(rx);
	
	let users = Users::default();
	let borrowed = users.clone();
	let users_filter = warp::any().map(move || borrowed.clone());
	
	let route = warp::path("websocket")
		.and(
			warp::header::<String>("secret")
				.map(|token: String| {
					token == get_todos_secret()
				})
		)
		.and(warp::ws())
		.and(users_filter)
		.and_then(handler);
	
	future::join(
		warp::serve(route).run(([0, 0, 0, 0], get_todos_port() + 1)),
		async { // send refresh messages to listeners
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
		}
	).await;
}
