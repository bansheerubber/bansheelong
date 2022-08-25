use bansheelong_types::{ IO, Resource };

#[tokio::main]
async fn main() {
	let mut io = IO::default();
	io.resource = Resource {
		reference: String::from("todos"),
	};

	if let Err(error) = io.read_database().await {
		eprintln!("{:?}", error);
	}

	println!("{:?}", io);
}
