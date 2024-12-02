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
            "wss://nostr.topeth.info",
            "wss://lightningrelay.com",
            //"wss://nostr-dev.wellorder.net",
            //"wss://nostr.bitcoiner.social",
            //"wss://nostr.easydns.ca",
            // "wss://nostr.roundrockbitcoiners.com",
            // "wss://relay.lexingtonbitcoin.org",
            // "wss://nostr21.com",
            // "wss://nostrue.com",
            // "wss://nostr-verified.wellorder.net",
            // "wss://bitcoiner.social",
            // "wss://relay.farscapian.com",
            // "wss://relay.oldcity-bitcoiners.info",
            // "wss://relay.sovereign-stack.org",
            // "wss://relay.damus.io",
            // "wss://nostr.fmt.wiz.biz",
            // "wss://nostr.ch3n2k.com",
            // "wss://relay.orange-crush.com",
            // "wss://freespeech.casa",
            // "wss://nostr.corebreach.com",
            // "wss://nostr.tools.global.id",
            // "wss://nostr.pjv.me",
            // "wss://nostrelay.yeghro.site",
            // "wss://relay.s3x.social",
            // "wss://nostr.lorentz.is",
            // "wss://private.red.gb.net",
            // "wss://nostr.koning-degraaf.nl",
            // "wss://knostr.neutrine.com",
            // "wss://relay.nostrview.com",
            // "wss://nostr-relay.bitcoin.ninja",
            // "wss://rsslay.ch3n2k.com",
            // "wss://frens.nostr1.com",
            // "wss://purplerelay.com",
            // "wss://relay.nostr.wirednet.jp",
            // "wss://nostr.hifish.org",
            // "wss://ithurtswhenip.ee",
            // "wss://relay.hodl.ar",
            // "wss://nostr.cercatrova.me",
            // "wss://relay1.nostrchat.io",
            // "wss://dev-relay.kube.b-n.space",
            // "wss://us.purplerelay.com",
            // "wss://relay.nostromo.social",
            // "wss://relay.nos.social",
            // "wss://relay.kamp.site",
            // "wss://relay.usefusion.ai",
            // "wss://thecitadel.nostr1.com",
            // "wss://nostr-01.yakihonne.com",
            // "wss://nosflare.plebes.fans",
            // "wss://nostr.stakey.net",
            // "wss://directory.yabu.me",
            // "wss://multiplexer.huszonegy.world",
            // "wss://relay.nostr.net",
            // "wss://relay.minibits.cash",
            // "wss://relays.diggoo.com",
            // "wss://freelay.sovbit.host",
            // "wss://nostr.heliodex.cf",
            // "wss://strfry.chatbett.de",
            // "wss://nostr.gerbils.online",
            // "wss://nostr.0x7e.xyz",
            // "wss://unhostedwallet.com",
            // "wss://relay.zap.store",
            // "wss://nostr2.daedaluslabs.io",
            // "wss://relay.noswhere.com",
            // "wss://relay.varke.eu",
            // "wss://br.purplerelay.com",
            // "wss://relay.nostrcn.com",
            // "wss://nostr.petrkr.net/strfry",
            // "wss://relay.corpum.com",
            // "wss://r314y.0xd43m0n.xyz",
            // "wss://hist.nostr.land",
            // "wss://relay.zhoushen929.com",
            // "wss://relay.livefreebtc.dev",
            // "wss://relay.notoshi.win",
            // "wss://n.ok0.org",
            // "wss://nostr.satstralia.com",
            // "wss://nostr.carroarmato0.be",
            // "wss://strfry.openhoofd.nl",
            // "wss://bostr.nokotaro.work",
            // "wss://bostr.nokotaro.com",
            // "wss://nostr-03.dorafactory.org"

        ];

        for relay in relays {
            client.add_relay(relay).await?;
        }

        client.connect().await?;
        Ok(client.client)
    }
}
