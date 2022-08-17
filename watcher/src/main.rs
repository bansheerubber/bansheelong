use std::process::{ Command, Stdio };

use notify::{ Op, RawEvent, RecursiveMode, Watcher, raw_watcher };
use std::sync::mpsc::channel;

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

	let mut io = IO {
		resource: Resource {
			reference: format!("http://{}:{}", get_todos_host(), get_todos_port()),
		},
		..IO::default()
	};

	if let Err(error)
		= io.parse_from_human_readable(String::from(todo_list), String::from(recipe_list))
	{
		eprintln!("{:?}", error);
	}

	draw_todo_list(&io, String::from("/home/me/Projects/bansheelong/todo-list.png"));
	draw_time_sheet(&io, String::from("/home/me/Projects/bansheelong/time-sheet.png"));
	combine(
		String::from("/home/me/.config/background2.png"),
		String::from("/home/me/Projects/bansheelong/todo-list.png"),
		String::from("/home/me/Projects/bansheelong/time-sheet.png"),
		String::from("/home/me/.config/real-background.png"),
	);
	reload_feh();

	loop {
		match rx.recv() {
			Ok(RawEvent{ path: Some(path), op: Ok(op), cookie: _ }) => {
				if (path.to_str() == Some(todo_list) || path.to_str() == Some(recipe_list)) && op == Op::CLOSE_WRITE {
					if let Err(error)
						= io.parse_from_human_readable(String::from(todo_list), String::from(recipe_list))
					{
						eprintln!("{:?}", error);
						continue;
					}

					draw_todo_list(&io, String::from("/home/me/Projects/bansheelong/todo-list.png"));
					draw_time_sheet(&io, String::from("/home/me/Projects/bansheelong/time-sheet.png"));
					combine(
						String::from("/home/me/.config/background2.png"),
						String::from("/home/me/Projects/bansheelong/todo-list.png"),
						String::from("/home/me/Projects/bansheelong/time-sheet.png"),
						String::from("/home/me/.config/real-background.png"),
					);
					reload_feh();

					if let Err(error) = write_database(
						WriteDatabase::Full {
							meals: &io.meals_database,
							todos: &io.todos_database,
						},
						io.resource.clone()
					).await {
						eprintln!("{:?}", error);
					}
				}
			},
			Ok(event) => println!("broken event: {:?}", event),
			Err(e) => println!("watch error: {:?}", e),
		};
	}
}
