use std::sync::Arc;
use std::sync::Mutex;

mod host;
mod todo_database;
use crate::host::http;

use crate::todo_database::types::IO;

#[tokio::main]
async fn main() {
	let io = Arc::new(Mutex::new(IO::default()));
	println!("Running HTTP server");
	if let Err(error) = http::test(io.clone()).await {
		panic!("{:?}", error);
	}
}
