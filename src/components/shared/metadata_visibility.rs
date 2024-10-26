use dioxus::prelude::*;
use nostr_sdk::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMetadata {
    pub name: String,
    pub nip05: String,
    pub about: String,
    pub lud16: String,
    pub display_name: String,
    pub picture: String,
    pub banner: String,
    pub website: String,
}

#[derive(Clone, PartialEq)]
pub struct SharedMetadataVisibility {
    pub user_metadata: Signal<UserMetadata>,
    pub raw_metadata: Signal<String>,
    pub metadata: Signal<Option<Event>>,
}

impl SharedMetadataVisibility {

    pub fn new() -> Self {
        let raw_metadata = use_signal(|| String::new());
        let metadata = use_signal(|| None);
        let user_metadata = use_signal(|| UserMetadata {
            name: String::new(),
            nip05: String::new(),
            about: String::new(),
            lud16: String::new(),
            display_name: String::new(),
            picture: String::new(),
            banner: String::new(),
            website: String::new(),
        });

        Self {
            raw_metadata,
            metadata,
            user_metadata,
        }
    }


}
