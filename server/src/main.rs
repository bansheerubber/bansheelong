use std::sync::Arc;
use std::sync::Mutex;

mod host;
use crate::host::http;

use bansheelong_types::IO;

#[tokio::main]
async fn main() {
	let io = Arc::new(Mutex::new(IO::default()));
	println!("Running HTTP server");
	if let Err(error) = http::test(io.clone()).await {
		panic!("{:?}", error);
	}
}
