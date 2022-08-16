use iced::{ Command, Container, Element, Length, Row };

use crate::calendar;
use crate::constants;
use crate::meals;
use crate::menu::{ Menu };
use crate::todos;

#[derive(Debug)]
pub struct View {
	calendar: calendar::View,
	meals: meals::View,
	todos: todos::View,
	
	menu: Menu,
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
			meals: meals::View::new(),
			todos: todos::View::new(),
			
			menu: Menu::Todos,
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
			Menu::Meals => {
				Container::new(
					self.meals.view().map(move |message| {
						Message::MealsMessage(message)
					})
				)
			},
			Menu::Todos => {
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
						.width(Length::Units(constants::MENU_WIDTH))
				)
			},
		};

		Container::new(menu)
			.width(Length::Units(constants::MENU_WIDTH))
			.into()
	}
}
