use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    TicketDeletFailIdNotFound { id: u64 },
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        res.extensions_mut().insert(self);
        res
    }
}

impl Error {
    pub fn client_status_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGINFAIL),
            Self::AuthFailTokenWrongFormat
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NOAUTH),
            Self::TicketDeletFailIdNotFound { id: _ } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALIDPARAMS)
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICEERROR),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGINFAIL,
    NOAUTH,
    INVALIDPARAMS,
    SERVICEERROR,
}
