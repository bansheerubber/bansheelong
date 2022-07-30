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

#[derive(Clone, Debug, Default)]
pub struct TemperatureDatum {
	pub time: u16,
	pub temperature: u16,
}

impl TemperatureDatum {
	pub fn get_temperature(&self) -> String {
		format!("{}°", self.temperature)
	}

	pub fn get_time(&self) -> String {
		if self.time == 12 {
			format!("{} PM", self.time)
		} else if self.time == 0 {
			String::from("12 AM")
		} else if self.time > 12 {
			format!("{} PM", self.time - 12)
		} else {
			format!("{} AM", self.time)
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct DailyStatus {
	pub current: TemperatureDatum,
	pub day: String,
	pub humidity: u16,
	pub icon: String,
	pub times: [TemperatureDatum; 3],
	pub uv: u16,
	pub wind: u16,
}
