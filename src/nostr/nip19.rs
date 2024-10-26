use nostr_sdk::{
    ToBech32,
    FromBech32,
    EventId
};
use nostr_sdk::prelude::{Nip19, Nip19Event};

pub struct Nip19Tool;

impl Nip19Tool {

    pub fn id_encode(event_id_hex: String) -> String {

        let event_id = EventId::from_hex(event_id_hex).unwrap();
        // let nip19_event = Nip19Event::new(event_id, vec!["relay1".to_string(), "relay2".to_string()]);
        let nip19_event = Nip19Event::new(event_id, Vec::<String>::new());
        let note_id = Nip19::Event(nip19_event).to_bech32().unwrap();

        note_id
    }

    pub fn id_decode(note_id: String) -> String {
        let nip19 = Nip19::from_bech32(&note_id).unwrap();
        match nip19 {
            Nip19::Event(event) => event.event_id.to_hex(),
            _ => String::from("Invalid Nip19 type"),
        }
    }

}
