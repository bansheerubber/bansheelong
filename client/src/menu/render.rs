use bansheelong_shared_ui::{ constants, meals };
use iced::{ Command, Container, Element, Length, Row };

use crate::calendar;
use crate::menu::MENU_STATE;
use crate::state::WINDOW_STATE;
use crate::todos;

#[derive(Debug)]
pub struct View {
	calendar: calendar::View,
	meals: meals::View,
	todos: todos::View,
	
	menu: constants::Menu,
}

#[derive(Debug, Clone)]
pub enum Message {
	CalendarMessage(calendar::Message),
	MealsMessage(meals::Message),
	Tick,
	TodosMessage(todos::Message),
}

impl View {
	pub fn new() -> Self {
		View {
			calendar: calendar::View::new(),
			meals: meals::View::new(MENU_STATE.clone(), WINDOW_STATE, [20, 15, 20, 0]),
			todos: todos::View::new(),
			
			menu: constants::Menu::Todos,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::CalendarMessage(message) => {
				self.calendar.update(message).map(move |message| {
					Message::CalendarMessage(message)
				})
			},
			Message::MealsMessage(message) => {
				if let meals::Message::MenuChange(menu) = message {
					self.menu = menu;
				}
				
				self.meals.update(message).map(move |message| {
					Message::MealsMessage(message)
				})
			},
			Message::Tick => {
				Command::batch([
					self.calendar.update(calendar::Message::Tick).map(move |message| {
						Message::CalendarMessage(message)
					}),
					self.meals.update(meals::Message::Tick).map(move |message| {
						Message::MealsMessage(message)
					}),
					self.todos.update(todos::Message::Tick).map(move |message| {
						Message::TodosMessage(message)
					}),
				])
			},
			Message::TodosMessage(message) => {
				if let todos::Message::MenuChange(menu) = message {
					self.menu = menu;
				}
				
				self.todos.update(message).map(move |message| {
					Message::TodosMessage(message)
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
			constants::Menu::Todos => {
				Container::new(
					Row::new()
						.push( // todo list
							self.todos.view().map(move |message| {
								Message::TodosMessage(message)
							})
						)
						.push( // calendar bar
							self.calendar.view().map(move |message| {
								Message::CalendarMessage(message)
							})
						)
						.width(Length::Units(MENU_STATE.width))
				)
			},
		};

		Container::new(menu)
			.width(Length::Units(MENU_STATE.width))
			.into()
	}
}
