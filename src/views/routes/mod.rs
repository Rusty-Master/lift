use actix_web::web::{get, post, scope, ServiceConfig};

mod create;
pub fn routes_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/v1/route")
            .route("/create", post().to(create::create)),
    );
}
