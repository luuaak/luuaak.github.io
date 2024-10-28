use dioxus::prelude::*;
use crate::pages::{HomePage, ArticlePage, ProfilePage, ErrorPage};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/profile/:npub")]
    ProfilePage { npub: String },
    #[route("/story/id/:event_id")]
    ArticlePage { event_id: String },
    #[route("/error")]
    ErrorPage {},
}
