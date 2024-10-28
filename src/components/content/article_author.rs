#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::styles::article_author_style::STYLE;
use crate::pages::router::Route;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_4.jpg"));
const _COMMENT: &str = manganis::mg!(file("src/assets/comment.svg"));
const _SHARE: &str = manganis::mg!(file("src/assets/Share.svg"));
const _MARKING: &str = manganis::mg!(file("src/assets/marking.svg"));
const _FOLLOW: &str = manganis::mg!(file("src/assets/plus.svg"));

#[derive(PartialEq, Props, Clone)]
pub struct AuthorInfoProps {
    npub: String,
    author_name: String,
    author_image: String,
}

#[component]
pub fn ArticleAuthor(props: AuthorInfoProps) -> Element {
    let navigator: Navigator = use_navigator();
    let npub = props.npub.clone();

    let handle_share = move |_| {
        info!("Share clicked!");
    };

    let handle_marking = move |_| {
        info!("Marking clicked!");
    };

    let handle_comment = move |_| {
        info!("Comment clicked!");
    };

    let handle_open_profile = move |_| {
        info!("Open profile clicked!");

        navigator.push(
            Route::ProfilePage {
                npub: npub.clone(),
            },
        );

    };

    rsx! {
        style { "{STYLE}" }
        div { class: "article-author-box",

            div { class: "field-button-pt",
                div { class: "field-button-util",
                    button { class: "article-button-item",
                        r#type: "button",
                        onclick: handle_share,
                        img { src: "{_SHARE}", alt: "Play Icon" }
                        span { class: "article-lable-item", "Share" }
                    }

                    button { class: "article-button-item",
                        r#type: "button",
                        onclick: handle_marking,
                        img { src: "{_MARKING}", alt: "Play Icon" }
                        span { class: "article-lable-item", "Marking" }
                    }

                    button { class: "article-button-item",
                        r#type: "button",
                        onclick: handle_comment,
                        img { src: "{_COMMENT}", alt: "Play Icon" }
                        span { class: "article-lable-item", "Comment" }
                    }
                }
            }

            div { class: "author-info",
                div { class: "article-author-bar",
                    div { id: "article-author",
                        img {
                            class: "article-profile-image",
                            src: "{props.author_image}",
                            alt: "Profile image",
                            onclick: handle_open_profile.clone(),
                        }
                        div { class: "article-author-details",
                            h4 { class: "article-author-name",
                                onclick: handle_open_profile,
                                "{props.author_name}"
                            }
                            button { class: "article-button-follow",
                                r#type: "button",
                                img { src: "{_FOLLOW}", alt: "Follow Icon" }
                                span { class: "article-lable-item", "Follow" }
                            }
                        }
                    }
                    p { class: "article-count", "32 Note" }
                }
            }

            div { class: "tags-info" }
        }
    }
}
