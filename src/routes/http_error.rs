use crate::db::DbErr;
use error_chain::ChainedError;
use log::error;
use rocket::{
    async_trait,
    http::Status,
    request::Request,
    response::{Responder, Result},
    serde::{json::Json, Serialize, Serializer},
};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HttpError {
    pub code: String,
    #[serde(serialize_with = "serialize_status")]
    pub status_code: Status,
    pub message: String,
    pub x_trace_id: Uuid,
}

fn serialize_status<S: Serializer>(
    status: &Status,
    ser: S,
) -> std::result::Result<S::Ok, S::Error> {
    ser.serialize_u16(status.code)
}

impl HttpError {
    pub fn from_message(status_code: Status, message: String) -> Self {
        Self {
            code: status_code.reason_lossy().to_string(),
            status_code,
            message,
            x_trace_id: Uuid::new_v4(),
        }
    }

    pub fn internal(msg: Option<String>) -> Self {
        Self::from_message(
            Status::InternalServerError,
            msg.unwrap_or("Something went wrong".to_string()),
        )
    }

    pub fn not_found() -> Self {
        Self::from_message(Status::NotFound, "Route not found".to_string())
    }

    pub fn is_unhandled(&self) -> bool {
        self.status_code.class().is_server_error() || self.status_code.class().is_unknown()
    }
}

impl From<DbErr> for HttpError {
    fn from(err: DbErr) -> Self {
        let err_str = err
            .chain_err(|| "Something went wrong")
            .display_chain()
            .to_string();
        // TODO: better handling for DbErr
        HttpError::internal(Some(err_str))
    }
}

#[async_trait]
impl<'r> Responder<'r, 'static> for &'r HttpError {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let internal_error = HttpError::internal(None);

        let sanitized_err = if self.is_unhandled() {
            let url = req.uri().to_string();
            error!("Unhandled error: {self:?}, url: {url}");
            &internal_error
        } else {
            self
        };

        Json(sanitized_err) //
            .respond_to(req)
            .map(|mut res| {
                res.set_status(sanitized_err.status_code);
                res
            })
    }
}

#[async_trait]
impl<'r> Responder<'r, 'static> for HttpError {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        (&self).respond_to(req)
    }
}
