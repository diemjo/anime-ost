mod proxer_route;
mod anime_route;
mod useranime_route;
mod users_route;

pub(crate) use proxer_route::get_proxer;
pub(crate) use anime_route::get_anime;
pub(crate) use useranime_route::get_useranime;
pub(crate) use users_route::get_users;