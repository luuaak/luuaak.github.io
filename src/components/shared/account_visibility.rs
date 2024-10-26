#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SharedAccountVisibility {
    pub show_account: Signal<bool>,
}

impl SharedAccountVisibility {

    pub fn new() -> Self {
        let show_account = use_signal(|| false);
        Self { show_account }
    }

}
