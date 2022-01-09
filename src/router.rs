use actix_web::{web};
use crate::handlers::post::index;

pub fn build(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/post")
            .route(web::get().to(index))
    );
}
