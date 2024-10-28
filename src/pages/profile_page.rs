#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::account::ProfileDetail;
use crate::components::NavigationBar;
use crate::styles::home_layout_style::STYLE;

#[component]
pub fn ProfilePage(npub: String) -> Element {
    rsx! {
        style { {STYLE} }
        NavigationBar {}
        div { class: "",
            ProfileDetail { npub: npub }
        }
    }
}