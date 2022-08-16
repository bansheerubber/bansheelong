use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };
use std::pin::Pin;
use std::future::Future;

use futures::future;

mod http;
mod types;
mod ws;

use bansheelong_types::IO;

#[tokio::main]
async fn main() {
	let (tx, rx) = mpsc::unbounded_channel::<types::WSCommand>();

	let tx = Arc::new(Mutex::new(tx));

	let http_host: Pin<Box<dyn Future<Output = ()>>> = Box::pin(async {
		let io = Arc::new(Mutex::new(IO::default()));
		println!("Running HTTP server");
		http::host(tx.clone(), io.clone()).await;
	});

	let ws_host = Box::pin(async {
		println!("Running WS server");
		ws::host(rx).await;
	});

	let ws_ping = Box::pin(async {
		loop {
			tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
			if let Err(error) = tx.lock().await.send(types::WSCommand::Ping) {
				panic!("{:?}", error);	
			}
		}
	});

	future::join_all([http_host, ws_host, ws_ping]).await;

	println!("Did we reach here");
}
