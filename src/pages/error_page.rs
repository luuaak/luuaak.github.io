#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::router::Route;
use crate::styles::error_style::STYLE;

/// https://dribbble.com/shots/3965778-Cezan-404-Page-Not-Found
/// Cezan - 404 Page Not Found' by Masoud Ardestani

#[component]
pub fn ErrorPage() -> Element {
    let navigator: Navigator = use_navigator();
    rsx! {
        style { {STYLE} }
        div { class: "center",

            div {
                class: "error",
                div {
                    class: "number", "4"
                }
                div {
                    class: "illustration",
                    div {
                        class: "circle"
                    }
                    div {
                        class: "clip",
                        div {
                            class: "paper",
                            div {
                                class: "face",
                                div {
                                    class: "eyes",
                                    div {
                                        class: "eye eye-left"
                                    }
                                    div {
                                        class: "eye eye-right"
                                    }
                                }
                                div {
                                    class: "rosyCheeks rosyCheeks-left"
                                }
                                div {
                                    class: "rosyCheeks rosyCheeks-right"
                                }
                                div {
                                    class: "mouth"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "number", "4"
                }
            }
            div {
                class: "text", "Oops. The page you're looking for doesn't exist."
            }
            a {
                class: "button",
                onclick: move |_| {
                    navigator.push(Route::HomePage {});
                },
                "Back Home"
            }

        }


    }
}
