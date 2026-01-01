use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct ResponseOK<T>(pub Option<T>)
where
    T: Serialize + Send;

impl<T> IntoResponse for ResponseOK<T> where T: Serialize + Send {}
