use std::collections::HashMap;
use actix_web::{
    http::header::ContentType,
    http::header::LOCATION, 
    web, Error, HttpResponse, Result
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use sqlx::PgPool;


use crate::session_state::TypedSession;
use crate::authentication;
use crate::domain::{LoginForm, SignupForm, PasswordUpdateForm, UserPasswordUpdate, UserLoginForm, Email};
use crate::models::{RegUserError, UserModel};
use crate::tmpl::{BaseTemplate, LoginTemplate, SignupTemplate, AcctTemplate, PasswordUpdateTemplate};
use crate::utils::{e500, see_other};
use crate::authentication::UserId;


pub async fn show_signup(
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let base = BaseTemplate::new("", "", is_auth.is_some());
    let sign = SignupTemplate {
        _parent: &base,
        error_map: HashMap::new(),
        form: SignupForm::default(),
    };
    let body = sign.render().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

pub async fn signup(
    form: web::Form<SignupForm>,
    pool: web::Data<PgPool>,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let orig_form = form.0.clone();
    let res = form.0.try_into();
    if let Err(err_map) = res {
        let base = BaseTemplate::new("", "", is_auth.is_some());
        let sign = SignupTemplate {
            _parent: &base,
            error_map: err_map,
            form: orig_form,
        };
        let body = sign.render().map_err(e500)?;
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type(ContentType::html())
            .body(body));
    }
    let um = UserModel { db: &pool };
    if let Err(e) = authentication::signup(res.unwrap(), um).await {
        let cause = e.chain().take(1).next().unwrap();
        let Some(RegUserError::DuplicateEmail) = cause.downcast_ref::<RegUserError>()  else {
             return Err(e500(e)); 
        };
        let base = BaseTemplate::new("", "", is_auth.is_some());
                let sign = SignupTemplate {
                    _parent: &base,
                    error_map: HashMap::from([("email", "Email address is already in use")]),
                    form: orig_form,
                };
        let body = sign.render().map_err(e500)?;
        return Ok(HttpResponse::UnprocessableEntity()
                    .content_type(ContentType::html())
                    .body(body));
    };
    FlashMessage::info("Your signup was successful. Please log in.").send();
    Ok(see_other("/user/login"))
}

pub async fn show_login(
    msg: IncomingFlashMessages,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let msg = msg.iter().take(1).next().map_or("", |m| m.content());
    let base = BaseTemplate::new(msg, "", is_auth.is_some());
    let login = LoginTemplate {
        _parent: &base,
        error_map: HashMap::new(),
        form: LoginForm::default(),
    };
    let body = login.render().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}


#[tracing::instrument(
    skip(form, pool, session, is_auth),
    fields(email=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Form<LoginForm>, 
    session: TypedSession,
    pool: web::Data<PgPool>,
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let orig_form = form.0.clone();
    let res = form.0.try_into();
    if let Err(err_map) = res {
        let base = BaseTemplate::new("", "", is_auth.is_some());
        let login = LoginTemplate {
            _parent: &base,
            error_map: err_map,
            form: orig_form,
        };
        let body = login.render().map_err(e500)?;
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type(ContentType::html())
            .body(body));
    }

    tracing::Span::current().record("email", &tracing::field::display(&orig_form.email));
    let um = UserModel { db: &pool };
    let res =  authentication::validate_credentials(res.unwrap(), um).await;
    if let Err(e) = res {
         return match e {
             authentication::AuthError::UnexpectedError(_) => Err(e500(e)),
             authentication::AuthError::InvalidCredentials(_) => {
                let base = BaseTemplate::new("", "", is_auth.is_some());
                let login = LoginTemplate {
                   _parent: &base,
                   error_map: HashMap::from([("login_err", "Email or password is incorrect")]),
                   form: orig_form,
                };
                let body = login.render().map_err(e500)?;
                Ok(HttpResponse::UnprocessableEntity()
                .content_type(ContentType::html())
                .body(body))
             }
         };
    };
    let user_id = res.unwrap();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    session.renew();
    session
    .insert_user_id(user_id)
    .map_err(e500)?;

    if let Some(Ok(path)) = session.remove_path() {
       return  Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, path))
                .finish()
               )
    }
            
    Ok(
        HttpResponse::SeeOther()
        .insert_header((LOCATION, "/snippet/create"))
        .finish()
    )
}

pub async fn logout(session: TypedSession) -> Result<HttpResponse, Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(see_other("/user/login"))
    } else {
        session.log_out();
        FlashMessage::info("You've been logged out successfully!").send();
        Ok(see_other("/"))
    }
}

pub async fn user_detail(
    msg: IncomingFlashMessages,
    session: TypedSession,
    pool: web::Data<PgPool>,
    is_auth: Option<web::ReqData<UserId>>,
) ->  Result<HttpResponse, Error> {

    let msg = msg.iter().take(1).next().map_or("", |m| m.content());
    let uid = session.get_user_id().map_err(e500)?;

    if uid.is_none() {
        Ok(see_other("/user/login"))
    } else {
        let um = UserModel{db: &pool};
        let user = um.get(uid.unwrap()).await.map_err(e500)?;
        let base = BaseTemplate::new(msg, "", is_auth.is_some());
        let acct  = AcctTemplate{
                _parent: &base, 
                user,
        };
        let body = acct.render().map_err(e500)?;
        Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    }
}

pub async fn show_password_update(
    is_auth: Option<web::ReqData<UserId>>,
) -> Result<HttpResponse, Error> {
    let base = BaseTemplate::new("", "", is_auth.is_some());
    let pass_tmpl = PasswordUpdateTemplate {
        _parent: &base,
        error_map: HashMap::new(),
    };
    let body = pass_tmpl.render().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}


pub async fn password_update(
    form: web::Form<PasswordUpdateForm>,
    user_id : web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let upd: Result<UserPasswordUpdate, _> = form.0.try_into();
    if let Err(err_map) = upd {
        let base = BaseTemplate::new("", "", true);
        let pass = PasswordUpdateTemplate {
            _parent: &base,
            error_map: err_map,
        };
        let body = pass.render().map_err(e500)?;
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type(ContentType::html())
            .body(body));
    }
    let um = UserModel { db: &pool };
    let email = um.get_email(**user_id).await.map_err(e500)?;
    
    let UserPasswordUpdate{ current_password: current,  new_password: new, ..} = upd.unwrap();
        
        
    let cred = UserLoginForm {
        email: Email::parse(email).unwrap(), 
        password: current,
    };
    
    let res =  authentication::validate_credentials(cred, um).await;
    if let Err(e) = res {
         return match e {
             authentication::AuthError::UnexpectedError(_) => Err(e500(e)),
             authentication::AuthError::InvalidCredentials(_) => {
                let base = BaseTemplate::new("", "", true);
                let pass = PasswordUpdateTemplate {
                   _parent: &base,
                   error_map: HashMap::from([("current_password", "current password is incorrect")]),
                };
                let body = pass.render().map_err(e500)?;
                Ok(HttpResponse::UnprocessableEntity()
                .content_type(ContentType::html())
                .body(body))
             }
         };
    };
    crate::authentication::change_password(**user_id, new.0, um).await.map_err(e500)?;
    FlashMessage::info("Your password has been changed.").send();
    Ok(see_other("/user/view"))
}

