use futures::{ StreamExt, SinkExt, TryFutureExt };
use std::sync::atomic::{ AtomicUsize, Ordering };
use std::sync::Arc;
use tokio::sync::{ RwLock, mpsc };
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{ Message, WebSocket };
use warp::reply::Reply;
use warp::reject::Rejection;

pub struct User {
	pub id: usize,
	pub channel: mpsc::UnboundedSender<Message>,
}

pub type Users = Arc<RwLock<Vec<User>>>;
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
		eprintln!("WS could not remove user {}", user_id);
	}

	println!("WS User {} disconnected", user_id);
	users.write().await.remove(index.unwrap());
}

pub async fn handler(correct: bool, ws: warp::ws::Ws, users_filter: Arc<RwLock<Vec<User>>>) -> Result<impl Reply, Rejection> {
	if correct {
		Ok(ws.on_upgrade(move |socket| user_connected(socket, users_filter)))
	} else {
		Err(warp::reject::not_found())
	}
}
