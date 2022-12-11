use bansheelong_types::{ JobStatusFlags, STORAGE_MESSAGE_COUNT, get_storage_port, get_storage_host };
use iced_native::subscription::{ self, Subscription };
use tokio::time::{ Duration, sleep };
use tokio::net::TcpStream;

#[derive(Debug)]
enum State {
	Connected(TcpStream, String),
	Disconnected,
	WaitToConnect,
}

#[derive(Clone, Debug)]
pub struct Data {
	pub job_flags: JobStatusFlags,

	pub used_size: u64,
	pub total_size: u64,

	pub btrfs_used_size: u64,
	pub btrfs_total_size: u64,
	pub btrfs_backup_count: u64,

	pub dailies: u8,
	pub weeklies: u8,
}

#[derive(Clone, Debug)]
pub enum Event {
	Error(String),
	Ignore,
	InvalidateState,
	Message(Data),
}

pub fn connect() -> Subscription<Event> {
	struct Connect;

	subscription::unfold(
		std::any::TypeId::of::<Connect>(),
		State::Disconnected,
		|state| async move {
			match state {
				State::Connected(socket, mut buffer) => { // receive messages in a way that is friendly to iced subscriptions
					if let Err(error) = socket.readable().await {
						eprintln!("TCP error {:?}", error);
						return (Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect);
					}

					let mut packet = Vec::new();
					match socket.try_read_buf(&mut packet) {
						Ok(0) => {
							eprintln!("TCP error lost connection");
							return (Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect);
						},
						Err(ref error) if error.kind() == tokio::io::ErrorKind::WouldBlock => {
							sleep(Duration::from_secs(1)).await;
							return (Some(Event::Ignore), State::Connected(socket, buffer));
						},
						Err(error) => {
							eprintln!("TCP error {:?}", error);
							return (Some(Event::Error(String::from("Lost connection"))), State::WaitToConnect);
						},
						Ok(_) => {
							let message = match String::from_utf8(packet) {
								Ok(string) => string,
								Err(error) => {
									eprintln!("TCP error {:?}", error);
									return (Some(Event::Error(String::from("Malformed message"))), State::WaitToConnect);
								},
							};

							buffer.push_str(&message);
						},
					}

					if buffer.contains("\n") {
						let split = buffer.split("\n").filter(|s| s.len() > 0).collect::<Vec<&str>>();

						let last_valid_message = if buffer.ends_with("\n") {
							split.len() - 1
						} else {
							split.len() - 2
						};

						// parse the last message only
						let parts: Vec<u64> = split[last_valid_message].split(" ").map(|x| {
							match x.parse::<u64>() {
								Err(_) => 0,
								Ok(value) => value
							}
						}).collect();

						if parts.len() != STORAGE_MESSAGE_COUNT as usize {
							eprintln!("TCP error message not right length");
							return (Some(Event::Error(String::from("Malformed message"))), State::WaitToConnect);
						}

						sleep(Duration::from_secs(1)).await;

						return (
							Some(Event::Message(Data {
								job_flags: if JobStatusFlags::from_bits(parts[0] as u64).is_none() {
									JobStatusFlags::GENERAL_ERROR
								} else {
									JobStatusFlags::from_bits(parts[0] as u64).unwrap()
								},

								used_size: parts[1],
								total_size: parts[2],

								btrfs_used_size: parts[3],
								btrfs_total_size: parts[4],
								btrfs_backup_count: parts[5],

								dailies: parts[6] as u8,
								weeklies: parts[7] as u8,
							})),
							State::Connected(socket, String::new())
						);
					} else {
						return (Some(Event::Ignore), State::Connected(socket, buffer));
					}
				},
				State::Disconnected => { // try connecting if we're disconnected
					match TcpStream::connect(
						format!("{}:{}", get_storage_host(), get_storage_port())
					).await {
						Ok(socket) => (Some(Event::InvalidateState), State::Connected(socket, String::new())),
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

