#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use pulldown_cmark::{Parser, Options, html};
use web_sys::{window, SpeechSynthesisUtterance};
use regex::Regex;
use crate::components::content::markdown::{Markdown,filter_text};
use crate::components::story_card::format_unix_to_date;
use crate::styles::article_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));
const _CALENDAR: &str = manganis::mg!(file("src/assets/date.svg"));
const _CATEGORY: &str = manganis::mg!(file("src/assets/category.svg"));
const _COMMENT: &str = manganis::mg!(file("src/assets/comment.svg"));
const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _PLAYING: &str = manganis::mg!(file("src/assets/playing.svg"));

#[derive(PartialEq, Props, Clone)]
pub struct ArticleProps {
    image: String,
    title: String,
    summary: String,
    content: String,
    published_at: String,
}


fn play_sound(text: String, is_playing: bool) {
    let window = window().expect("no global `window` exists");
    let speech_synthesis = window
        .speech_synthesis()
        .expect("Speech synthesis not supported");

    if !is_playing {
        speech_synthesis.cancel();
        return;
    }

    let utterance = SpeechSynthesisUtterance::new_with_text(&text)
        .expect("Unable to create utterance");

    utterance.set_rate(0.9);    // ปรับความเร็วเสียง (rate)
    utterance.set_pitch(0.9);   // โทนเสียง (pitch)
    utterance.set_volume(1.0);  // ความดังเสียง (volume)

    speech_synthesis.speak(&utterance);
}

fn detect_browser() -> String {
    if let Some(navigator) = window().and_then(|win| win.navigator().user_agent().ok()) {
        if navigator.contains("Edg") {
            "Microsoft Edge".to_string()
        } else if navigator.contains("Chrome") && !navigator.contains("Chromium") {
            "Google Chrome".to_string()
        } else if navigator.contains("Firefox") {
            "Mozilla Firefox".to_string()
        } else if navigator.contains("Safari") && !navigator.contains("Chrome") {
            "Safari".to_string()
        } else if navigator.contains("Opera") || navigator.contains("OPR") {
            "Opera".to_string()
        } else {
            "Unknown Browser".to_string()
        }
    } else {
        "Unable to detect browser".to_string()
    }
}



#[component]
pub fn Article(props: ArticleProps) -> Element {

    let browser_name = detect_browser();
    //info!("Browser detected: {}", browser_name);

    // สถานะสำหรับจัดการการแสดง play icon
    let mut play_signal = use_signal(|| false);

    let formatted_date = format_unix_to_date(&props.published_at);
    //let content = markdown_to_html(&props.content);

    // กรองข้อความจากเนื้อหาที่เป็น Markdown
    let filtered_content = filter_text(&props.content);

    // แสดงผลใน console
    info!("Filtered content: {}", filtered_content);
    info!("Raw content: {}", &props.content);

    // ฟังก์ชันจัดการการกดปุ่ม play
    let handle_play = move |_| {
        let mut is_playing = play_signal.write();
        *is_playing = !*is_playing;
        info!("Play clicked!");

        play_sound(
            filtered_content.clone(),
            *is_playing
        );
    };

    rsx! {
        style { {STYLE} }
        div { class: "article-box",

            div { class: "article-field",
                div { class: "markdown-field-text-title", "{props.title}" }
                div { class: "article-field-image-header",
                    img {
                        class: "field-title-image-box",
                        src: "{props.image}",
                        max_height: "420px"
                    }
                    div { class: "field-pt",
                        div { class: "article-field-icons",

                            // ตรวจสอบเบราว์เซอร์ก่อนแสดงปุ่ม play
                            if browser_name == "Microsoft Edge" {
                                div { class: "field-icon-box", onclick: handle_play,
                                    img {
                                        src: if *play_signal.read() { _PLAYING } else { _PLAY },
                                        alt: "Play Icon"
                                    }
                                    span {
                                        "Play"
                                    }
                                }
                            }

                            div { class: "field-icon-box", img { src: "{_CALENDAR}" }, span { "{formatted_date}" } }
                            div { class: "field-icon-box", img { src: "{_COMMENT}" }, span { "comments : 35" } }
                            div { class: "field-icon-box", img { src: "{_CATEGORY}" }, span { "Category : Sport" } }
                        }
                    }
                }

                Markdown { text: &props.content }

            }
        }
    }
}
