use crate::authentication::{authenticate, reject_anonymous_users};
use crate::handlers::snippet::{home, show_snippet_create, snippet_create, snippet_view};
use crate::handlers::user::{login, logout, show_login, show_signup, signup, user_detail, show_password_update, password_update};
use crate::handlers::{about, ping};
use actix_files as fs;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub fn user_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .service(
                web::resource("signup")
                    .wrap(from_fn(authenticate))
                    .route(web::get().to(show_signup))
                    .route(web::post().to(signup)),
            )
            .service(
                web::resource("login")
                    .wrap(from_fn(authenticate))
                    .route(web::post().to(login))
                    .route(web::get().to(show_login)),
            )
            .service(
                web::resource("logout")
                    .route(web::post().to(logout))
                    .wrap(from_fn(reject_anonymous_users)),
            )
            .service(
                web::resource("view")
                    .route(web::get().to(user_detail))
                    .wrap(from_fn(reject_anonymous_users)),
            )
            .service(
                web::resource("password/update")
                    .wrap(from_fn(reject_anonymous_users))
                    .route(web::post().to(password_update))
                    .route(web::get().to(show_password_update)),
            )
    );
}

pub fn snippet_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/snippet")
            .service(
                web::resource("view/{id}")
                    .route(web::get().to(snippet_view))
                    .wrap(from_fn(authenticate)),
            )
            .service(
                web::resource("create")
                    .route(
                        web::get()
                            .to(show_snippet_create)
                            .wrap(from_fn(reject_anonymous_users)),
                    )
                    .route(
                        web::post()
                            .to(snippet_create)
                            .wrap(from_fn(reject_anonymous_users)),
                    ),
            ),
    );
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/ping").route(web::get().to(ping)))
            .service(
                web::resource("/about").route(web::get().to(about))
                    .wrap(from_fn(authenticate)),
            )
            .service(
                web::resource("/")
                    .route(web::get().to(home))
                    .wrap(from_fn(authenticate)),
            ),
    );
}
