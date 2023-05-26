use actix_web::{web, Error, HttpResponse};

use crate::{rholang::parse_simple_deploy, types::SimpleDeployPayload};

pub async fn simple_deploy_handler(
    payload: web::Json<SimpleDeployPayload>,
) -> Result<HttpResponse, Error> {
    match parse_simple_deploy(&payload.text) {
        Ok(text) => Ok(HttpResponse::Ok().body(text)),
        Err(e) => {
            let error = format!("Error: {:?}", e);
            Ok(HttpResponse::BadRequest().body(error))
        }
    }
}
