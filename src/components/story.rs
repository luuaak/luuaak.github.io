#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use nostr_sdk::{
    serde_json,
    Client,
    Event,
    EventSource,
    Filter,
    ToBech32,
    FromBech32,
    Kind,
    Metadata,
    PublicKey,
    EventId,
};
use std::time::Duration;
use nostr_sdk::prelude::{Nip19, Nip19Event, PREFIX_BECH32_NOTE_ID};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlImageElement;

use crate::components::anim::EllipsisLoading;
use crate::components::navigation_bar::FollowList;
use crate::components::StoryCard;
use crate::model::SessionStorage;
use crate::nostr::{Nip19Tool, NostrClient};
use crate::styles::story_style::STYLE;

const _IMG: manganis::ImageAsset =
    manganis::mg!(image("./src/assets/Untitled.webp"));

/// โครงสร้างข้อมูลสำหรับเก็บข้อมูล
/// Struct นี้จะเก็บข้อมูลเช่น id ของ story, รูปภาพ, ชื่อเรื่อง, บทสรุป, เวลาที่เผยแพร่,
/// ชื่อและรูปภาพของผู้เขียน
#[derive(Debug, Clone)]
pub struct StoryData {
    pub(crate) note_id: Option<String>,      // ค่า Event id ของ note
    pub(crate) image: Option<String>,        // รูปภาพของ note
    pub(crate) title: Option<String>,        // ชื่อของ note
    pub(crate) summary: Option<String>,      // สรุปของ note
    pub(crate) article: Option<String>,      // เนื้อหาบทความ
    pub(crate) published_at: Option<String>, // เวลาที่เผยแพร่
    pub(crate) npub: Option<String>,
    pub(crate) author_name: Option<String>,  // ชื่อผู้เขียน
    pub(crate) author_image: Option<String>, // รูปภาพผู้เขียน
}

