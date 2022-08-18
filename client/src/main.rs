mod calendar;
mod flavor;
mod meals;
mod menu;
mod state;
mod storage;
mod todos;
mod weather;

use std::sync::Arc;
use std::time::{ Duration, Instant };

use bansheelong_types::{ Date, Error, IO, MealsDatabase, PlannedMeal, PlannedMealsWriteLog, Resource, TodosDatabase, WriteDatabase, get_todos_host, get_todos_port, read_database, write_database };
use bansheelong_shared_ui::style;
use iced::alignment;
use iced::executor;
use iced::{ Application, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text };

struct Window {
	flavor: flavor::View,
	menu: menu::View,
	storage: storage::View,
	weather: weather::View,

	last_update_to_log: Instant,
	io: Arc<IO>,
	update_log: PlannedMealsWriteLog,
}

#[derive(Debug)]
enum Message {
	AddPlannedMeal(PlannedMeal),
	FetchedTodos(Result<(TodosDatabase, MealsDatabase), Error>),
	FlavorMessage(flavor::Message),
	MenuMessage(menu::Message),
	Noop,
	Refresh,
	RefreshTodos,
	RemovePlannedMeal(Date),
	StorageMessage(storage::Message),
	Tick,
	UpdatePlannedMeal(PlannedMeal),
	WeatherMessage(weather::Message),
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let resource = Resource {
			reference: format!("http://{}:{}", get_todos_host(), get_todos_port()),
		};

