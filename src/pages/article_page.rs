#![allow(non_snake_case)]

use std::str::FromStr;
use std::time::Duration;
use dioxus::prelude::*;
use crate::components::{NavigationBar, SearchBar};
use crate::components::content::{Article, ArticleAuthor};
use crate::model::SessionStorage;
use dioxus_logger::tracing::info;
use nostr_sdk::{EventId, EventSource, Metadata, Filter, Kind, PublicKey, ToBech32};
use crate::components::anim::EllipsisLoading;
use crate::components::story::{
    check_image,
    extract_tags,
    StoryData
};
use crate::components::story_card::StoryCardProps;
use crate::nostr::{Nip19Tool, NostrClient};
use crate::styles::article_layout_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/Untitled.webp"));

/*
#[derive(Clone, Debug)]
struct TagsInfo {
    tags: Vec<String>,
}

fn process_tags(event: &nostr_sdk::Event) -> TagsInfo {

    let mut tags_list = TagsInfo { tags: Vec::new() };
    info!("Tags: {:#?}", event);
    for tag in &event.tags {
        let tag_data = tag.as_slice();
        if tag_data.len() > 1 && tag_data[0] == "t" {
            tags_list.tags.push(tag_data[1].to_string());
        }
    }

    tags_list
}
*/

#[component]
pub fn ArticlePage(event_id: String) -> Element {

    let note_id = Nip19Tool::id_decode(event_id.clone());

    let mut image: String = String::new();
    let mut title: String = String::new();
    let mut summary: String = String::new();
    let mut article: String = String::new();
    let mut published_at: String = String::new();
    let mut npub: String = String::new();
    let mut author_name: String = String::new();
    let mut author_image: String = String::new();

    let article_data_signal: Signal<Option<StoryData>> = use_signal(|| None);
    //let tag_data_signal: Signal<Option<TagsInfo>> = use_signal(|| None);


    let key = format!("story-teller_note_{}", event_id);
    //info!("key: {}", key);
    let event_id = EventId::from_str(&note_id).expect("Invalid note_id format");

    // ดึงข้อมูลจาก SessionStorage และเก็บในตัวแปร
    let story_string = SessionStorage::get(&key);

    // เช็คว่ามีข้อมูลหรือไม่
    if let Some(story_string) = story_string {
        //info!("Story found: {}", story_string);
        let mut article_data_signal = article_data_signal.clone();
        use_future( move || {
        let value = story_string.clone();
            async move {
                if let Ok(story) = serde_json::from_str::<StoryCardProps>(&value) {
                    let story_data = StoryData {
                        note_id: Some(story.note_id.clone()),
                        image: Some(story.image.clone()),
                        title: Some(story.title.clone()),
                        summary: Some(story.summary.clone()),
                        article: Some(story.article.clone()),
                        published_at: Some(story.published_at.clone()),
                        npub: Some(story.npub.clone()),
                        author_name: Some(story.author_name.clone()),
                        author_image: Some(story.author_image.clone()),
                    };
                    article_data_signal.set(Some(story_data));
                }
            }
        });

    } else {
        //info!("No story found, fetching from the Relay.");

        // ใช้ use_future เพื่อดึงข้อมูลจาก Relay
        let mut article_data_signal = article_data_signal.clone();
        //let mut tags_list_signal = tag_data_signal.clone();
        let event_id = event_id.clone();
        use_future({
            move || async move {
                let client = NostrClient::setup_and_connect()
                    .await.expect("Failed to setup client");

                let filter = Filter::new()
                    .id(event_id)
                    .kind(Kind::LongFormTextNote);

                let events = client
                    .get_events_of(
                        vec![filter],
                        EventSource::relays(Some(Duration::from_secs(10)))
                    )
                    .await;

                if let Ok(events) = events {
                    if let Some(event) = events.get(0) {
                        let metadata_filter = Filter::new().author(event.pubkey).kind(Kind::Metadata);
                        let metadata_events = client
                            .get_events_of(
                                vec![metadata_filter],
                                EventSource::relays(Some(Duration::from_secs(10)))
                            )
                            .await;

                        //let npub = PublicKey::to_bech32(&event.pubkey);
                        let mut author_name = None;
                        let mut author_image = None;

                        if let Ok(metadata_events) = metadata_events {
                            let mut ts = 0u64;
                            for metadata_event in metadata_events {
                                let user_metadata: Metadata = serde_json::from_str::<Metadata>(&metadata_event.content).unwrap();
                                if !metadata_event.is_expired() && ts < metadata_event.created_at.as_u64() {
                                    author_name = user_metadata.name.clone();
                                    author_image = user_metadata.picture.clone();
                                }
                                ts = metadata_event.created_at.as_u64();
                            }
                        }

                        if author_image.is_some() {
                            let result = check_image(author_image.clone().unwrap().as_str()).await;
                            if !result {
                                let pk = event.pubkey.to_hex();
                                let image_proxy = format!("https://media.nostr.band/thumbs/{}/{}-picture-64", &pk[60..], pk);
                                let result = check_image(&image_proxy).await;
                                author_image = if result { Some(image_proxy) } else { Some(_IMG.to_string()) };
                            }
                        } else {
                            author_image = Some(_IMG.to_string());
                        }

                        let article = extract_tags(event.clone(), author_name, author_image).await;
                        //let tags = process_tags(&event);

                        article_data_signal.set(Some(article));
                        //tags_list_signal.set(Some(tags));
                    }
                }

            }
        });
    }


    if let Some(story_data) = article_data_signal.read().clone() {
        image = story_data.image.clone().unwrap_or_default();
        title = story_data.title.clone().unwrap_or_default();
        summary = story_data.summary.clone().unwrap_or_default();
        article = story_data.article.clone().unwrap_or_default();
        published_at = story_data.published_at.clone().unwrap_or_default();
        npub = story_data.npub.clone().unwrap_or_default();
        author_name = story_data.author_name.clone().unwrap_or_default();
        author_image = story_data.author_image.clone().unwrap_or_default();
    }

    // if let Some(tag_data) = tag_data_signal.read().clone() {
    //     info!("Tags List: {:#?}", tag_data);
    // }

    rsx! {
        style { {STYLE} }
        NavigationBar {}
        SearchBar {}
        div { class: "container",

            if article_data_signal.read().is_none() {
                EllipsisLoading {}
            } else {
                div { class: "control-box",
                    div { class: "col-lg-8 col-sm-8",
                        Article {
                            image: image,
                            title: title,
                            summary: summary,
                            content: article,
                            published_at: published_at
                        }
                    }
                    div { class: "col-lg-4 col-sm-4",
                        ArticleAuthor {
                            npub: npub,
                            author_name: author_name,
                            author_image: author_image
                        }
                    }
                }
            }
        }
    }
}
