use std::sync::Arc;
use std::time::{ Duration, Instant };

use chrono::{ Local, Timelike };

use iced::scrollable;
use iced::{ Command, Element, Length, Scrollable };

use bansheelong_types::{ IO };

use crate::constants;
use crate::style;
use super::calendar;

#[derive(Debug)]
pub struct View {
	last_interaction: Option<Instant>,
	scrollable_state: scrollable::State,
	todos: Option<Arc<IO>>,
}

#[derive(Debug, Clone)]
pub enum Message {
	Scroll(f32),
	Tick,
	Update(Option<Arc<IO>>),
}

impl View {
	pub fn new() -> Self {
		View {
			last_interaction: None,
			scrollable_state: scrollable::State::new(),
			todos: None,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Scroll(_) => {
				self.last_interaction = Some(Instant::now());
			},
			Message::Tick => {
				if self.last_interaction.is_none() || Instant::now() - self.last_interaction.unwrap() > Duration::from_secs(120) {
					let time = Local::now();
					let current_hour = time.hour() as i32;
					let current_minute = time.minute() as i32;
					let current_seconds = time.second() as i32;

					let height = (calendar::END_TIME - calendar::START_TIME + 1) as f32 * calendar::TEXT_SPACING.y + calendar::Y_OFFSET + 20.0 - constants::WINDOW_HEIGHT as f32;
					let time_height
						= calendar::TEXT_SPACING.y * (current_hour - calendar::START_TIME) as f32 + calendar::TEXT_SPACING.y * (current_minute as f32 / 60.0)  + calendar::TEXT_SPACING.y * (current_seconds as f32 / 60.0 / 60.0) + calendar::Y_OFFSET + 20.0 - (constants::WINDOW_HEIGHT / 2) as f32;

					self.scrollable_state.snap_to(time_height / height);
				}
			},
			Message::Update(io) => {
				self.todos = io;
			}
		}
		
		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		Scrollable::new(&mut self.scrollable_state)
			.width(Length::Units(355))
			.height(Length::Fill)
			.padding([20, 15, 20, 5])
			.style(style::TodoScrollable)
			.push(
				calendar::Calendar::new(self.todos.clone())
					.width(Length::Fill)
			)
			.on_scroll(move |offset| Message::Scroll(offset))
			.into()
	}
}
