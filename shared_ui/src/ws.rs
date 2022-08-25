use async_tungstenite::tokio::{ ConnectStream, connect_async };
use async_tungstenite::tungstenite;
use bansheelong_types::{ get_todos_port, get_todos_host, get_todos_secret };
use futures::StreamExt;
use iced_native::subscription::{ self, Subscription };
use tokio::time::{ Duration, sleep };
use tungstenite::handshake::client::Request;

#[derive(Debug)]
enum State {
	Connected(async_tungstenite::WebSocketStream<ConnectStream>),
	Disconnected,
	WaitToConnect,
}

#[derive(Debug, Clone)]
pub enum Event {
	Error(String),
	InvalidateState,
	Refresh,
}

pub fn connect() -> Subscription<Event> {
	struct Connect;

	subscription::unfold(
		std::any::TypeId::of::<Connect>(),
		State::Disconnected,
		|state| async move {
			match state {
				State::Connected(mut websocket) => { // receive messages in a way that is friendly to iced subscriptions
					let mut fused = websocket.by_ref().fuse();
					match fused.select_next_some().await {
						Ok(tungstenite::Message::Text(message)) => {
							if message == "refresh" {
								(Some(Event::Refresh), State::Connected(websocket))
							} else {
								(None, State::Connected(websocket))
							}
						},
						Ok(tungstenite::Message::Close(_)) => {
							(Some(Event::Error(String::from("Lost connection"))), State::Disconnected)
						},
						Ok(_) => (None, State::Connected(websocket)),
						Err(error) => {
							eprintln!("WS error {}", error);
							(Some(Event::Error(String::from("Lost connection"))), State::Disconnected)
						}
					}
				},
				State::Disconnected => { // try connecting if we're disconnected
					match connect_async(
						Request::builder()
							.uri(format!("ws://{}:{}/websocket", get_todos_host(), get_todos_port()))
							.header("Secret", get_todos_secret())
							.body(())
							.unwrap()
					).await {
						Ok((websocket, _)) => {
							return (Some(Event::Refresh), State::Connected(websocket)); // force refresh when websocket is established
						},
						Err(error) => {
							eprintln!("WS error {}", error);
							return (Some(Event::Error(String::from("Could not connect"))), State::WaitToConnect);
						}
					};
				},
				State::WaitToConnect => { // sleep so we don't DoS our poor server
					sleep(Duration::from_secs(5)).await;
					return (Some(Event::InvalidateState), State::Disconnected);
				}
			}
		}
	)
}
