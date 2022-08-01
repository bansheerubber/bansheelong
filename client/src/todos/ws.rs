use futures::StreamExt;
use iced_native::subscription::{ self, Subscription };

use async_tungstenite::tokio::{ ConnectStream, connect_async };
use async_tungstenite::tungstenite;

#[derive(Debug)]
enum State {
	Disconnected,
	Connected(async_tungstenite::WebSocketStream<ConnectStream>),
}

#[derive(Debug, Clone)]
pub enum Event {
	Refresh,
}

pub fn ws() -> Subscription<Event> {
	struct Connect;

	subscription::unfold(
		std::any::TypeId::of::<Connect>(),
		State::Disconnected,
		|state| async move {
			match state {
				State::Disconnected => { // try connecting if we're disconnected
					match connect_async("ws://bansheerubber:3001/websocket").await {
						Ok((websocket, _)) => {
							return (None, State::Connected(websocket));
						},
						Err(error) => {
							eprintln!("{}", error);
							return (None, State::Disconnected);
						}
					};
				},
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
						Ok(_) => (None, State::Connected(websocket)),
						Err(error) => {
							eprintln!("{}", error);
							(None, State::Disconnected)
						}
					}
				},
			}
		}
	)
}
