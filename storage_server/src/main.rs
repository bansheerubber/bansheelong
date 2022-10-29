use std::path::Path;
use std::process::{ Command, Stdio };
use std::sync::Arc;

use futures::future;

use tokio::net::tcp::{ OwnedReadHalf, OwnedWriteHalf };
use tokio::net::TcpListener;
use tokio::time::{ Duration, sleep };
use tokio::sync::Mutex;

use bansheelong_types::{ JobStatusFlags, STORAGE_MESSAGE_COUNT };

#[derive(Debug)]
enum Error {
	DiskUsage(String),
	LocalInfo(String),
	ZPool(String),
}

// run a command and return the stdout
fn run_command(command: &mut Command) -> Result<String, Error> {
	let child = command.stdout(Stdio::piped())
		.spawn();

	// make sure the command spawned
	let child = match child {
		Err(error) => {
			return Err(Error::ZPool(format!("child spawn error: {:?}", error)));
		},
		Ok(child) => child,
	};

	// make sure we got an output
	let child = match child.wait_with_output() {
		Err(error) => {
			return Err(Error::ZPool(format!("child wait error: {:?}", error)));
		},
		Ok(child) => child,
	};

	if !child.status.success() {
		return Err(Error::ZPool(format!("child returned non-zero exit code: {:?}", child.status.code())));
	}

	// make sure we decode the utf8 correctly
	match String::from_utf8(child.stdout) {
		Err(error) => {
			return Err(Error::ZPool(format!("utf8 decode error: {:?}", error)));
		},
		Ok(stdout) => Ok(stdout),
	}
}

#[derive(Debug)]
enum ZPoolStatus {
	Error,
	HardDriveReadWriteChecksumError,
	HardDriveParseError,
	HardDriveStateError,
	Scrubbing,
	Safe,
}

#[derive(Debug)]
enum HardDriveStatus {
	Degraded(i64, i64, i64),
	Faulted(i64, i64, i64),
	Offline(i64, i64, i64),
	Online(i64, i64, i64),
	Unavailable(i64, i64, i64),
}

fn get_hard_drive_status(line: &str) -> Option<HardDriveStatus> {
	let is_hard_drive_line = ["ONLINE", "FAULTED", "DEGRADED", "UNAVAIL", "OFFLINE"].iter()
		.fold(None, |is_hard_drive_line, item| {
			if line.to_string().contains(item)
				&& is_hard_drive_line == None
				&& !line.to_string().contains("state:")
			{
				Some(item)
			} else {
				is_hard_drive_line
			}
		});

	if is_hard_drive_line != None {
		let split_line = line.split(" ")
			.fold(Vec::new(), |mut accum, item| {
				let mut trimmed = item.replace("\t", "");
				trimmed = trimmed.trim().to_string();
				if trimmed.len() == 0 {
					accum
				} else {
					accum.push(trimmed);
					accum
				}
			});

		let read_errors: i64 = split_line[2].parse().unwrap_or(-1);
		let write_errors: i64 = split_line[3].parse().unwrap_or(-1);
		let checksum_errors: i64 = split_line[4].parse().unwrap_or(-1);

		match is_hard_drive_line {
			Some(&"ONLINE") => Some(HardDriveStatus::Online(read_errors, write_errors, checksum_errors)),
			Some(&"FAULTED") => Some(HardDriveStatus::Faulted(read_errors, write_errors, checksum_errors)),
			Some(&"DEGRADED") => Some(HardDriveStatus::Degraded(read_errors, write_errors, checksum_errors)),
			Some(&"UNAVAIL") => Some(HardDriveStatus::Unavailable(read_errors, write_errors, checksum_errors)),
			Some(&"OFFLINE") => Some(HardDriveStatus::Offline(read_errors, write_errors, checksum_errors)),
			_ => None,
		}
	} else {
		return None;
	}
}

