use std::process::{ Command, Stdio };
use std::sync::{ Arc, mpsc::TryRecvError, mpsc::channel };

use futures::future;
use notify::{ Op, RawEvent, RecursiveMode, Watcher, raw_watcher };
use tokio::sync::Mutex;

use bansheelong_local::{ combine, draw_time_sheet, draw_todo_list };
use bansheelong_types::{ IO, Resource, WriteDatabase, get_todos_host, get_todos_port, write_database };

fn reload_feh() {
	let child = Command::new("feh")
		.env("DISPLAY", ":0.0")
		.arg("--bg-fill")
		.arg("/home/me/.config/real-background.png")
		.stdout(Stdio::piped())
		.spawn();
	
	if let Err(_) = child {
		eprintln!("Could not run command");
		return;
	}

	if let Err(_) = child.unwrap().wait_with_output() {
		eprintln!("Could not wait for command to finish");
		return;
	}
}

#[tokio::main]
async fn main() {
	let todo_list = "/home/me/Projects/bansheetodo/todo-list";
	let recipe_list = "/home/me/Projects/bansheetodo/recipe-list";

	let (tx, rx) = channel();
	let mut watcher = raw_watcher(tx).unwrap();
	watcher.watch("/home/me/Projects/bansheetodo", RecursiveMode::Recursive).unwrap();

	let io = Arc::new(Mutex::new(IO {
		resource: Resource {
			reference: format!("http://{}:{}", get_todos_host(), get_todos_port()),
		},
		..IO::default()
	}));

	future::join(
		async {
			let io = io.clone();
			loop {
				tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;

				let locked = io.lock().await;
				draw_time_sheet(&locked, String::from("/home/me/Projects/bansheelong/time-sheet.png"));
				combine(
					String::from("/home/me/.config/background2.png"),
					String::from("/home/me/Projects/bansheelong/todo-list.png"),
					String::from("/home/me/Projects/bansheelong/time-sheet.png"),
					String::from("/home/me/.config/real-background.png"),
				);
				reload_feh();
			}
		},
		async {
			let io = io.clone();
			loop {
				match rx.try_recv() {
					Ok(RawEvent{ path: Some(path), op: Ok(op), cookie: _ }) => {
						if (path.to_str() == Some(todo_list) || path.to_str() == Some(recipe_list)) && op == Op::CLOSE_WRITE {
							let mut locked = io.lock().await;

							if let Err(error)
								= locked.parse_from_human_readable(String::from(todo_list), String::from(recipe_list))
							{
								eprintln!("{:?}", error);
								continue;
							}

							draw_todo_list(&locked, String::from("/home/me/Projects/bansheelong/todo-list.png"));
							draw_time_sheet(&locked, String::from("/home/me/Projects/bansheelong/time-sheet.png"));
							combine(
								String::from("/home/me/.config/background2.png"),
								String::from("/home/me/Projects/bansheelong/todo-list.png"),
								String::from("/home/me/Projects/bansheelong/time-sheet.png"),
								String::from("/home/me/.config/real-background.png"),
							);
							reload_feh();

							if let Err(error) = write_database(
								WriteDatabase::Full {
									meals: &locked.meals_database,
									todos: &locked.todos_database,
								},
								locked.resource.clone()
							).await {
								eprintln!("{:?}", error);
							}
						}
					},
					Ok(event) => println!("broken event: {:?}", event),
					Err(TryRecvError::Empty) => {},
					Err(e) => println!("watch error: {:?}", e),
				};

				tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
			}
		}
	).await;
}
