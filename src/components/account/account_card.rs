#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use web_sys::window;
use crate::components::shared::SharedMetadataVisibility;
use crate::styles::account_card_style::STYLE;
use crate::model::{LocalStorage, SessionStorage};

#[component]
pub fn AccountCard(state_metadata: SharedMetadataVisibility) -> Element {

    let profile_image = state_metadata.user_metadata.read().picture.clone();
    let display_name = state_metadata.user_metadata.read().display_name.clone();
    let nip05 = state_metadata.user_metadata.read().nip05.clone();

    let public_key = state_metadata.metadata.read().as_ref().map(|event| event.pubkey.to_hex()).unwrap_or_default();

    let handle_sign_out = move |_| {
        let key = format!("story-teller_{}", public_key);

        // ลบข้อมูลจาก Local Storage
        match LocalStorage::remove(&key) {
            Ok(_) => info!("Removed key: {}", key),
            Err(e) => error!("Error removing from Local Storage: {}", e),
        }

        // ลบข้อมูลจาก Session Storage
        match SessionStorage::remove("story-teller_follow_1") {
            Ok(_) => info!("Removed key: story-teller_follow_1"),
            Err(e) => error!("Error removing from Session Storage: {}", e),
        }

        // รีเฟรชหน้าเว็บ
        if let Some(win) = window() {
            match win.location().reload() {
                Ok(_) => info!("Page reloaded"),
                Err(e) => error!("Error reloading the page: {:?}", e),
            }
        }
    };


    rsx! {
        style { {STYLE} }
        div { class: "account-card",

            // เพิ่ม nip05 ไว้ด้านบนของรูปโปรไฟล์
            p { class: "nip05-info", "{nip05}" }

            img { class: "profile-image", id: "submit-on-card-pt",
                src: "{profile_image}",
                alt: "user",
            }

            div { class: "user-info",
                h3 { class: "user-name",
                    "{display_name}"
                }

                div { id: "submit-on-card-pt",
                    button { id: "submit-on-card",
                        r#type: "button",
                        "More settings"
                    }
                }

                div { id: "submit-on-card-pt",
                    button { id: "submit-on-card",
                        r#type: "button",
                        onclick: handle_sign_out,
                        "Sign Out"
                    }
                }

            }

        }
    }
}
