use std::sync::Arc;
use tokio::sync::{ Mutex, mpsc };

use futures::future;

mod http;
mod types;
mod ws;

use bansheelong_types::IO;

#[tokio::main]
async fn main() {
	let (tx, rx) = mpsc::unbounded_channel::<types::WSCommand>();
	future::join(
		async {
			let io = Arc::new(Mutex::new(IO::default()));
			println!("Running HTTP server");
			if let Err(error) = http::host(tx, io.clone()).await {
				panic!("{:?}", error);
			}
		},
		async {
			println!("Running WS server");
			ws::host(rx).await;
		}
	).await;

	println!("Did we reach here");
}
