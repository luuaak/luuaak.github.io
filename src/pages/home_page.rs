#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    NavigationBar,
    CheckBox,
    SearchBar,
    Story
};
use crate::styles::home_layout_style::STYLE;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        style { {STYLE} }

        NavigationBar {}
        SearchBar {}
        div { class: "",
            div { class: "control-box",
                //style: "background-color:yellow;",

                div { class: "col-xs-12 col-sm-4 col-md-4 col-lg-3 col-xl-2",
                    //style: "background-color:red;", // สำหรับ Debug
                    CheckBox {}
                }
                div { class: "col-xs-12 col-sm-8 col-md-8 col-lg-9 col-xl-10",
                    //style: "background-color:green;", // สำหรับ Debug
                    Story {}
                }
            }
        }


    }
}