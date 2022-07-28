pub struct Time {
	pub hour: u8,
	pub minute: u8,
}

pub struct Item {
	pub description: String,
	pub time: Time,
}

pub struct Day {
	pub items: Vec<Item>,
	pub day: u8,
	pub month: u8,
	pub year: u8,
	pub day_name: String,
}
