#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::NavigationBar;
use crate::styles::home_layout_style::STYLE;

#[component]
pub fn ProfilePage() -> Element {
    rsx! {
        style { {STYLE} }
        NavigationBar {}
    }
}