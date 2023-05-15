use crate::handlers::*;
use actix_web::web;

//this functions build the route config with the appropriate handlers
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/test").route("/rho", web::get().to(test)));
}
