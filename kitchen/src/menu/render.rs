use bansheelong_shared_ui::{ constants, meals };
use iced::{ Command, Container, Element, Length, Space };

use crate::menu::MENU_STATE;
use crate::state::WINDOW_STATE;

#[derive(Debug)]
pub struct View {
	meals: meals::View,
	menu: constants::Menu,
}

#[derive(Debug, Clone)]
pub enum Message {
	MealsMessage(meals::Message),
	Tick,
}

impl View {
	pub fn new() -> Self {
		View {
			meals: meals::View::new(
				meals::Props {
					calendar_day_size: 65,
					calendar_day_spacing: 6,
					calendar_day_text_size: 25,
					calendar_month_text_size: 35,
					ingredient_list_width: 400,
					text_size: 25,
				},
				MENU_STATE.clone(),
				WINDOW_STATE,
				[20, 15, 20, 0]
			),
			menu: constants::Menu::Meals,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::MealsMessage(message) => {
				if let meals::Message::MenuChange(menu) = message {
					self.menu = menu;
				}
				
				self.meals.update(message).map(move |message| {
					Message::MealsMessage(message)
				})
			},
			Message::Tick => {
				self.meals.update(meals::Message::Tick).map(move |message| {
					Message::MealsMessage(message)
				})
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let menu = match self.menu {
			constants::Menu::Meals => {
				Container::new(
					self.meals.view().map(move |message| {
						Message::MealsMessage(message)
					})
				)
			},
			_ => {
				Container::new(
					Space::new(Length::Units(0), Length::Units(0))
				)
			},
		};

		Container::new(menu)
			.width(Length::Units(MENU_STATE.width))
			.into()
	}
}
