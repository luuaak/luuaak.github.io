#![allow(non_snake_case)]

use std::time::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use nostr_sdk::{
    Event,
    EventId,
    EventSource,
    Metadata,
    Filter,
    Kind,
    PublicKey,
    ToBech32,
    FromBech32,
    serde_json,
};
use crate::components::content::NewStory;
use crate::components::Story;
use crate::components::story::check_image;
use crate::model::LocalStorage;
use crate::nostr::NostrClient;
use crate::styles::profile_style::STYLE;

const _BANNER: manganis::ImageAsset = manganis::mg!(image("./src/assets/banner.jpg"));
const _PROFILE: manganis::ImageAsset = manganis::mg!(image("./src/assets/Untitled.webp"));
const _EDIT: &str = manganis::mg!(file("src/assets/edit.svg"));
const _MORE: &str = manganis::mg!(file("src/assets/more.svg"));


// ฟังก์ชันดึงข้อมูลจาก Local Storage และตรวจสอบการ Login แล้ว
async fn check_account_activation(
    npub_value: String,
    mut activate_account: Signal<bool>,
) {
    // ดึงคีย์ทั้งหมดที่ขึ้นต้นด้วย "story-teller_"
    if let Some(keys) = LocalStorage::get_all_keys() {
        if keys.is_empty() {
            activate_account.set(false);
        } else {
            // ตรวจสอบแต่ละคีย์ใน Local Storage
            for key in keys {
                if let Some(json_str) = LocalStorage::get(&key) {
                    if let Ok(event_obj) = serde_json::from_str::<Event>(&json_str) {
                        let pubkey_ls = event_obj.pubkey;
                        let pubkey_url = FromBech32::from_bech32(&npub_value)
                            .expect("Invalid npub format");

                        // ตรวจสอบว่าค่าของ pubkey ตรงกันหรือไม่
                        if pubkey_ls == pubkey_url {
                            activate_account.set(true);
                            break;
                        } else {
                            activate_account.set(false);
                        }
                    } else {
                        info!("Failed to parse JSON for key: {}", key);
                    }
                } else {
                    info!("No data found for key: {}", key);
                }
            }
        }
    } else {
        activate_account.set(false);
    }
}


#[component]
pub fn ProfileDetail(npub: Option<String>) -> Element {
    let mut show_new_post = use_signal(|| false);
    let mut show_article_list = use_signal(|| false);
    let metadata_signal = use_signal::<Option<Metadata>>(|| None);

    let activate_account = use_signal(|| false);

    // กำหนดค่าเริ่มต้นให้แสดง Story {}
    let is_story_visible = !*show_new_post.read() && !*show_article_list.read();

    let handle_new_post = move |_| {
        show_new_post.set(true); // แสดง NewStory
        show_article_list.set(false); // ซ่อน Article List
    };

    let handle_article_list = move |_| {
        show_article_list.set(true); // แสดง Story
        show_new_post.set(false); // ซ่อน NewStory
    };


    // เรียกใช้ use_future เพื่อเช็คการเปิดใช้งานบัญชี
    use_future({
        let activate_account = activate_account.clone();
        let npub_value = npub.clone().unwrap_or_default();

        move || {
            let activate_account = activate_account.clone();
            let npub_value = npub_value.clone();
            async move {
                check_account_activation(npub_value, activate_account).await;
            }
        }
    });


    use_future({
        let value = npub.clone();
        let npub_value = value.unwrap_or_else(|| String::new());
        let mut metadata_signal = metadata_signal.clone();

        move || {
            let npub_value = npub_value.clone();
            async move {
                let client = NostrClient::setup_and_connect()
                    .await.expect("Failed to setup client");

                let pubkey = PublicKey::from_bech32(npub_value)
                    .expect("Invalid npub format");

                let metadata_filter = Filter::new()
                    .author(pubkey)
                    .kind(Kind::Metadata);

                let metadata_events = client
                    .get_events_of(
                        vec![metadata_filter],
                        EventSource::relays(Some(Duration::from_secs(10))),
                    )
                    .await;

                if let Ok(events) = metadata_events {
                    for event in events {
                        if let Ok(mut metadata) = serde_json::from_str::<Metadata>(&event.content) {

                            // ตรวจสอบลิงก์ของฟิลด์ `picture`
                            if let Some(ref picture_url) = metadata.picture {
                                if !check_image(picture_url).await {
                                    let pk = event.pubkey.to_hex();
                                    let fallback_url = format!(
                                        "https://media.nostr.band/thumbs/{}/{}-picture-64",
                                        &pk[60..],
                                        pk
                                    );
                                    if check_image(&fallback_url).await {
                                        metadata.picture = Some(fallback_url);
                                    } else {
                                        metadata.picture = Some(_PROFILE.to_string());
                                    }
                                }
                            }

                            // ตรวจสอบลิงก์ของฟิลด์ `banner`
                            if let Some(ref banner_url) = metadata.banner {
                                if !check_image(banner_url).await {
                                    metadata.banner = Some(_BANNER.to_string());
                                }
                            }

                            // เก็บ Metadata ที่ตรวจสอบแล้วใน metadata_signal
                            metadata_signal.set(Some(metadata));
                        }
                    }
                }
            }
        }
    });

    // อ่านค่าจาก metadata_signal
    let metadata = metadata_signal.read().clone();
    let name = metadata.as_ref().and_then(|m| m.display_name.clone()).unwrap_or_else(|| "Unknown".to_string());
    //let about = metadata.as_ref().and_then(|m| m.about.clone()).unwrap_or_else(|| "No bio available".to_string());
    let picture = metadata.as_ref().and_then(|m| m.picture.clone()).unwrap_or_else(|| _PROFILE.to_string());
    let banner = metadata.as_ref().and_then(|m| m.banner.clone()).unwrap_or_else(|| _BANNER.to_string());


    // อ่านค่าจาก activate_account
    let is_activated = *activate_account.read();

    rsx! {
        style { {STYLE} }

        div { class: "profile-box",
            div { class: "banner-box col-xs-hidden",
                img { src: "{banner}" }
            }

            div { class: "profile-info",
                div { class: "profile-bar",
                    div { class: "profile-field-image",
                        img { src: "{picture}", alt: "Profile Image" }
                        span { class: "profile-name", "{name}" }
                    }

                    div { class: "profile-field-menu",
                        if is_activated {
                            button { class: "menu-btn",
                                r#type: "button",
                                onclick: handle_new_post,
                                span { "New Post" }
                            }
                        }

                        button { class: "menu-btn",
                            r#type: "button",
                            onclick: handle_article_list,
                            span { "Article List" }
                        }
                    }

                    div { class: "profile-field-options-btn",
                        if is_activated {
                            button { class: "edit-btn",
                                r#type: "button",
                                img { src: "{_EDIT}", alt: "Edit Icon" }
                                span { "Edit Profile" }
                            }
                        }

                        button { class: "more-btn",
                            r#type: "button",
                            img { src: "{_MORE}", alt: "More Icon" }
                        }
                    }
                }
            }
        }

        div { class: "content-box",
            if is_story_visible {
                Story { npub_value: npub }
                //NewStory {}
            } else if *show_new_post.read() {
                NewStory {}
            } else if *show_article_list.read() {
                Story { npub_value: npub }
            }
        }
    }
}
