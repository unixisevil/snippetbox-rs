use askama::Template; // bring trait in scope
use std::collections::HashMap;
use std::ops::Deref;
use chrono::prelude::*;
use crate::domain::{Snippet, SnippetCreateForm, LoginForm, SignupForm};

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    is_authenticated: bool,
    current_year: i32,
    flash: &'a str,
    csrf_token: &'a str,
}

impl<'a> BaseTemplate<'a> {
    pub fn new(flash:  &'a (impl AsRef<str> + ?Sized)  , csrf: &'a (impl AsRef<str> + ?Sized) , is_authenticated: bool) -> Self {
         BaseTemplate {
            current_year: Utc::now().year(),
            flash: flash.as_ref(),
            csrf_token: csrf.as_ref(),
            is_authenticated,
        }
    }
}

#[derive(Template)]
#[template(path = "view.html")]
pub struct ViewTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub snippet: Option<Snippet>,
}

impl<'a> Deref for ViewTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub form: SnippetCreateForm,
    pub error_map: HashMap<&'static str, &'static str>,
}

impl<'a> Deref for CreateTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
   pub _parent: &'a BaseTemplate<'a>,
   pub error_map: HashMap<&'static str, &'static str>,
   pub form: LoginForm,
}

impl<'a> Deref for LoginTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub error_map: HashMap<&'static str, &'static str>,
    pub form: SignupForm,
}

impl<'a> Deref for SignupTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub snippets: Vec<Snippet>,
}

impl<'a> Deref for HomeTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

