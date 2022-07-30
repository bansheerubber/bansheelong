use chrono::{ DateTime, Local, NaiveDateTime, Utc };

use crate::constants::{ get_api_key };
use crate::weather::types::{ OneAPIError, OneAPIResponse };

pub fn convert_to_time(time: i64) -> DateTime<Local> {
	DateTime::<Local>::from(DateTime::<Utc>::from_utc(
		NaiveDateTime::from_timestamp(time, 0),
		Utc
	))
}

pub fn decode_icon(id: u16, high_clouds: bool, day: bool) -> String {
	let status = match id {
		200..=202 => if high_clouds {
			"thunderstorms-rain"
		} else if day {
			"thunderstorms-day-rain"
		} else {
			"thunderstorms-night-rain"
		},
		210..=212 | 221 | 230..=232 => if high_clouds {
			"thunderstorms"
		} else if day {
			"thunderstorms-day"
		} else {
			"thunderstorms-night"
		},
		300..=302 | 313 | 314 | 321 => if high_clouds {
			"drizzle"
		} else if day {
			"partly-cloudy-day-drizzle"
		} else {
			"partly-cloudy-night-drizzle"
		},
		310..=312 | 500..=504 | 511 | 520..=522 | 530 => if high_clouds {
			"rain"
		} else if day {
			"partly-cloudy-day-rain"
		} else {
			"partly-cloudy-night-rain"
		},
		600..=602 | 615 | 616 | 620..=622 => if high_clouds {
			"snow"
		} else if day {
			"partly-cloudy-day-snow"
		} else {
			"partly-cloudy-night-snow"
		},
		611..=613 => if high_clouds {
			"sleet"
		} else if day {
			"partly-cloudy-day-sleet"
		} else {
			"partly-cloudy-night-sleet"
		},
		701 => "mist",
		711 => if high_clouds {
			"smoke"
		} else if day {
			"partly-cloudy-day-smoke"
		} else {
			"partly-cloudy-night-smoke"
		},
		721 => if high_clouds {
			"haze"
		} else if day {
			"haze-day"
		} else {
			"haze-night"
		},
		731 => if high_clouds {
			"dust"
		} else if day {
			"dust-day"
		} else {
			"dust-night"
		},
		741 => if high_clouds {
			"fog"
		} else if day {
			"fog-day"
		} else {
			"fog-night"
		},
		751 => if high_clouds {
			"dust"
		} else if day {
			"dust-day"
		} else {
			"dust-night"
		},
		762 => "volcano",
		771 => "wind",
		781 => "tornado",
		800 => "clear-day",
		801 | 802 => if day {
			"partly-cloudy-day"
		} else {
			"partly-cloudy-night"
		},
		803 => "cloudy",
		804 => "overcast",
		_other => "",
	}.to_string();

	return status;
}

pub async fn dial() -> Result<OneAPIResponse, OneAPIError> {
	let client = reqwest::Client::new();
	let response_result = client
		.get("https://api.openweathermap.org/data/3.0/onecall")
		.query(&[
			("lat", "33.4484"),
			("lon", "-112.074"),
			("exclude", "minutely,alerts"),
			("appid", get_api_key().as_str()),
			("units", "imperial"),
		])
		.header(reqwest::header::CONTENT_TYPE, "application/json")
    .header(reqwest::header::ACCEPT, "application/json")
		.send()
		.await;
	
	if let Err(error) = response_result {
		return Err(OneAPIError {
			message: error.to_string(),
		});
	}
	
	let response = response_result.unwrap();	
	match response.status() {
		reqwest::StatusCode::OK => {
			match response.json::<OneAPIResponse>().await {
				Ok(result) => return Ok(result),
				Err(error) => return Err(OneAPIError {
					message: format!("Could not deserialize JSON: {:?}", error),
				}),
			};
		},
		other => {
			return Err(OneAPIError {
				message: format!("Error code {}", other),
			});
		},
	}
}
