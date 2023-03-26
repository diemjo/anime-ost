use rocket::{response::Responder, http::Status};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

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
    #[error("Failed underlying SQLite call: {0}")]
    SQLError(#[from] rusqlite::Error),

    // rocket
    #[error("rocket error: {0}")]
    RocketError(#[from] rocket::Error),

    #[error("cannot read user from headers")]
    AuthError(),

    // config
    #[error("Error parsing config: {0}")]
    FigmentError(#[source] figment::Error),

    // parse
    #[error("Error parsing main anime table")]
    MainTableParseError(),

    #[error("Error parsing user name")]
    UserNameParseError(),

    #[error("No access permissions for proxer, maybe login first")]
    ProxerAccessError(),

    #[error("Error parsing anime table for rows, no rows found, expecting at least one entry")]
    AnimeRowParseError(),

    #[error("Error parsing anime row for {0}")]
    AnimeRowFieldParseError(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            _ => Status::InternalServerError.respond_to(request)
        }
    }
}