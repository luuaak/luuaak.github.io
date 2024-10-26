use nostr_sdk::Client;
use std::error::Error;

pub struct NostrClient {
    pub client: Client,
}

impl NostrClient {
    pub fn new() -> Self {
        let client = Client::default();
        NostrClient { client }
    }

    pub async fn add_relay(&mut self, relay: &str) -> Result<(), Box<dyn Error>> {
        self.client.add_relay(relay).await?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.connect().await;
        Ok(())
    }

    // ฟังก์ชันสำหรับการตัดการเชื่อมต่อ
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.disconnect().await?;
        Ok(())
    }

    // ฟังก์ชันสำหรับการตั้งค่าและเชื่อมต่อกับ relays
    pub async fn setup_and_connect() -> Result<Client, Box<dyn Error>> {
        let mut client = NostrClient::new();
        let relays: Vec<&str> = vec![
            //"ws://localhost:6724",
            // "wss://relay.rushmi0.win",
            "wss://nos.lol",
            "wss://relay.notoshi.win",
            "wss://nostr.mom",
            "wss://relay.snort.social",
            "wss://nostr-01.yakihonne.com",
        ];

        for relay in relays {
            client.add_relay(relay).await?;
        }

        client.connect().await?;
        Ok(client.client)
    }
}
