#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::router::Route;
use crate::styles::screen_config::STYLE;

// use dioxus_logger::tracing::Level;
// use nostr_sdk::{FromBech32, ToBech32};
// use nostr_sdk::prelude::{Client, Event as NostrEvent, EventBuilder, Keys, RelaySendOptions, SecretKey};
// use web_sys::console;

//const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const _CONFIG: &str = manganis::mg!(file("src/main.css"));
const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));

#[component]
pub fn App() -> Element {
    /*
    use_future(|| async move {

        let secret_key = SecretKey::from_bech32(BECH32_SK).expect("Invalid Secret Key");
        let my_keys = Keys::new(secret_key);

        let client = Client::new(&my_keys);

        client.add_relay("wss://relay.notoshi.win").await.expect("Failed to add relay");
        client.add_relay("wss://relay.siamstr.com").await.expect("Failed to add relay");
        client.connect().await;

        // Publish a text note
        let output = client.publish_text_note("นึกแล้วมึงต้องอ่าน", []).await.expect("Failed to publish text note");
        console::log_1(&format!("Event ID: {}", output.id().to_bech32().unwrap()).into());
        console::log_1(&format!("Sent to: {:?}", output.success).into());
        console::log_1(&format!("Not sent to: {:?}", output.failed).into());

        /*
        // Create a text note POW event and broadcast to all connected relays
        let event: NostrEvent = EventBuilder::text_note("POW text note from rust-nostr", [])
            .pow(20)
            .to_event(&my_keys)
            .expect("Failed to create event");
        client.send_event(event).await.expect("Failed to send event");

        // Send multiple events at once (to all relays)
        let mut events: Vec<NostrEvent> = Vec::new();  // ใช้ NostrEvent แทน
        for i in 0..10 {
            events.push(EventBuilder::text_note(format!("Event #{i}"), []).to_event(&my_keys).expect("Failed to create event"));
        }
        let opts = RelaySendOptions::default();
        client.batch_event(events, opts).await.expect("Failed to batch events");

        // Send event to specific relays
        let event: NostrEvent = EventBuilder::text_note("POW 7 นึกแล้วมึงต้องอ่าน", [])
            .pow(7)
            .to_event(&my_keys)
            .expect("Failed to create event");
        client.send_event_to(["wss://relay.notoshi.win", "wss://relay.siamstr.com"], event).await.expect("Failed to send event to specific relays");
        */
    });*/

    rsx! {
        link { rel: "icon", href: _ICON }
        link { rel: "stylesheet", href: _CONFIG }
        style { {STYLE} }
        Router::<Route> { }
    }

}
