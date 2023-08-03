pub mod snippet;
pub mod user;

use crate::tmpl::{AboutTemplate, BaseTemplate};
use crate::utils::e500;
use crate::authentication::UserId;
use actix_web::{http::header::ContentType, web, Error, HttpResponse, Responder, Result};
use askama::Template;

pub async fn ping() -> impl Responder {
    "OK"
}

pub async fn about(is_auth: Option<web::ReqData<UserId>>) -> Result<HttpResponse, Error> {
    let base = BaseTemplate::new("", "", is_auth.is_some());
    let about = AboutTemplate { _parent: &base };
    let body = about.render().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
