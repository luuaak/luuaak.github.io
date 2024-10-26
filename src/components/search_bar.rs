#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::styles::search_bar_style::STYLE;

#[component]
pub fn SearchBar() -> Element {

    // ใช้ use_signal เพื่อเก็บข้อความที่ผู้ใช้ป้อนเข้ามาในช่องค้นหา
    let mut search_value = use_signal(String::new);

    rsx! {
        style { {STYLE} }
        div { id: "search-pt",
            div { class: "search-bar",

                div { class: "input-container",

                    // ช่อง input สำหรับให้ผู้ใช้ป้อนคำค้นหา
                    input {
                        class: "search-box",
                        r#type: "text",
                        placeholder: "Search",
                        // เมื่อมีการพิมพ์ (input) ให้เก็บค่าลงใน search_value
                        oninput: move |event| {
                            // เขียนค่าจากการพิมพ์ของผู้ใช้ลงใน search_value
                            search_value.set(event.value().clone());
                            // Debug ข้อความที่ผู้ใช้ป้อน
                            info!("Search input: {}", event.value());
                        },
                        // ตั้งค่า value ของ input ให้เป็นค่าปัจจุบันใน search_value
                        value: "{search_value.read()}"
                    }

                    // Search Icon ในช่องใส่ข้อมูล
                    svg { class: "searchIcon",
                        view_box: "0 0 512 512",
                        path {
                            d: "M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z",
                        }
                    }

                }
            }
        }

        //h3 { style: "color: white", "Current search value: {search_value}" }

    }
}