use rocket::{
    async_trait,
    http::Status,
    request::Request,
    response::{Responder, Result},
    serde::json::Json,
};
use uuid::Uuid;

#[derive(serde::Serialize)]
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
            return Self::internal(url);
        }

        Self {
            code: status.reason_lossy().to_string(),
            message: message.to_string(),
            url: url.to_string(),
            x_trace_id: Uuid::new_v4(),
        }
    }

    pub fn internal(url: &str) -> Self {
        let status = Status::InternalServerError;
        let message = "Internal Server Error";
        Self {
            code: status.reason_lossy().to_string(),
            message: message.to_string(),
            url: url.to_string(),
            x_trace_id: Uuid::new_v4(),
        }
    }

    pub fn not_found(url: &str) -> Self {
        let status = Status::NotFound;
        Self::from_message(&status, "route not found", url)
    }
}

#[async_trait]
impl<'r> Responder<'r, 'static> for &'r HttpError {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        Json(self).respond_to(req)
    }
}

#[async_trait]
impl<'r> Responder<'r, 'static> for HttpError {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        (&self).respond_to(req)
    }
}
