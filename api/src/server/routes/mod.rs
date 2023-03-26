mod proxer_route;
mod anime_route;
mod useranime_route;
mod users_route;

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    let proxer_routes = proxer_route::get_routes();
    let anime_routes = anime_route::get_routes();
    let useranime_routes = useranime_route::get_routes();
    let users_routes = users_route::get_routes();
    [proxer_routes, anime_routes, useranime_routes, users_routes].concat()
}