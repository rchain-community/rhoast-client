use crate::rholang::test_rholang;
use actix_web::{Error, HttpResponse};

pub async fn test() -> Result<HttpResponse, Error> {
    let rholang_code = test_rholang();
    Ok(HttpResponse::Ok().body(rholang_code))
}
