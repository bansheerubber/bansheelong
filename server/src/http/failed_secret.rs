use crate::http::Response;

pub(crate) fn failed_secret() -> warp::reply::WithStatus<warp::reply::Json> {
	warp::reply::with_status(
		warp::reply::json(&Response {
			error: None,
			success: false,
		}),
		warp::http::StatusCode::FORBIDDEN
	)
}
