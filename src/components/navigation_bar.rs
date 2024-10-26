#![allow(non_snake_case)]

use crate::pages::router::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, error};
use crate::components::account::{AuthCard, AccountCard};
use crate::styles::navigation_bar_style::STYLE;
use std::time::Duration;
use nostr_sdk::{
    serde_json,
    EventBuilder,
    EventSource,
    PublicKey,
    Filter,
    Kind,
    Event
};
use serde::{Deserialize, Serialize};
use crate::components::shared::{SharedAccountVisibility, SharedAuthVisibility, SharedMetadataVisibility};
use crate::model::{LocalStorage, SessionStorage};
use crate::nostr::NostrClient;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));
//const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_2.jpg"));

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FollowList {
    pub(crate) public_key: Vec<String>,
}

fn process_event(event: &Event) -> FollowList {
    let mut follow_list = FollowList { public_key: Vec::new() };

    // ตรวจสอบ tags ใน Event
    for tag in &event.tags {
        let tag_data = tag.as_slice(); // แปลง tag เป็น slice

        // ตรวจสอบว่า tag มี prefix เป็น "p" และมีอย่างน้อย 2 ค่า
        if tag_data.len() > 1 && tag_data[0] == "p" {
            // เพิ่ม value เข้าไปใน FollowList
            follow_list.public_key.push(tag_data[1].to_string());
        }

    }

    follow_list
}

