use rocket::{http::Status, request::Outcome, Request};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpErrorResponse {
    pub code: String,
    pub message: String,
    pub url: String,
    pub x_trace_id: Uuid,
}

impl HttpErrorResponse {
    pub fn from_message(status: &Status, message: &str, url: &str) -> Self {
        let is_unhandled_error = status.class().is_server_error() || status.class().is_unknown();

        if is_unhandled_error {
            return Self::server_error(url);
        }

        Self {
            code: status.reason_lossy().into(),
            message: message.into(),
            url: url.to_string(),
            x_trace_id: Uuid::new_v4(),
        }
    }

    pub fn server_error(url: &str) -> Self {
        let status = Status::InternalServerError;
        let message = "Internal Server Error";
        Self {
            code: status.reason_lossy().into(),
            message: message.into(),
            url: url.to_string(),
            x_trace_id: Uuid::new_v4(),
        }
    }

    pub fn not_found(url: &str) -> Self {
        let status = Status::NotFound;
        Self::from_message(&status, "route not found", url)
    }
}

pub fn reply_with_err<'r, T>(
    req: &'r Request<'_>,
    status: Status,
    error_message: &'r str,
) -> Outcome<T, &'r str> {
    req.local_cache(|| {
        HttpErrorResponse::from_message(&status, error_message, &req.uri().to_string())
    });
    return Outcome::Failure((status, error_message.into()));
}

pub type StdErr = Box<dyn std::error::Error>;
