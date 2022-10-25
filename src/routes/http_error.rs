use rocket::http::Status;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpError {
    pub code: String,
    pub message: String,
    pub url: String,
    pub x_trace_id: Uuid,
}

impl HttpError {
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

pub type StdErr = Box<dyn std::error::Error>;