#[component]
pub fn NavigationBar() -> Element {

    let navigator: Navigator = use_navigator();

    let mut state_auth = SharedAuthVisibility::new();
    let mut state_account = SharedAccountVisibility::new();
    let mut state_metadata = SharedMetadataVisibility::new();



    // สร้าง Signal เพื่อควบคุมการแสดง AccountCard
    let mut show_account_card = use_signal(|| false);

    // เก็บ key จาก LocalStorage
    let mut story_teller_keys = use_signal(|| Vec::new());

    // ตรวจสอบ Local Storage
    use_future( move || async move {
        // ดึงค่า keyทั้งหมดใน LocalStorage
        if let Some(storage) = LocalStorage::get_all_keys() {
            // ไม่ต้องกรองอีกต่อไปเนื่องจาก get_all_keys คืนคีย์ที่ต้องการแล้ว
            if !storage.is_empty() {
                story_teller_keys.set(storage);
                state_account.show_account.set(true);
            }
        }
    });


    use_future( move || async move {
        if !story_teller_keys.is_empty() {
            // ดึงค่าคีย์ตำแหน่งแรกจาก LocalStorage
            if let Some(first_key) = story_teller_keys.get(0) {

                // ดึงข้อมูลที่เกี่ยวข้องจาก LocalStorage
                if let Some(data) = LocalStorage::get(&first_key) {
                    let client = NostrClient::setup_and_connect().await.expect("Failed to setup client");
                    let metadata = serde_json::from_str::<Event>(&data).unwrap();

                    let follow_filter = Filter::new()
                        .authors(vec![metadata.pubkey])
                        .kind(Kind::ContactList);

                    let events = client
                        .get_events_of(
                            vec![follow_filter],
                            EventSource::relays(Some(Duration::from_secs(10))),
                        )
                        .await;

                    if let Ok(events) = events {
                        // เลือก Event ที่มีค่า created_at มากที่สุด
                        if let Some(latest_event) = events.iter().max_by_key(|e| e.created_at) {
                            //info!("Latest Follow List Event received: {:?}", latest_event);

                            // ส่ง Event ไปยังฟังก์ชัน process_event เพื่อประมวลผล
                            let follow_list = process_event(latest_event);
                            //info!("Follow List: {:?}", follow_list);

                            let follow_list_string = serde_json::to_string(&follow_list).unwrap();

                            // ตรวจสอบว่ามี key ที่ขึ้นต้นด้วย "story-teller_" อยู่ใน SessionStorage หรือไม่
                            if let Some(existing_keys) = SessionStorage::get_all_keys() {
                                // ถ้าไม่มี key ที่ขึ้นต้นด้วย "story-teller_" ให้บันทึกข้อมูลใหม่
                                if !existing_keys.iter().any(|key| key.starts_with("story-teller_")) {
                                    match SessionStorage::set("story-teller_follow_1", &follow_list_string) {
                                        Ok(_) => info!("Follow List saved to Session Storage"),
                                        Err(err) => error!("Failed to save to Session Storage: {}", err),
                                    }
                                } else {
                                    info!("Key 'story-teller_' exists. Skipping save.");
                                }
                            } else {
                                // ถ้าไม่สามารถดึง key จาก SessionStorage ได้ (อาจเกิดข้อผิดพลาด)
                                match SessionStorage::set("story-teller_follow_1", &follow_list_string) {
                                    Ok(_) => info!("Follow List saved to Session Storage"),
                                    Err(err) => error!("Failed to save to Session Storage: {}", err),
                                }
                            }


                        }

                    }

                }

            }
        }
    });



    // ตรวจสอบว่า show_account ถูกตั้งค่าหรือไม่
    use_future( move || async move {
        if !story_teller_keys.is_empty() {
            // ดึงค่าคีย์คำแหน่งแรก
            if let Some(first_key) = story_teller_keys.get(0) {

                // ดึงข้อมูลจาก LocalStorage
                if let Some(data) = LocalStorage::get(&first_key) {

                    state_metadata.raw_metadata.set(data.to_string().clone());
                    state_metadata.metadata.set(serde_json::from_str(&data).unwrap());

                    if let Some(event) = state_metadata.metadata.read().as_ref() {
                        // ดึง content จาก event
                        let metadata_content = &event.content;
                        //info!("{}", metadata_content);

                        state_metadata.user_metadata.set(serde_json::from_str(metadata_content).unwrap());
                    } else {
                        error!("No event metadata available.");
                    }

                }
            }
        }
    });


    // ใช้ แสดงข้อมูลเมื่อ show_account เป็น true
    use_future( move || async move {
        if *state_account.show_account.read() {

            // สร้าง Event object จาก JSON string
            // let data_object: Event = serde_json::from_str(&data_string).unwrap();
            // let content = serde_json::from_str(&data_object.content).unwrap();

            // อ่านค่าจาก raw_metadata
            let metadata = state_metadata.raw_metadata.read();
            //info!("{}", &metadata);
            // let public_key = state_metadata.metadata.read().as_ref().map(|event| event.pubkey).unwrap();
            // let event = EventBuilder::text_note("POW text note from rust-nostr", []).to_unsigned_event(public_key);
            // let event_str = serde_json::to_string(&event).unwrap();
            // info!("event_str: {}", event_str);
        }
    });

    // use_effect(move || {
    //     let profile_image = state_metadata.user_metadata.read().picture.clone();
    //     info!("profile image update {}", profile_image);
    // });

    let profile_image = use_signal(|| state_metadata.user_metadata.read().picture.clone());

    use_effect({
        let state_metadata = state_metadata.clone();
        let mut profile_image = profile_image.clone();
        move || {
            // เมื่อ user_metadata เปลี่ยนแปลง ให้ทำการอัปเดต profile_image
            profile_image.set(state_metadata.user_metadata.read().picture.clone());

            info!("Profile image updated: {}", profile_image.read());
            ()
        }
    });


    rsx! {
        style { {STYLE} }
        div { class: "item-nav", id: "nav",
            img {
                src: "{IMG_BANNER}",
                onclick: move |_| {
                    navigator.push(Route::HomePage {});
                }
            }

            if *state_account.show_account.read() {

                // แสดงรูป Profile หากมีข้อมูลใน Local storage
                div { class: "nav-profile-round",
                    button {
                        onclick: move |_| {
                            let mut is_show_account = show_account_card.write();
                            *is_show_account = !*is_show_account;
                            info!("Profile clicked");
                        },
                        img { src: "{profile_image.read()}" }
                    }
                }

                if *show_account_card.read() {
                    AccountCard { state_metadata: state_metadata.clone() }
                }

            } else {
                // แสดงปุ่ม Login หากไม่มีข้อมูลใน Local storage
                div { class: "col-xs-3 col-sm-3 col-md-2 col-lg-1 col-xl-1",
                    button { class: "nav-login login-position",
                        onclick: move |_| {
                            let mut is_dropdown = state_auth.show_auth_card.write();
                            *is_dropdown = !*is_dropdown;
                            info!("Login clicked");
                        },
                        "Login"
                    }
                }

                if *state_auth.show_auth_card.read() {
                    // หน้า UI ตัวเลือกสำหรับ Login
                    AuthCard {
                        state_auth: state_auth.clone(),
                        state_account: state_account.clone()
                    }
                }

            }
        }
    }
}