pub async fn check_image(url: &str) -> bool {
    // Create a new HtmlImageElement
    let img = HtmlImageElement::new().unwrap();
    img.set_src(url);

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let img_clone = img.clone();
        let resolve = resolve.clone();
        let reject = reject.clone();

        let onload = Closure::wrap(Box::new(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        let onerror = Closure::wrap(Box::new(move || {
            reject.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        img_clone.set_onload(Some(onload.as_ref().unchecked_ref()));
        img_clone.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onload.forget();
        onerror.forget();
    });

    // Await the promise
    match JsFuture::from(promise).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// ฟังก์ชัน `extract_tags`
/// ฟังก์ชันนี้ใช้สำหรับดึงข้อมูลจาก tags ของ event ที่ได้รับ เช่น รูปภาพ ชื่อเรื่อง สรุป และเวลาที่เผยแพร่
/// และนำข้อมูลของผู้เขียนจาก parameter `author_name` และ `author_image` เข้ามาประกอบ
///
/// # Arguments
/// - `event`: ข้อมูล event ที่ได้รับจากเครือข่าย Nostr
/// - `author_name`: ชื่อผู้เขียน (เป็น Option เผื่อว่าอาจจะไม่มีข้อมูล)
/// - `author_image`: รูปภาพของผู้เขียน (เป็น Option เช่นกัน)
///
/// # Returns
/// คืนค่าเป็น Struct `StoryData` ที่ประกอบไปด้วยข้อมูล story ทั้งหมด
pub async fn extract_tags(
    event: Event,
    author_name: Option<String>,
    author_image: Option<String>,
) -> StoryData {
    // กำหนดประเภทของ tags ที่ต้องการหา
    let tags_to_find = ["image", "title", "summary", "published_at"];

    let mut image: Option<String> = None; // ตัวแปรเก็บค่า image
    let mut title: Option<String> = None; // ตัวแปรเก็บค่า title
    let mut summary: Option<String> = None; // ตัวแปรเก็บค่า summary
    let mut published_at: Option<String> = None; // ตัวแปรเก็บค่า published_at
    let note_id = Some(event.id.to_hex()); // แปลง id ของ event เป็นรูปแบบ hex และเก็บใน note_id
    let article = Some(event.content.clone());
    let npub = PublicKey::to_bech32(&event.pubkey).unwrap().into();
    //info!("event: {:#?}", event);

    // วนซ้ำเพื่อตรวจสอบ tags ภายใน event
    for tag in event.tags.iter() {
        let tag_data = tag.as_slice(); // แปลง tag เป็นรูปแบบเวกเตอร์
        // ตรวจสอบว่ามีข้อมูลใน tag และเป็น tag ที่เราต้องการหรือไม่
        if tag_data.len() > 1 && tags_to_find.contains(&tag_data[0].as_str()) {
            match tag_data[0].as_str() {
                "image" => {
                    image = {
                        let image = tag_data[1].to_string();
                        if check_image(&image).await {
                            Some(image)
                        } else {
                            Some(_IMG.to_string())
                        }
                    }
                } // หากเป็น image, เก็บข้อมูล
                "title" => title = Some(tag_data[1].to_string()), // หากเป็น title, เก็บข้อมูล
                "summary" => summary = Some(tag_data[1].to_string()), // หากเป็น summary, เก็บข้อมูล
                "published_at" => published_at = Some(tag_data[1].to_string()), // หากเป็น published_at, เก็บข้อมูล
                _ => {}
            }
        }
    }

    // คืนค่า StoryData ที่ประกอบไปด้วยข้อมูลที่เราดึงมาได้จาก tags และข้อมูลผู้เขียน
    StoryData {
        note_id,
        image,
        title,
        summary,
        article,
        published_at,
        npub,
        author_name,
        author_image,
    }
}

/// ฟังก์ชัน `Story`
/// เป็น component ที่ทำหน้าที่ดึงข้อมูล NIP-23 (Long-form Content) จากเครือข่าย Nostr
/// และแสดงผลออกมาเป็นรายการของ story โดยใช้ `StoryCard` component
#[component]
pub fn Story(npub_value: Option<String>) -> Element {

    // สร้าง signal เพื่อเก็บ events ที่ได้จากการดึงข้อมูล
    let events_signal: Signal<Vec<Event>> = use_signal(Vec::new);

    // เก็บข้อมูล StoryData ที่ประมวลผลแล้วจาก `event`
    let story_data_signal: Signal<Vec<StoryData>> = use_signal(Vec::new);

    use_future({
        // clone ตัวแปร signal ที่สร้างขึ้นเพื่อใช้ใน future
        let mut events_signal = events_signal.clone();
        let mut story_data_signal = story_data_signal.clone();

        // ฟังก์ชัน async ที่ดึงข้อมูล events จาก Nostr
        move || {
        let value = npub_value.clone();
        async move {
            let client = NostrClient::setup_and_connect()
                .await
                .expect("Failed to setup client");

            // ตรวจสอบว่า npub_value มีค่าเป็น Some หรือไม่
            let npub: String = value.unwrap_or_else(|| String::new());

            // สร้าง filter สำหรับดึงข้อมูล event ที่เป็นประเภท LongFormTextNote
            let mut filter = Filter::new().kind(Kind::LongFormTextNote);

            // ตรวจสอบว่า SessionStorage มีค่า key ที่ขึ้นต้นด้วย story-teller_
            let key_exists = SessionStorage::has_key_starting_with("story-teller_");

            let mut authors: Vec<PublicKey> = Vec::new();

            // ตรวจสอบว่าค่า npub ไม่เป็นค่าว่าง
            if !npub.is_empty() {
                // กำหนด filter ด้วย npub
                filter = filter.authors(vec![PublicKey::from_bech32(&npub).expect("Invalid npub format")]);
            } else if key_exists {
                // ถ้าค่า npub เป็นค่าว่าง ให้ดึงข้อมูลจาก SessionStorage
                if let Some(json_str) = SessionStorage::get("story-teller_follow_1") {
                    // แปลง JSON string เป็น FollowList object
                    let follow_list: FollowList = serde_json::from_str(&json_str)
                        .expect("Failed to parse follow list");

                    // วน loop เพื่อแปลง public_key เป็น PublicKey
                    for public_key in follow_list.public_key {
                        if let Ok(pk) = PublicKey::from_hex(&public_key) {
                            authors.push(pk); // เก็บค่า PublicKey ลงใน authors
                        }
                    }
                }
            }

            // ถ้ามีค่า authors ให้เพิ่มลงใน filter
            if !authors.is_empty() {
                info!("Now query follows list...");
                filter = filter.authors(authors);
            }

            // ดึงข้อมูล events จากเครือข่ายด้วย filter ที่เรากำหนด
            let events = client
                .get_events_of(
                    vec![filter],
                    EventSource::relays(Some(Duration::from_secs(10))),
                )
                .await;


            // ถ้าการดึงข้อมูลสำเร็จ
            if let Ok(events) = events {
                // อัพเดตค่า signal ด้วยรายการ events ที่ได้รับมา
                events_signal.set(events.clone());

                // สร้างเวกเตอร์เปล่าเพื่อเก็บ StoryData
                let mut stories: Vec<StoryData> = Vec::new();

                // วนซ้ำผ่านแต่ละ event เพื่อดึงข้อมูล Metadata ของผู้เขียน
                for event in events.iter() {
                    // สร้าง filter เพื่อดึง metadata ของผู้เขียน
                    let metadata_filter =
                        Filter::new().author(event.pubkey).kind(Kind::Metadata);

                    let metadata_events = client
                        .get_events_of(
                            vec![metadata_filter],
                            EventSource::relays(Some(Duration::from_secs(10))),
                        )
                        .await;

                    let mut author_name = None;
                    let mut author_image = None;

                    // ถ้าการดึงข้อมูล Metadata สำเร็จ
                    if let Ok(metadata_events) = metadata_events {
                        let mut ts = 0u64;
                        for metadata_event in metadata_events {
                            let user_metadata: Metadata =
                                serde_json::from_str::<Metadata>(
                                    &*metadata_event.content,
                                )
                                    .unwrap();

                            if !metadata_event.is_expired()
                                && ts.lt(&metadata_event.created_at.as_u64())
                            {
                                // ดึงข้อมูลชื่อและรูปภาพของผู้เขียนจาก Metadata
                                author_name = user_metadata.name.clone();
                                author_image = user_metadata.picture.clone();
                            }
                            ts = metadata_event.created_at.as_u64();

                            //info!("User Metadata - Name: {:?}, Picture: {:?}", author_name, author_image);
                        }
                    }

                    if author_image.is_some() {
                        let result =
                            check_image(author_image.clone().unwrap().as_str())
                                .await;
                        if !result {
                            let pk = event.pubkey.to_hex();
                            let image_proxy = format!(
                                "https://media.nostr.band/thumbs/{}/{}-picture-64",
                                &pk[60..],
                                pk);
                            let result = check_image(&image_proxy).await;
                            if result {
                                author_image = Some(image_proxy);
                            } else {
                                author_image = Some(_IMG.to_string());
                            }
                        }
                    } else {
                        author_image = Some(_IMG.to_string());
                    }

                    // ใช้ฟังก์ชัน extract_tags เพื่อดึงข้อมูลจาก event และ Metadata ของผู้เขียน
                    let story = extract_tags(
                        event.clone(),
                        author_name.clone(),
                        author_image.clone(),
                    )
                        .await;
                    stories.push(story);
                    story_data_signal.set(stories.clone());
                }


                // ตัดการเชื่อมต่อหลังจากดึงข้อมูลเสร็จ
                client.disconnect().await.expect("Failed to disconnect");
            } else {
                info!("Failed to retrieve events");
            }
        }
        }
    });

    // Signal สำหรับหน้าเซ็ตปัจจุบัน
    let mut current_page: Signal<usize> = use_signal(|| 0);

    // จำนวน Card ต่อหนึ่งหน้าเซ็ต
    const CARDS_PER_PAGE: usize = 20;
    // รายการข้อมูลทั้งหมด
    let stories = story_data_signal.read();
    // จำนวนทั้งหมดของหน้าเซ็ต
    let total_pages = (stories.len() + CARDS_PER_PAGE - 1) / CARDS_PER_PAGE;

    // ฟังก์ชันแบ่ง Card เป็นเซ็ต ๆ
    let displayed_stories: Vec<StoryData> = stories
        .iter()
        .skip(current_page.with(|p| *p) * CARDS_PER_PAGE)
        .take(CARDS_PER_PAGE)
        .cloned()
        .collect();

    rsx! {
        style { {STYLE} }

        // ถ้าข้อมูลใน story_data_signal ยังว่างอยู่ ให้แสดง EllipsisLoading
        if story_data_signal.read().is_empty() {
            EllipsisLoading {}
        } else {

            div { class: "displayed-box",
                div {
                    class: "note-container",
                    //style: "background-color: red;",

                    // ถ้ามีข้อมูลแล้ว ให้วนซ้ำแสดงผลแต่ละ story โดยใช้ StoryCard component
                    for story in displayed_stories {
                        StoryCard {
                            note_id: Nip19Tool::id_encode(story.note_id.clone().unwrap_or_default()),
                            image: story.image.clone().unwrap_or_else(|| _IMG.to_string()),
                            title: story.title.clone().unwrap_or_default(),
                            summary: story.summary.clone().unwrap_or_default(),
                            article: story.article.clone().unwrap_or_default(),
                            published_at: story.published_at.clone().unwrap_or_default(),
                            npub: story.npub.clone().unwrap_or_default(),
                            author_name: story.author_name.clone().unwrap_or("Unknown Author".to_string()),
                            author_image: story.author_image.clone().unwrap_or_default(),
                        }
                    }
                }

                div { class: "foot-pt",

                    section {
                        div { class: "btn-pagination",

                            ul {
                                // วนลูปเพื่อสร้างปุ่มสำหรับหน้าแรกถึงหน้าที่สาม
                                for i in 0..total_pages.min(3) {
                                    li {
                                        class: "page-item",
                                        // กำหนดการคลิกเพื่อเปลี่ยนหน้า
                                        label { onclick: move |_| current_page.set(i), "{i + 1}" }
                                    }
                                }

                                // ตรวจสอบว่าจำนวนหน้ามากกว่าสามหน้า
                                if total_pages > 3 {
                                    li {
                                        class: "page-item ellipsis",
                                        "..." // แสดงเครื่องหมายการข้ามหน้า
                                    }
                                    li {
                                        class: "page-item",
                                        // กำหนดการคลิกไปยังหน้าสุดท้าย
                                        label { onclick: move |_| current_page.set(total_pages - 1), "{total_pages}" }
                                    }
                                }

                                // เพิ่มปุ่ม "เซ็ตก่อนหน้า" ถ้าไม่อยู่ที่หน้าแรก
                                if current_page() > 0 {
                                    li {
                                        class: "page-item previous",
                                        // กำหนดการคลิกเพื่อไปหน้าก่อนหน้า
                                        label { onclick: move |_| current_page.set(current_page - 1), "Previous" }
                                    }
                                }

                                // ตรวจสอบว่าไม่อยู่ที่หน้าสุดท้าย
                                if current_page() < total_pages - 1 {
                                    li {
                                        class: "page-item next",
                                        // กำหนดการคลิกเพื่อไปหน้าถัดไป
                                        label { onclick: move |_| current_page.set(current_page + 1), "Next" }
                                    }
                                }
                            }


                        }
                    }
                }
            }

        }
    }

}