fn get_zpool_error() -> Result<ZPoolStatus, Error> {
	let stdout = run_command(
		Command::new("zpool")
			.arg("status")
	)?;

	// analyze the output
	for line in stdout.split('\n') {
		// parse scrubbing information
		if line.contains("scan:") && line.contains("scrub in progress") {
			return Ok(ZPoolStatus::Scrubbing);
		}

		// parse hard drive information
		match get_hard_drive_status(line) {
			Some(HardDriveStatus::Online(read_errors, write_errors, checksum_errors)) => {
				// return error if we have error counts set
				if read_errors == -1 || write_errors == -1 || checksum_errors == -1 {
					return Ok(ZPoolStatus::HardDriveParseError);
				} else if read_errors != 0 || write_errors != 0 || checksum_errors != 0 {
					return Ok(ZPoolStatus::HardDriveReadWriteChecksumError);
				}
			},
			None => {},
			_ => {
				return Ok(ZPoolStatus::HardDriveStateError);
			},
		}

		if line.contains("errors:") && line != "errors: No known data errors" {
			return Ok(ZPoolStatus::Error)
		}
	}

	Ok(ZPoolStatus::Safe)
}

fn get_disk_usage() -> Result<(u64, u64), Error> {
	let stdout = run_command(
		Command::new("df")
			.arg("-B1")
	)?;

	// analyze the output
	let mut used_size = 0;
	let mut total_size = 0;
	for line in stdout.split('\n') {
		if line.contains("bansheerubber") {
			let items: Vec<String> = line.split(" ")
				.filter(|i| i.len() > 0)
				.map(|i| i.to_string())
				.collect();

			used_size = match items[2].parse() {
				Err(error) => {
					return Err(Error::DiskUsage(format!("used size parse error: {:?}", error)));
				},
				Ok(size) => size,
			};

			total_size = match items[1].parse() {
				Err(error) => {
					return Err(Error::DiskUsage(format!("total size parse error: {:?}", error)));
				},
				Ok(size) => size,
			};
		}
	}

	Ok((used_size, total_size))
}

fn get_backups_count() -> Result<(u8, u8), Error> {
	let read_count = |file_name: &str| {
		let value = match std::fs::read_to_string(format!("/home/me/bansheestorage/{}-count", file_name)) {
			Err(error) => {
				return Err(Error::LocalInfo(format!("{} count read error: {:?}", file_name, error)));
			},
			Ok(value) => value,
		};

		match value.trim().parse::<u8>() {
			Err(error) => {
				return Err(Error::LocalInfo(format!("{} parse error: {:?}", file_name, error)));
			},
			Ok(value) => Ok(value),
		}
	};

	Ok((read_count("dailies")?, read_count("weeklies")?))
}

fn get_job_flags() -> Result<JobStatusFlags, Error> {
	let mut result = JobStatusFlags::IDLE;

	match get_zpool_error() { // handle zpool error by indicating it on the bansheelong
		Err(_) => {
			result |= JobStatusFlags::ZPOOL_ERROR;
		},
		Ok(ZPoolStatus::Error) => {
			result |= JobStatusFlags::ZPOOL_ERROR;
		},
		Ok(ZPoolStatus::HardDriveParseError) => {
			result |= JobStatusFlags::ZPOOL_HARD_DRIVE_PARSE_ERROR;
		},
		Ok(ZPoolStatus::HardDriveReadWriteChecksumError) => {
			result |= JobStatusFlags::ZPOOL_HARD_DRIVE_RW_ERROR;
		},
		Ok(ZPoolStatus::HardDriveStateError) => {
			result |= JobStatusFlags::ZPOOL_HARD_DRIVE_STATE_ERROR;
		},
		Ok(ZPoolStatus::Scrubbing) => {
			result |= JobStatusFlags::ZPOOL_SCRUBBING;
		},
		Ok(ZPoolStatus::Safe) => {},
	};

	// check daily backup
	if Path::new("/home/me/bansheestorage/writing-daily-backup").exists() {
		result |= JobStatusFlags::DOWNLOADING_DAILY;
	}

	// check weekly backup
	if Path::new("/home/me/bansheestorage/writing-weekly-backup").exists() {
		result |= JobStatusFlags::CREATING_WEEKLY;
	}

	// check monthly backup
	if Path::new("/home/me/bansheestorage/writing-monthly-backup").exists() {
		result |= JobStatusFlags::CREATING_MONTHLY;
	}

	// check git backup
	if Path::new("/home/me/bansheestorage/writing-git-backup").exists() {
		result |= JobStatusFlags::SYNCING_GITHUB;
	}

	// check removing daily backup
	if Path::new("/home/me/bansheestorage/removing-daily-backup").exists() {
		result |= JobStatusFlags::REMOVING_DAILY;
	}

	// check removing weekly backup
	if Path::new("/home/me/bansheestorage/removing-weekly-backup").exists() {
		result |= JobStatusFlags::REMOVING_WEEKLY;
	}

	// check btrbk status
	if Path::new("/home/me/bansheestorage/writing-btrbk").exists() {
		result |= JobStatusFlags::WRITING_BTRBK;
	}

	Ok(result)
}

