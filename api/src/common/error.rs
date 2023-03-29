use rocket::{response::{Responder, content}, http::Status, Response};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::models::OstType;

#[derive(Error, Debug)]
pub enum Error {
    // reqwest
    #[error("reqwest client error: {0}")]
    ReqwestClientError(#[source] reqwest::Error),

    #[error("reqwest request error: {0}")]
    ReqwestClientRequestError(String),

    #[error("cannot access proxer.me as {0}: {1}")]
    ReqwestProxerLoginError(String, String),

    // fs
    #[error("error creating directories: {0}")]
    DirectoryCreateError(#[source] std::io::Error),

    // sql
    #[error("failed underlying SQLite call: {0}")]
    SQLError(#[from] rusqlite::Error),

    #[error("enum parse error: {0}")]
    EnumParseError(String),

    // rocket
    #[error("rocket error: {0}")]
    RocketError(#[from] rocket::Error),

    #[error("cannot read user from headers")]
    AuthError(),
    
    // data
    #[error("ost does not exist: Anime {0} {1:?} {2}")]
    MissingOstError(u32, OstType, u32),

    #[error("anime with id {0} does not exist")]
    MissingAnimeError(u32),

    // config
    #[error("error parsing config: {0}")]
    FigmentError(#[source] figment::Error),

    // html parse
    #[error("error parsing main anime table")]
    MainTableParseError(),

    #[error("error parsing user name")]
    UserNameParseError(),

    #[error("no access permissions for proxer, maybe login first")]
    ProxerAccessError(),

    #[error("error parsing anime table for rows, no rows found, expecting at least one entry")]
    AnimeRowParseError(),

    #[error("error parsing anime row for {0}")]
    AnimeRowFieldParseError(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Self::MissingOstError(_, _, _) |
            Self::MissingAnimeError(_) => self.response_with_status(request, Status::NotFound),
            _ => self.response_with_status(request, Status::InternalServerError)
        }
    }
}

impl Error {
    fn response_with_status(&self, request: &rocket::Request, status: Status) -> rocket::response::Result<'static> {
        Response::build_from(wrap_err(self.to_string()).respond_to(request).unwrap())
            .status(status)
            .ok()
    }
}

fn wrap_err<S: Serialize>(serializable: S) -> content::RawJson<String> {
    content::RawJson(serde_json::to_string(&json!({
        "messages": [json!({
                "type": "error",
                "value": serializable,
            })],
        "data": None as Option<String>,
    })).unwrap())
}