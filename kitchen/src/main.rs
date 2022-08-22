mod menu;
mod state;

use std::sync::Arc;
use std::time::{ Duration, Instant };

use bansheelong_types::{ Date, Error, IO, MealsDatabase, PlannedMeal, PlannedMealsWriteLog, Resource, TodosDatabase, WriteDatabase, get_todos_host, get_todos_port, read_database, write_database };
use bansheelong_shared_ui::{ meals, style, ws };
use iced::executor;
use iced::{ Application, Command, Container, Element, Length, Row, Settings, Subscription };

struct Window {
	menu: menu::View,

	last_update_to_log: Instant,
	io: Arc<IO>,
	update_log: PlannedMealsWriteLog,
}

#[derive(Debug)]
enum Message {
	AddPlannedMeal(PlannedMeal),
	FetchedTodos(Result<(TodosDatabase, MealsDatabase), Error>),
	MenuMessage(menu::Message),
	Refresh,
	RemovePlannedMeal(Date),
	Tick,
	UpdatePlannedMeal(PlannedMeal),
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
				menu: menu::View::new(),

				last_update_to_log: Instant::now(),
				io: Arc::new(IO {
					resource: resource.clone(),
					..IO::default()
				}),
				update_log: Vec::new(),
			},
			Command::perform(read_database(resource), Self::Message::FetchedTodos),
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
			ws::connect().map(|event| {
				match event {
					ws::Event::Error(_) => Self::Message::MenuMessage(menu::Message::MealsMessage(
						meals::Message::Update(None)
					)),
					ws::Event::InvalidateState => Self::Message::MenuMessage(menu::Message::MealsMessage(
						meals::Message::Update(None)
					)),
					ws::Event::Refresh => Self::Message::Refresh,
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

					self.menu.update(menu::Message::MealsMessage(
						meals::Message::Update(None)
					)).map(move |message| {
						self::Message::MenuMessage(message)
					})
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

					self.menu.update(menu::Message::MealsMessage(
						meals::Message::Update(Some(self.io.clone()))
					)).map(move |message| {
						self::Message::MenuMessage(message)
					})
				}
			},
			Self::Message::MenuMessage(message) => {
				self.menu.update(message).map(move |message| {
					Self::Message::MenuMessage(message)
				})
			},
			Self::Message::Refresh => {
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
			Self::Message::Tick => {
				let mut commands = vec![
					self.menu.update(menu::Message::Tick).map(move |message| {
						Self::Message::MenuMessage(message)
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
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		Container::new(
			Row::new()
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
		)
			.width(Length::Fill)
			.padding([0, 5, 0, 20])
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
