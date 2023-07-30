use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{InternalError, ErrorInternalServerError};
use actix_web::{web, FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use sqlx::PgPool;
use std::ops::Deref;

use crate::models::UserModel;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

#[derive(Copy, Clone, Debug)]
pub struct UserId(i64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = see_other("/user/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}

pub async fn authenticate(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        None => next.call(req).await,
        Some(user_id) => {
            let (http_request, _) = req.parts_mut();
            let Some(pg) = http_request.app_data::<web::Data<PgPool>>() else {
                let e = "PgPool not embeded in app_data";
                return Err(ErrorInternalServerError(e))
            };
            let um = UserModel{db: pg};
            if um.exists(user_id).await.map_err(e500)? {
                req.extensions_mut().insert(UserId(user_id));
            }
            next.call(req).await
        }
    }
}
