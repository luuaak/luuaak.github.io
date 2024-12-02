#![allow(non_snake_case)]

use std::str::FromStr;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use std::time::Duration;
use nostr_sdk::nips::nip07;
use nostr_sdk::{
    serde_json,
    EventSource,
    Filter,
    Kind,
    Event
};
use web_sys::window;
use crate::components::shared::{
    SharedAccountVisibility,
    SharedAuthVisibility
};
use crate::model::LocalStorage;
use crate::nostr::NostrClient;
use crate::styles::auth_card_style::STYLE;

const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));
const _CROSS: &str = manganis::mg!(file("src/assets/fi-rr-cross-small.svg"));

// ฟังก์ชันบันทึก event ลง localStorage
fn store_events_in_local_storage(events: Vec<Event>, public_key: String) {
    for event in events {
        let json_string = serde_json::to_string(&event).unwrap();
        let key = format!("story-teller_{}", public_key);

        // เรียกใช้ฟังก์ชัน set จาก local_storage.rs
        if let Err(e) = LocalStorage::set(&key, &json_string) {
            error!("Failed to store event with key: {}. Error: {}", key, e);
        } else {
            info!("Stored event with key: {}", key);
        }
    }
}

#[component]
pub fn AuthCard(state_auth: SharedAuthVisibility, state_account: SharedAccountVisibility) -> Element {

    // ฟังก์ชันจัดการการคลิกที่ overlay เพื่อซ่อน AuthCard
    let handle_click_overlay = move |_| {
        state_auth.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม cross เพื่อปิด Card
    let handle_click_cross = move |_| {
        state_auth.show_auth_card.set(false);
    };


    // ฟังก์ชันจัดการการคลิกที่ปุ่ม "Sign in with nsec"
    let handle_sign_in_with_nsec = move |_| {
        info!("Sign in with nsec clicked!");
    };


    // ฟังก์ชันจัดการการคลิกที่ปุ่ม "Sign in with extension"
    let handle_sign_in_with_extension = move |_| {
        info!("Sign in with extension clicked!");

        use_future({
            let mut state_auth = state_auth.clone();
            let mut state_account = state_account.clone();
            move || async move {
                let client = NostrClient::setup_and_connect()
                    .await.expect("Failed to setup client");

                match nip07::Nip07Signer::new() {
                    Ok(signer) => {
                        match signer.get_public_key().await {
                            Ok(public_key) => {
                                match NostrClient::setup_and_connect().await {
                                    Ok(client) => {
                                        let filter = Filter::new()
                                            .author(public_key)
                                            .kind(Kind::Metadata);

                                        let events = client
                                            .get_events_of(
                                                vec![filter],
                                                EventSource::relays(Some(Duration::from_secs(10))),
                                            )
                                            .await;

                                        info!("Events received: {:?}", events);

                                        if let Ok(events) = events {
                                            store_events_in_local_storage(events, public_key.to_hex());

                                            // อัพเดทสถานะหลังบันทึกข้อมูล
                                            state_account.show_account.set(true);
                                            state_auth.show_auth_card.set(false);

                                            // รีโหลดหน้าเว็บหลังจากสำเร็จ
                                            if let Some(win) = window() {
                                                win.location().reload().expect("Failed to reload");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Error setting up client: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error getting public key: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error initializing Nip07Signer: {:?}", e);
                    }
                }
            }
        });
    };


    rsx! {
        style { {STYLE} }

        div { id: "overlay",
            onclick: handle_click_overlay
        }

        div { id: "form-ui",
            div { id: "form-card",

                div { id: "close-button",
                    onclick: handle_click_cross,
                    img { src: "{_CROSS}" }
                }

                form { id: "form",
                    div { id: "form-body",

                        div { id: "welcome-lines",
                            div { id: "logo-top-card",
                                img { src: "{_ICON}" }
                            }
                            div { id: "welcome-line-2", "Sign in to Story Teller" }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button",
                                r#type: "button",
                                onclick: handle_sign_in_with_extension,
                                "Sign in with extension",
                            }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button",
                                r#type: "button",
                                onclick: handle_sign_in_with_nsec,
                                "Sign in with nsec"
                            }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button",
                                r#type: "button",
                                "Sign up"
                            }
                        }

                    }

                }
            }
        }
    }
}