		(
			Window {
				flavor: flavor::View::new(),
				menu: menu::View::new(),
				storage: storage::View::new(),
				weather: weather::View::new(),

				last_update_to_log: Instant::now(),
				io: Arc::new(IO {
					resource: resource.clone(),
					..IO::default()
				}),
				update_log: Vec::new(),
			},
			Command::batch([
				Command::perform(weather::api::dial(), move |result| {
					Self::Message::WeatherMessage(weather::Message::Fetched(result))
				}),
				Command::perform(read_database(resource), Self::Message::FetchedTodos),
			])
		)
	}

	fn title(&self) -> String {
		String::from("bansheelong")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		Subscription::batch([
			iced::time::every(std::time::Duration::from_secs(300)).map(|_| Self::Message::Refresh), // refresh weather/todos
			iced::time::every(std::time::Duration::from_secs(1)).map(|_| { // tick weather widget so it can detect absense of user interaction, etc
				Self::Message::Tick
			}),
			todos::connect().map(|event| {
				match event {
					todos::Event::Error(_) => Self::Message::MenuMessage(menu::Message::TodosMessage(
						todos::Message::Update(None)
					)),
					todos::Event::InvalidateState => Self::Message::MenuMessage(menu::Message::TodosMessage(
						todos::Message::Update(None)
					)),
					todos::Event::Refresh => Self::Message::RefreshTodos,
				}
			}),
			storage::connect().map(|event| {
				match event {
					storage::tcp::Event::Error(_) => Self::Message::StorageMessage(storage::Message::Received(None)),
					storage::tcp::Event::Ignore => Self::Message::Noop,
					storage::tcp::Event::InvalidateState => Self::Message::StorageMessage(storage::Message::Received(None)),
					storage::tcp::Event::Message(data) => Self::Message::StorageMessage(storage::Message::Received(Some(data))),
				}
			}),
		])
	}

	fn update(&mut self, _message: Message) -> Command<Self::Message> {
		match _message {
			Self::Message::AddPlannedMeal(meal) => {
				let log = self.io.as_ref().add_planned_meal_log(meal.clone());

				let resource = self.io.resource.clone();
				Command::batch([
					self.menu.update(menu::Message::MealsMessage(
						meals::Message::APIAddPlannedMeal(meal)
					)).map(move |message| {
						self::Message::MenuMessage(message)
					}),
					Command::perform(async move {
						if let Err(error) = write_database(
							WriteDatabase::Partial {
								planned_meals_remove_log: &Vec::new(),
								planned_meals_write_log: &log,
								todos_write_log: &Vec::new(),
							},
							resource
						).await {
							eprintln!("{:?}", error);
						}
					}, move |()| {
						Self::Message::Refresh
					}),
				])
			},
			Self::Message::FetchedTodos(result) => {
				if let Err(error) = result {
					eprintln!("{:?}", error);
					
					Command::batch([
						self.menu.update(menu::Message::CalendarMessage(
							calendar::Message::Update(None)
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
						self.menu.update(menu::Message::TodosMessage(
							todos::Message::Update(None)
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
					])
				} else {
					let result = result.unwrap();
					self.io = Arc::new(IO { // TODO clean this up
						meals_database: result.1,
						resource: self.io.resource.clone(),
						todos_database: result.0,
						..IO::default()
					});

					for recipe in self.io.meals_database.recipes.iter() {
						if let None = recipe.image_url {
							continue;
						}

						let image_url = recipe.image_url.as_ref().unwrap();
						if !meals::is_valid_image_url(image_url) || meals::has_image(&recipe.name) {
							continue;
						}

						meals::download_image(image_url, &recipe.name);
					}

					Command::batch([
						self.menu.update(menu::Message::CalendarMessage(
							calendar::Message::Update(Some(self.io.clone()))
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
						self.menu.update(menu::Message::TodosMessage(
							todos::Message::Update(Some(self.io.clone()))
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
						self.menu.update(menu::Message::MealsMessage(
							meals::Message::Update(Some(self.io.clone()))
						)).map(move |message| {
							self::Message::MenuMessage(message)
						}),
					])
				}
			},
			Self::Message::FlavorMessage(message) => {
				self.flavor.update(message).map(move |message| {
					Self::Message::FlavorMessage(message)
				})
			},
			Self::Message::MenuMessage(message) => {
				self.menu.update(message).map(move |message| {
					Self::Message::MenuMessage(message)
				})
			},
			Self::Message::Noop => { Command::none() },
			Self::Message::Refresh => {
				Command::batch([
					Command::perform(read_database(self.io.resource.clone()), Self::Message::FetchedTodos),
					self.weather.update(weather::Message::Refresh).map(move |message| {
						Self::Message::WeatherMessage(message)
					}),
				])
			},
			Self::Message::RefreshTodos => {
				Command::perform(read_database(self.io.resource.clone()), Self::Message::FetchedTodos)
			},
			Self::Message::RemovePlannedMeal(date) => {
				let log = self.io.as_ref().remove_planned_meal_log(date.clone());

				let resource = self.io.resource.clone();
				Command::batch([
					self.menu.update(menu::Message::MealsMessage(
						meals::Message::APIRemovePlannedMeal(date)
					)).map(move |message| {
						self::Message::MenuMessage(message)
					}),
					Command::perform(async move {
						if let Err(error) = write_database(
							WriteDatabase::Partial {
								planned_meals_remove_log: &log,
								planned_meals_write_log: &Vec::new(),
								todos_write_log: &Vec::new(),
							},
							resource
						).await {
							eprintln!("{:?}", error);
						}
					}, move |()| {
						Self::Message::Refresh
					}),
				])
			},
			Self::Message::StorageMessage(message) => {
				self.storage.update(message).map(move |message| {
					Self::Message::StorageMessage(message)
				})
			},
			Self::Message::Tick => {
				let mut commands = vec![
					self.menu.update(menu::Message::Tick).map(move |message| {
						Self::Message::MenuMessage(message)
					}),
					self.storage.update(storage::Message::Tick).map(move |message| {
						Self::Message::StorageMessage(message)
					}),
					self.weather.update(weather::Message::Tick).map(move |message| {
						Self::Message::WeatherMessage(message)
					}),
				];

				if Instant::now() - self.last_update_to_log > Duration::from_secs(5) && self.update_log.len() > 0 {
					let resource = self.io.resource.clone();
					let log = self.update_log.clone();
					commands.push(
						Command::perform(async move {
							if let Err(error) = write_database(
								WriteDatabase::Partial {
									planned_meals_remove_log: &Vec::new(),
									planned_meals_write_log: &log,
									todos_write_log: &Vec::new(),
								},
								resource
							).await {
								eprintln!("{:?}", error);
							}
						}, move |()| {
							Self::Message::Refresh
						})
					);
					self.update_log.clear();

					Command::batch(commands)
				} else {
					Command::batch(commands)
				}
			},
			Self::Message::UpdatePlannedMeal(meal) => {
				self.update_log.append(&mut self.io.as_ref().add_planned_meal_log(meal.clone()));
				self.last_update_to_log = Instant::now();

				Command::batch([
					self.menu.update(menu::Message::MealsMessage(
						meals::Message::APIUpdatePlannedMeal(meal)
					)).map(move |message| {
						self::Message::MenuMessage(message)
					}),
				])
			},
			Self::Message::WeatherMessage(message) => {
				self.weather.update(message).map(move |message| {
					Self::Message::WeatherMessage(message)
				})
			},
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		Container::new(
			Row::new()
				.push( // weather
					self.weather.view().map(move |message| {
						Self::Message::WeatherMessage(message)
					})
				)
				.push( // vertical rule
					Container::new(
						Container::new(Text::new(""))
							.style(style::VerticalRule)
							.width(Length::Units(2))
							.height(Length::Units(state::WINDOW_STATE.height - 50))
					)
						.height(Length::Fill)
						.padding([0, 25])
						.align_y(alignment::Vertical::Center)
				)
				.push(
					self.menu.view().map(move |message| {
						if let menu::Message::MealsMessage(meals::Message::APIAddPlannedMeal(meal)) = &message {
							Self::Message::AddPlannedMeal(meal.clone())
						} else if let menu::Message::MealsMessage(meals::Message::APIRemovePlannedMeal(date)) = &message {
							Self::Message::RemovePlannedMeal(date.clone())
						} else if let menu::Message::MealsMessage(meals::Message::APIUpdatePlannedMeal(date)) = &message {
							Self::Message::UpdatePlannedMeal(date.clone())
						} else {
							Self::Message::MenuMessage(message)
						}
					})
				)
				.push( // storage thing & neat picture
					Column::new()
						.push(
							self.storage.view().map(move |message| {
								Self::Message::StorageMessage(message)
							})
						)
						.push(
							self.flavor.view().map(move |message| {
								Self::Message::FlavorMessage(message)
							})
						)
				)
		)
			.width(Length::Fill)
			.style(style::Container)
			.into()
	}
}

#[tokio::main]
async fn main() -> iced::Result {
	Window::run(Settings {
		antialiasing: false,
		default_font: Some(include_bytes!("../../shared_ui/data/fonts/NotoSans-Medium.ttf")),
		text_multithreading: true,
		window: iced::window::Settings {
			size: (state::WINDOW_STATE.width as u32, state::WINDOW_STATE.height as u32),
			resizable: false,
			decorations: false,
			..iced::window::Settings::default()
		},
		..Settings::default()
	})
}
