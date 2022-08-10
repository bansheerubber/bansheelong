use iced_native::subscription::{ self, Subscription };

use tokio::time::{ Duration, sleep };
use tokio::net::TcpStream;

use bansheelong_types::{ get_storage_port, get_storage_host };

#[derive(Debug)]
enum State {
	Connected(TcpStream),
	Disconnected,
	WaitToConnect,
}

#[derive(Debug, Clone)]
pub enum Event {
	Error(String),
	Ignore,
	InvalidateState,
	Message(String),
}

pub fn connect() -> Subscription<Event> {
	struct Connect;

	subscription::unfold(
		std::any::TypeId::of::<Connect>(),
		State::Disconnected,
		|state| async move {
			match state {
				State::Connected(socket) => { // receive messages in a way that is friendly to iced subscriptions
					if let Err(error) = socket.readable().await {
						eprintln!("TCP error {:?}", error);
						return (Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect);
					}

					let mut buffer = Vec::new();
					match socket.try_read_buf(&mut buffer) {
						Ok(0) => {
							(Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect)
						},
						Ok(_) => {
							let message = match String::from_utf8(buffer) {
								Ok(string) => string,
								Err(error) => {
									eprintln!("TCP error {:?}", error);
									return (Some(Event::Error(String::from("Malformed message"))), State::WaitToConnect);
								},
							};

							sleep(Duration::from_secs(1)).await;

							(Some(Event::Message(message)), State::Connected(socket))
						},
						Err(ref error) if error.kind() == tokio::io::ErrorKind::WouldBlock => {
							sleep(Duration::from_secs(1)).await;
							(Some(Event::Ignore), State::Connected(socket))
						},
						Err(error) => {
							eprintln!("TCP error {:?}", error);
							(Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect)
						},
					}
				},
				State::Disconnected => { // try connecting if we're disconnected
					match TcpStream::connect(
						format!("{}:{}", get_storage_host(), get_storage_port())
					).await {
						Ok(socket) => (Some(Event::InvalidateState), State::Connected(socket)),
						Err(error) => {
							eprintln!("TCP error {:?}", error);
							(Some(Event::Error(String::from("Could not connect"))), State::WaitToConnect)
						},
					}
				},
				State::WaitToConnect => { // sleep so we don't DoS our poor server
					sleep(Duration::from_secs(5)).await;
					(Some(Event::InvalidateState), State::Disconnected)
				}
			}
		}
	)
}

