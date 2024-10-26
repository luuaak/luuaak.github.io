#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::ellipsis_loading_style::STYLE;

#[component]
pub fn EllipsisLoading() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "lds-ellipsis-container",
            div { class: "lds-ellipsis",
                 div {} div {} div {} {}
            }
        }
    }
}
