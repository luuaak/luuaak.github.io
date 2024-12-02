#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use regex::Regex;
use crate::styles::markdown_style::STYLE;
use pulldown_cmark::{html, Options, Parser, Event, Tag, BlockQuoteKind, TagEnd};

/*
pub fn markdown_to_html(markdown_input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options)
        .map(|event| match event {
        Event::Start(Tag::BlockQuote(Some(BlockQuoteKind::Note))) => {
            if markdown_input.contains("[!IMPORTANT]") {
                Event::Html("<blockquote class=\"important\">".into())
            } else if markdown_input.contains("[!WARNING]") {
                Event::Html("<blockquote class=\"warning\">".into())
            } else {
                Event::Html("<blockquote>".into())
            }
        }
        Event::End(TagEnd::BlockQuote(_)) => { // ใช้ Tag::BlockQuote โดยไม่มี Some
            Event::Html("</blockquote>".into())
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
*/

pub fn markdown_to_html(markdown_input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}



pub fn filter_text(markdown_input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);

    let mut filtered_text = String::new();

    for event in parser {
        match event {
            pulldown_cmark::Event::Text(text) => {
                filtered_text.push_str(&text);
            }
            pulldown_cmark::Event::HardBreak | pulldown_cmark::Event::SoftBreak => {
                filtered_text.push_str("\n\n");
            }
            _ => {}
        }
    }

    // ฟิลเตอร์ลิงก์ในข้อความที่ได้จาก Markdown
    let filtered_with_links = filter_links(filtered_text);

    filtered_with_links
}


pub fn filter_links(input: String) -> String {
    let re = Regex::new(r"https?://[^\s]+").unwrap();
    re.replace_all(&input, "\n").to_string()
}


#[component]
pub fn Markdown(text: String) -> Element {

    let content = markdown_to_html(&*text);
    info!("HTML content: {}", content);

    rsx! {
        style { {STYLE} }
        article { class: "markdown-field-body",
            dangerous_inner_html: "{content}"
        }
    }
}