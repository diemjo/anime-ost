use rocket::{request::{FromRequest, Outcome}, Request, http::Status};

use crate::error::Error;

#[derive(Debug)]
pub(crate) struct RequestUser {
    user: String,
    groups: Vec<String>,
    name: String,
    email: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestUser {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        let auth_user = request.headers().get("HTTP_REMOTE_USER").next();
        let auth_groups = request.headers().get("HTTP_REMOTE_GROUPS").next();
        let auth_name = request.headers().get("HTTP_REMOTE_NAME").next();
        let auth_email = request.headers().get("HTTP_REMOTE_EMAIL").next();
        match (auth_user, auth_groups, auth_name, auth_email) {
            (Some(user), Some(groups), Some(name), email) => Outcome::Success(RequestUser {
                user: user.to_string(),
                groups: groups.split(",").map(str::to_string).collect::<Vec<String>>(),
                name: name.to_string(),
                email: email.map(str::to_string)
            }),
            _ => Outcome::Failure((Status::BadRequest, Error::AuthError()))
        }
    }
}