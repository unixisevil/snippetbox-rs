use std::collections::HashMap;
use actix_web::{http::header::ContentType, web, Error, HttpResponse, Result};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use sqlx::PgPool;

use crate::domain::SnippetCreateForm;
use crate::models::SnippetModel;
use crate::tmpl::{BaseTemplate, CreateTemplate, HomeTemplate, ViewTemplate};
use crate::utils::{e500, see_other};
use crate::authentication::UserId;

pub async fn home(
    pool: web::Data<PgPool>,
    msg: IncomingFlashMessages,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let sm = SnippetModel { db: &pool };

    let snips = sm.latest().await.map_err(e500)?;

    let msg = msg.iter().take(1).next().map_or("", |m| m.content());

    let base = BaseTemplate::new(msg, "", is_auth.is_some());
    let home = HomeTemplate {
        _parent: &base,
        snippets: snips,
    };

    let body = home.render().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    //Ok(HttpResponse::Ok().json(snips))
}

pub async fn snippet_view(
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
    msg: IncomingFlashMessages,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let snip_id = path.into_inner();
    let sm = SnippetModel { db: &pool };
    let msg = msg.iter().take(1).next().map_or("", |m| m.content());
    let snip = sm.get(snip_id).await.map_err(e500)?;

    let base = BaseTemplate::new(msg, "", is_auth.is_some());
    let view = ViewTemplate {
        _parent: &base,
        snippet: snip,
    };

    let body = view.render().map_err(e500)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

pub async fn snippet_create(
    form: web::Form<SnippetCreateForm>,
    pool: web::Data<PgPool>,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let err_map = form.0.validate();
    if !err_map.is_empty() {
        let base = BaseTemplate::new("", "", is_auth.is_some());
        let view = CreateTemplate {
            _parent: &base,
            form: form.0,
            error_map: err_map,
        };
        let body = view.render().map_err(e500)?;
        return Ok(
            HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(body)
        );
    }
    let sm = SnippetModel { db: &pool };
    let  SnippetCreateForm{title, content, expires} = form.0;
    let snip_id = sm.insert(title, content, expires).await.map_err(e500)?;
    FlashMessage::info("Snippet successfully created!").send();
    Ok(see_other(&format!("/snippet/view/{}", snip_id)))
}

pub async fn show_snippet_create(
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let base = BaseTemplate::new("", "",  is_auth.is_some());
    let view = CreateTemplate {
        _parent: &base,
        form: SnippetCreateForm::default(),
        error_map: HashMap::new(),
    };

    let body = view.render().map_err(e500)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