async fn read_socket(
	socket: Arc<OwnedReadHalf>
) -> Result<(), tokio::io::Error> {
	loop { // keep reading forever, until socket closes
		socket.readable().await?;

		let mut buffer = Vec::new();
		match socket.try_read_buf(&mut buffer) {
			Ok(0) => break,
			Ok(n) => {
				println!("read {} bytes", n);
			},
			Err(ref error) if error.kind() == tokio::io::ErrorKind::WouldBlock => {
				continue;
			},
			Err(error) => {
				return Err(error.into());
			},
		}
	}

	Ok(())
}

async fn write_socket(
	socket: Arc<OwnedWriteHalf>,
	message: String,
) -> Result<(), tokio::io::Error> {
	loop { // keep looping until we can write
		socket.writable().await?;

		match socket.try_write(message.as_bytes()) {
			Ok(_) => {
				break;
			},
			Err(ref error) if error.kind() == tokio::io::ErrorKind::WouldBlock => {
				continue;
			}
			Err(error) => {
				return Err(error)
			}
		}
	}

	Ok(())
}

#[tokio::main]
async fn main() {
	let sockets = Arc::new(Mutex::new(Vec::new()));
	let message = Arc::new(Mutex::new(String::new()));

	future::join(
		async { // server listener
			let sockets_reference = sockets.clone();

			let listener = TcpListener::bind("0.0.0.0:3002").await;
			if let Err(error) = listener.as_ref() {
				eprintln!("could not open socket {:?}", error);
				std::process::exit(1);
			}

			let listener = listener.unwrap();
			loop { // accept sockets
				let socket = listener.accept().await;
				if let Err(error) = socket.as_ref() {
					eprintln!("could not accept socket {:?}", error);
					continue;
				}

				let split = socket.unwrap().0.into_split();

				// move to longer lifetime
				let mut locked = sockets_reference.lock().await;
				locked.push((Arc::new(split.0), Arc::new(split.1)));

				// send status message right away
				let index = locked.len() - 1;
				let locked_message = message.lock().await;
				if let Err(error) = write_socket(locked[index].1.clone(), locked_message.clone()).await {
					eprintln!("socket write error {:?}", error);
				}

				// spawn read task
				let read_half = locked[index].0.clone();
				let sockets_reference = sockets_reference.clone();
				tokio::spawn(async move {
					if let Err(error) = read_socket(read_half).await {
						eprintln!("socket read error {:?}", error);
					}

					let mut locked = sockets_reference.lock().await;
					locked.remove(index);
				});
			}
		},
		async { // status getter
			let sockets_reference = sockets.clone();

			let mut sleep_time = 0;
			loop {
				sleep(Duration::from_secs(sleep_time)).await;
				sleep_time = 5;

				// get server job status
				let job_status = match get_job_flags() {
					Err(error) => {
						eprintln!("job status error: {:?}", error);
						0
					},
					Ok(value) => value.bits(),
				};

				// get df
				let (used_size, total_size) = match get_disk_usage() {
					Err(error) => {
						eprintln!("disk usage error: {:?}", error);
						(0, 0)
					},
					Ok(value) => value
				};

				// get dailies/weeklies count
				let (dailies, weeklies) = match get_backups_count() {
					Err(error) => {
						eprintln!("dailies/weeklies error: {:?}", error);
						(0, 0)
					},
					Ok(value) => value
				};

				// update message
				let mut locked_message = message.lock().await;
				*locked_message = format!("{} {} {} {} {}\n", job_status, used_size, total_size, dailies, weeklies);

				let whitespace_count = locked_message.chars()
					.fold(0, |prev, c| {
						prev + if c.is_whitespace() {
							1
						} else {
							0
						}
					});

				if whitespace_count != STORAGE_MESSAGE_COUNT {
					eprintln!("error: unexpected sent message word length {}", whitespace_count);
				}

				// send to all sockets
				let locked = sockets_reference.lock().await;
				for (_, writable) in locked.iter() {
					if let Err(error) = write_socket(writable.clone(), locked_message.clone()).await {
						eprintln!("socket write error {:?}", error);
					}
				}
			}
		}
	).await;
}
