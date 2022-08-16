use notify::{ Op, RawEvent, RecursiveMode, Watcher, raw_watcher };
use std::sync::mpsc::channel;

use bansheelong_types::{ IO, Resource, get_todos_host, get_todos_port, write_database };

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

					if let Err(error) = write_database(
						(&io.todos_database, &io.meals_database),
						None,
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
