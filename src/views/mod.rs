mod routes;
use actix_web::web::ServiceConfig;

use self::routes::routes_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    routes_views_factory(app)
}
