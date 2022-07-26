use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone)]
pub struct OneAPIError {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneAPIResponse {
	pub current: CurrentDatum,
	pub hourly: Vec<HourlyDatum>,
	pub daily: Vec<DailyDatum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkyDescription {
	pub id: u16,
	pub main: String,
	pub description: String,
	pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyTemperature {
	pub morn: f32,
	pub day: f32,
	pub eve: f32,
	pub night: f32,
	pub min: f32,
	pub max: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyFeelsLike {
	pub morn: f32,
	pub day: f32,
	pub eve: f32,
	pub night: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyDatum {
	pub feels_like: DailyFeelsLike,
	pub sunrise: u64,
	pub sunset: u64,
	pub temp: DailyTemperature,
	
	pub dt: i64,
	pub pressure: u16,
	pub humidity: u16,
	pub dew_point: f32,
	pub uvi: f32,
	pub clouds: u16,
	pub wind_speed: f32,
	pub wind_deg: u16,
	pub weather: Vec<SkyDescription>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyDatum {
	pub pop: f32,
	pub visibility: u16,
	pub temp: f32,
	pub feels_like: f32,
	
	pub dt: i64,
	pub pressure: u16,
	pub humidity: u16,
	pub dew_point: f32,
	pub uvi: f32,
	pub clouds: u16,
	pub wind_speed: f32,
	pub wind_deg: u16,
	pub weather: Vec<SkyDescription>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentDatum {
	pub sunrise: u64,
	pub sunset: u64,
	pub visibility: u16,
	pub temp: f32,
	pub feels_like: f32,
	
	pub dt: i64,
	pub pressure: u16,
	pub humidity: u16,
	pub dew_point: f32,
	pub uvi: f32,
	pub clouds: u16,
	pub wind_speed: f32,
	pub wind_deg: u16,
	pub weather: Vec<SkyDescription>,
}
