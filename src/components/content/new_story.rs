#![allow(non_snake_case)]

use std::str::FromStr;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use nostr_sdk::{FromBech32, Keys, Kind, SecretKey, UncheckedUrl};
use nostr_sdk::nips::nip07;
use crate::nostr::NostrClient;
use nostr_sdk::prelude::{
    Nip07Signer,
    UnsignedEvent,
    PublicKey,
    Timestamp,
    EventId,
    Tag
};
use serde::__private::de::TagContentOtherField;
use web_sys::window;
use crate::components::content::Markdown;
use crate::components::content::markdown::markdown_to_html;
use crate::styles::new_story_style::STYLE;

const _PLUS: &str = manganis::mg!(file("src/assets/plus.svg"));
const _EDIT_TEXT: &str = manganis::mg!(file("src/assets/text.svg"));
const _PREVIEW: &str = manganis::mg!(file("src/assets/preview.svg"));

#[component]
pub fn NewStory() -> Element {

    // สร้างตัวแปร use_signal สำหรับเก็บข้อมูลจาก input
    let mut input_text = use_signal(|| None::<String>);

    let mut input_title = use_signal(|| None::<String>);
    let mut input_summary = use_signal(|| None::<String>);
    let mut input_image_url = use_signal(|| None::<String>);

    // สร้างตัวแปร use_signal สำหรับเก็บสถานะการแสดง textarea
    let mut show_textarea = use_signal(|| true);

    // สร้างตัวแปรเพื่อควบคุมการทำงานของ use_future เมื่อกดปุ่ม Submit
    let mut submit_trigger = use_signal(|| false);


    let content = markdown_to_html(input_text.read().as_deref().unwrap_or(""));
    info!("HTML content: {}", content);

    if *submit_trigger.read() {
        // Logic สำหรับการทำงานหลังจากกด Submit โดยใช้ use_future
        use_future(move || {
            let input_text = input_text.clone();
            let input_title = input_title.clone();
            let input_summary = input_summary.clone();
            let input_image_url = input_image_url.clone();

            let mut submit_trigger = submit_trigger.clone();

            async move {

                let content = input_text.read().as_deref().unwrap_or("").to_string();
                if content.is_empty() {
                    error!("Content is empty. Submission aborted.");
                    return;
                }

                let client = NostrClient::setup_and_connect().await
                    .expect("Failed to setup client");


                // สร้าง SecretKey จากคีย์ Bech32 ที่คุณให้มา
                let secret_key = SecretKey::from_bech32("nsec1htrkd6wqtj2x0g02detk7ud3w0mzshxz3erkkj7jgu8rlg3hml0sejqzwf")
                    .expect("Invalid Secret Key");

                // สร้าง Keys จาก SecretKey
                let my_keys = Keys::new(secret_key);

                // สร้าง PublicKey
                let pubkey = PublicKey::from_str("565843b28205d6688cb3a522e09b763f6eaf159fdb8ca51d9324a4284fe37c09").unwrap();
                let created_at = Timestamp::now();
                let kind = Kind::LongFormTextNote;

                // สร้าง Tags ที่ต้องการใช้
                let tags = vec![
                    Tag::title(input_title.unwrap()), // สร้าง tag title
                    Tag::description(input_summary.unwrap()), // สร้าง tag description
                    Tag::image(
                        UncheckedUrl::new(input_image_url.unwrap()),
                        None,
                    ),
                    Tag::alt("This is a short summary."), // สร้าง tag summary
                ];


                // สร้าง UnsignedEvent
                let unsigned_event = UnsignedEvent {
                    id: None,
                    pubkey,
                    created_at,
                    kind,
                    tags,
                    content,
                };

                // เซ็น Event
                let signed_event = unsigned_event.sign(&my_keys).expect("Failed to sign event");

                info!("Signed Event: {:?}", signed_event);

                if let Some(win) = window() {
                    win.location().reload().expect("Failed to reload");
                }

                client.send_event(signed_event).await
                    .expect("TODO: panic message");


                // match Nip07Signer::new() {
                //     Ok(signer) => {
                //         let pubkey_str = signer.get_public_key().await.unwrap();
                //         let pubkey = PublicKey::from_str(&pubkey_str.to_hex()).unwrap();
                //
                //         let created_at = Timestamp::now(); // กำหนดเวลา Unix time ปัจจุบัน
                //         let kind = Kind::LongFormTextNote;
                //         let tags = vec![
                //
                //         ];
                //         // nsec1htrkd6wqtj2x0g02detk7ud3w0mzshxz3erkkj7jgu8rlg3hml0sejqzwf
                //
                //         let unsigned_event = UnsignedEvent {
                //             id: None, // `id` จะถูกสร้างตอนที่เซ็น event
                //             pubkey,
                //             created_at,
                //             kind,
                //             tags,
                //             content: content.clone(),
                //         };
                //
                //         info!("Unsigned Event: {:?}", unsigned_event);
                //
                //         match signer.sign_event(unsigned_event).await {
                //             Ok(signed_event) => {
                //                 info!("Event signed successfully: {:?}", signed_event);
                //
                //                 client.send_event(signed_event).await
                //                     .expect("TODO: panic message");
                //
                //                 // รีเซ็ตค่า submit_trigger กลับเป็น false
                //                 submit_trigger.set(false);
                //             }
                //             Err(e) => error!("Failed to sign event: {:?}", e),
                //         }
                //     }
                //     Err(e) => error!("Error initializing Nip07Signer: {:?}", e),
                // }



            }
        });



    }


    rsx! {
        style { {STYLE} }
        div { class: "story-write-box",

        div { class: "story-header-box",
            span { "Add a title" }
            div { class: "input-text-title",
                input {
                    class: "input-field",
                    placeholder: "Enter your title here",
                    value: input_title.read().as_deref().unwrap_or(""),
                    oninput: move |evt| {
                        let value = evt.value().clone();
                        input_title.set(Some(value));
                    }
                }
            }
        }

        div { class: "story-header-box",
            span { "Add a summary" }
            div { class: "input-text-summary",
                input {
                    class: "input-field",
                    placeholder: "Enter your summary here",
                    value: input_summary.read().as_deref().unwrap_or(""),
                    oninput: move |evt| {
                        let value = evt.value().clone();
                        input_summary.set(Some(value));
                    }
                }
            }
        }

        div { class: "story-header-box",
            span { "Add a image URL" }
            div { class: "input-text-image-url",
                input {
                    class: "input-field",
                    placeholder: "Enter your image url here",
                    value: input_image_url.read().as_deref().unwrap_or(""),
                    oninput: move |evt| {
                        let value = evt.value().clone();
                        input_image_url.set(Some(value));
                    }
                }
            }
        }


            div { class: "write-box",
                div { class: "title-box",
                    span { "Add a story" }
                    div { class: "action-btn",
                        button { class: "submit-btn",
                            r#type: "button",
                            onclick: move |_| {
                                if let Some(ref text) = *input_text.read() {
                                    info!("Input Text Submitted: {}", text);
                                    // กำหนดค่าให้ submit_trigger เป็นตีวกระตุ้นเพื่อให้ use_future ทำงาน
                                    submit_trigger.set(true);
                                }
                            },
                            span { "Submit" }
                        }
                        button { class: "cancel-btn",
                            r#type: "button",
                            onclick: move |_| {
                                // ล้างข้อมูลใน textarea
                                input_text.set(None);
                            },
                            span { "Cancel" }
                        }
                    }
                }

                div { class: "option-btn",
                    button { class: "option-btn-item",
                        r#type: "button",
                        onclick: move |_| show_textarea.set(true),
                        img { src: "{_EDIT_TEXT}" },
                        span { "Edit" }
                    }
                    button { class: "option-btn-item",
                        r#type: "button",
                        onclick: move |_| show_textarea.set(false),
                        img { src: "{_PREVIEW}" },
                        span { "Preview" }
                    }
                }

                // แสดง textarea เฉพาะเมื่อ show_textarea เป็น true
                if *show_textarea.read() {
                    textarea {
                        class: "input-long-text",
                        placeholder: "Type...",
                        rows: "10",
                        // ใช้ค่าใน input_text เป็นค่าเริ่มต้นของ textarea
                        value: input_text.read().as_deref().unwrap_or(""),
                        oninput: move |evt| {
                            let value = evt.value().clone();
                            input_text.set(Some(value));
                        }
                    }
                } else {
                    // แสดงผลข้อความในโหมด Preview โดยดึงข้อมูลจาก input_text
                    Markdown { text: content }
                }
            }
        }
    }
}
