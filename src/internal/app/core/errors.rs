use crate::internal::app::core::ResponseBody;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpResponseError {
    #[error("invalid paramters: {0}")]
    InvalidParamtersError(String),

    #[error("unauthorized")]
    UnauthorizedError,

    #[error("error not found: {0}")]
    NotFoundError(String),

    #[error("server internal error")]
    ServerInternalError(#[from] anyhow::Error),
}

impl HttpResponseError {
    pub fn code(&self) -> i32 {
        match self {
            HttpResponseError::InvalidParamtersError(_) => 100400,
            HttpResponseError::UnauthorizedError => 100401,
            HttpResponseError::NotFoundError(_) => 100404,
            HttpResponseError::ServerInternalError(_) => 100500,
        }
    }
}

impl IntoResponse for HttpResponseError {
    fn into_response(self) -> Response {
        if let HttpResponseError::ServerInternalError(e) = &self {
            tracing::error!(error = ?e)
        }

        let body = ResponseBody::<()> {
            code: self.code(),
            msg: self.to_string(),
            data: None,
        };
        Json(body).into_response()
    }
}

pub struct IsResponseOK<T>(pub Option<T>)
where
    T: Serialize + Send;

impl<T> IntoResponse for IsResponseOK<T>
where
    T: Serialize + Send,
{
    fn into_response(self) -> Response {
        let body = ResponseBody {
            code: 0,
            msg: "success".to_string(),
            data: self.0,
        };
        Json(body).into_response()
    }
}
