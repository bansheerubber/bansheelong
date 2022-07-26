use bansheelong_shared_ui::style;
use bansheelong_types::JobStatusFlags;
use iced::{ Column, Command, Container, Element, Length, Row, Space, Text };

use super::Data;

#[derive(Debug)]
pub struct View {
	data: Option<Data>,
	ellipses: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
	Received(Option<Data>),
	Tick,
}

impl View {
	pub fn new() -> Self {
		View {
			data: None,
			ellipses: 0,
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Received(data) => {
				self.data = data;
				Command::none()
			},
			Message::Tick => {
				self.ellipses = (self.ellipses + 1) % 5;
				Command::none()
			},
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let ellipses: String = std::iter::repeat(".").take(std::cmp::min((self.ellipses + 1) as usize, 4)).collect();

		let format_size = |size: u64| {
			let gigabytes = size / 1_000_000_000;
			if gigabytes < 1_500 {
				format!("{:.1}T", gigabytes as f64 / 1_000.0)
			} else {
				format!("{}T", gigabytes / 1_000)
			}
		};

		Container::new(
			Container::new(
				Column::new()
					.push(
						Row::new()
							.push(
								Text::new(
									format!(
										"{} backups",
										if let None = self.data {
											0
										} else {
											self.data.as_ref().unwrap().btrfs_backup_count
										}
									)
								)
							)
							.push(Space::new(Length::Units(18), Length::Units(0)))
							.push(
								Text::new(
									format!(
										"{}/{}",
										if let None = self.data { // used size
											"0T".to_string()
										} else {
											format_size(self.data.as_ref().unwrap().btrfs_used_size)
										},
										if let None = self.data { // total size
											"0T".to_string()
										} else {
											format_size(self.data.as_ref().unwrap().btrfs_total_size)
										}
									)
								)
							)
							.push(Space::new(Length::Units(18), Length::Units(0)))
							.push(
								Text::new(
									format!(
										"{}/{}",
										if let None = self.data { // used size
											"0T".to_string()
										} else {
											format_size(self.data.as_ref().unwrap().used_size)
										},
										if let None = self.data { // total size
											"0T".to_string()
										} else {
											format_size(self.data.as_ref().unwrap().total_size)
										}
									)
								)
							)
							.width(Length::Fill)
					)
					.push(
						Text::new(
							if self.data.is_none() {
								String::from("Not connected")
							} else {
								if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::GENERAL_ERROR) {
									String::from("Error")
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::CREATING_MONTHLY) {
									String::from("Creating monthly backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::CREATING_WEEKLY) {
									String::from("Creating weekly backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::DOWNLOADING_DAILY) {
									String::from("Downloading daily backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::SYNCING_GITHUB) {
									String::from("Syncing GitHub to backup") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::REMOVING_DAILY) {
									String::from("Removing stale daily") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::REMOVING_WEEKLY) {
									String::from("Removing stale weekly") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::ZPOOL_ERROR) { // start here
									String::from("ZPool error")
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::ZPOOL_HARD_DRIVE_PARSE_ERROR) {
									String::from("Hard drive parse error")
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::ZPOOL_HARD_DRIVE_RW_ERROR) {
									String::from("Hard drive r/w/c error")
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::ZPOOL_HARD_DRIVE_STATE_ERROR) {
									String::from("Hard drive error")
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::ZPOOL_SCRUBBING) {
									String::from("Scrubbing") + &ellipses
								} else if self.data.as_ref().unwrap().job_flags.contains(JobStatusFlags::WRITING_BTRBK) {
									String::from("Writing btrfs backup") + &ellipses
								} else {
									String::from("Idle")
								}
							}
						)
					)
					.width(Length::Units(240))
			)
				.padding(10)
				.style(style::TodoItem)
		)
			.padding([20, 0, 0, 5])
			.into()
	}
}
