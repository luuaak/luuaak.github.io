#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use web_sys::window;
use crate::styles::checkbox_style::STYLE;

const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));
const _ICON_ARROW_DOWN: &str = manganis::mg!(file("src/assets/chevron-down.svg"));
const _ICON_ARROW_UP: &str = manganis::mg!(file("src/assets/chevron-up.svg"));

#[component]
pub fn CheckBox() -> Element {

    let mut key_word_list: Signal<Vec<String>> = use_signal(Vec::<String>::new);
    let mut is_dropdown: Signal<bool> = use_signal(|| false);
    let mut arrow_flipped: Signal<bool> = use_signal(|| false);
    let screen_size: Signal<(u32, u32)> = use_signal(|| (0u32, 0u32));

    // ดึงขนาดหน้าจอเมื่อ component mount
    use_effect({
        let mut screen_size = screen_size.clone();
        move || {
            let window = window().unwrap();
            let width: u32 = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height: u32 = window.inner_height().unwrap().as_f64().unwrap() as u32;
            screen_size.set((width, height));
            //info!("Screen size: {}x{}", width, height);
            ()
        }
    });

    // ตรวจสอบขนาดหน้าจอและตั้งค่า dropdown
    use_effect({
        let mut is_dropdown = is_dropdown.clone();
        let screen_size = screen_size.clone();
        move || {
            let (width, _) = *screen_size.read();
            //info!("Current Width size: {width}");
            if width >= 640 {
                is_dropdown.set(true);
            } else {
                is_dropdown.set(false);
            }
            ()
        }
    });


    let (width, height) = screen_size();
    info!("Current key word list: {:?}", key_word_list.read());
    info!("Current Screen size: {width} x {height}");
    info!("Current dropdown state: {is_dropdown}");

    // ฟังก์ชันสำหรับจัดการการคลิก checkbox
    let mut handle_checkbox_click = move |label: &str| {
        let mut list = key_word_list.write();
        if list.contains(&label.to_string()) {
            list.retain(| x: &String | x != label);
        } else {
            list.push(label.to_string());
        }
        //info!("{label} checkbox clicked");
    };

    // ฟังก์ชันสำหรับจัดการการคลิกปุ่ม dropdown
    let handle_button_click = move |_| {
        let mut dropdown = is_dropdown.write();
        *dropdown = !*dropdown;

        // สลับสถานะ arrow_flipped
        let mut flipped = arrow_flipped.write();
        *flipped = !*flipped;
    };

    rsx! {
        style { {STYLE} }

        div { class: "checkbox-container",
            div { class: "checkbox-sidebar",

                div {
                    button {
                        class: "icon-container",
                        onclick: handle_button_click,
                        img { src: "{_ICON_FILTER}" }
                        img {
                            class: "col-lg-hidden col-sm-hidden",
                            src: format!("{}", if *arrow_flipped.read() {
                                _ICON_ARROW_UP
                            } else {
                                _ICON_ARROW_DOWN
                            })
                        }
                    }
                }

                if *is_dropdown.read() {
                    div { class: "checkbox-pt",
                        h3 { class: "header", "General" }
                        ul { class: "detail ",
                            // Checkbox สำหรับตัวเลือกต่างๆ
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "chill",
                                    onclick: move |_| handle_checkbox_click("Chill")
                                }
                                label { class: "filter-label", "Chill" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "dramatic",
                                    onclick: move |_| handle_checkbox_click("Dramatic")
                                }
                                label { class: "filter-label", "Dramatic" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "happy",
                                    onclick: move |_| handle_checkbox_click("Happy")
                                }
                                label { class: "filter-label", "Happy" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "sad",
                                    onclick: move |_| handle_checkbox_click("Sad")
                                }
                                label { class: "filter-label", "Sad" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "hopeful",
                                    onclick: move |_| handle_checkbox_click("Hopeful")
                                }
                                label { class: "filter-label", "Hopeful" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "fantasy",
                                    onclick: move |_| handle_checkbox_click("Fantasy")
                                }
                                label { class: "filter-label", "Fantasy" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "romantic",
                                    onclick: move |_| handle_checkbox_click("Romantic")
                                }
                                label { class: "filter-label", "Romantic" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "relaxing",
                                    onclick: move |_| handle_checkbox_click("Relaxing")
                                }
                                label { class: "filter-label", "Relaxing" }
                            }
                        }
                    }

                } // * จบเงื่อนไข


            }
        }
    }
}
