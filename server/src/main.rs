use futures::future;
use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };

mod http;
mod types;
mod ws;

#[tokio::main]
async fn main() {
	let (tx, rx) = mpsc::unbounded_channel::<types::WSCommand>();

	let tx = Arc::new(Mutex::new(tx));

	let http_ws_host = http::host(rx, tx.clone());

	let ws_ping = Box::pin(async {
		loop {
			tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
			if let Err(error) = tx.lock().await.send(types::WSCommand::Ping) {
				panic!("{:?}", error);	
			}
		}
	});

	future::join_all([http_ws_host.0, http_ws_host.1, ws_ping]).await;

	println!("Did we reach here");
}
