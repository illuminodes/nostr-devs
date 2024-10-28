pub const TCP_ADDRESS: &str = "0.0.0.0:4200";
pub const GITHUB_LINK: &str = "https://github.com/illuminodes/nostr-devs";
pub const X_LINK: &str = "https://twitter.com/illuminodes";
pub const PRIMAL_LINK: &str =
    "https://primal.net/p/npub1dmnzphvk097ahcpecwfeml08xw8sg2cj4vux55m5xalqtzz9t78q6k3kv6";

pub const MEETUP_LINK: &str = "https://www.eventbrite.co/e/nostr-devs-reunion-el-salvador-aprender-y-conocer-y-desarrollar-tickets-1057503193519?aff=ebdssbdestsearch";
pub const MEETUP_EVENTS: &[MeetupEvent] = &[
    MeetupEvent {
        name: "Meetup Inaugural",
        date: "2024-04-24",
        link: "event/0",
    },
    MeetupEvent {
        name: "Meetup #2",
        date: "2024-05-29",
        link: "event/1",
    },
    MeetupEvent {
        name: "Meetup #3",
        date: "2024-06-26",
        link: "event/2",
    },
    MeetupEvent {
        name: "Meetup #4",
        date: "2024-07-31",
        link: "event/3",
    },
    MeetupEvent {
        name: "Meetup #5",
        date: "2024-08-28",
        link: "event/4",
    },
    MeetupEvent {
        name: "Meetup #6",
        date: "2024-09-25",
        link: "event/5",
    },
    MeetupEvent {
        name: "Meetup #7",
        date: "2024-10-30",
        link: "event/6",
    },
];
pub const MEETUP_DETAILS: &[MeetupDetails] = &[
    MeetupDetails {
        topics: [
            Some(("Introduccion a Nostr", "https://nips.nostr.com/")),
            Some((
                "Descentralizacion, Censura, Comodidad",
                "https://x.com/jack/status/1666076985242836993",
            )),
            Some((
                "Gossip Model vs Distribucion Masiva",
                "https://mikedilger.com/gossip-model/",
            )),
            Some(("Fondos Open-Source", "https://opensats.org/funds/nostr")),
            Some((
                "ONOSENDAI/Protocolo CyberSpace",
                "https://github.com/arkin0x/ONOSENDAI?tab=readme-ov-file",
            )),
            None,
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some((
                "Puede Nostr reemplazar a Telegram y Signal?",
                "https://x.com/matthew_d_green/status/1789687898863792453?t=bSn_mQqmMNjWr_JGJabSRA",
            )),
            Some((
                    "La NSA se une a Nostr", 
                    "https://njump.me/nevent1qqsvyh0e0a49gnesf3yek46fg5efzdv5tvevrmuvs4v696fsjmckjpqzyzslgkdxahcay09sp0fcr09r76zcfz7xryg808nph93uflkpayzfjhtuhq0"
            )),
            Some(("Incentivos Para Correr un Relay", "https://yakihonne.com/article/_@yakihonne.com/OikGnwP-WH9AGVC9HYqdo")),
            Some(("Lanzamiento de Rusty CRIB", "https://crib.illuminodes.com/")),
            Some(("Taller de Programacion en Vivo", "https://github.com/supertestnet/nostr-workshop-demo")),
            None,
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some(("TIDAL integrea acceso con Nostr.", "https://developer.tidal.com/blog/tidal-embeds-the-other-stuff-transmitted-by-nostr")),
            Some(("Ditto, comunidades soberanas?", "https://soapbox.pub/blog/announcing-ditto/")),
            Some(("Fiatjaf desanonimizado!", "https://www.businessinsider.com/jack-dorsey-fiatjaf-nostr-donation-2024-6?op=1")),
            Some(("Demostracion de Rusty CRIB", "https://crib.illuminodes.com/")),
            None,
            None,
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some(("Soporte para desarrolladores de Nostr", "https://opensats.org/blog/fiatjaf-receives-lts-grant")),
            Some(("Nostr Web Services, una alternativa a servicios IP", "https://www.nobsbitcoin.com/introducing-nostr-web-services-nws/")),
            Some(("Derivacion de Llaves de Nostr", "https://yakihonne.com/notes/nevent1qgsth7fr42fyvpjl3rzqclvm7cwves8l8l8lqedgevhlfnamvgyg78sqyr6h8vkahes9h3kxd9ld22v9qmzyu9p9eza0skutcnk4qrg9d2l3gt7h2dv")),
            Some(("YakiHonne - Red Social Incentivada", "https://yakihonne.com/flash-news/nevent1qgszpxr0hql8whvk6xyv5hya7yxwd4snur4hu4mg5rctz2ehekkzrvcpr3mhxue69uhkummnw3ez6vp39eukz6mfdphkumn99e3k7mgpr3mhxue69uhkummnw3ez6vpj9eukz6mfdphkumn99e3k7mgpremhxue69uhkummnw3ez6vpj9ejx7unpveskxar0wfujummjvuq3gamnwvaz7tmjv4kxz7fwv3sk6atn9e5k7qpqugnsmzkjrd4kdj28znq6dq7j9rdjchflm2yh3xsj7arxjtept7gs45m9wu")),
            Some(("Lanzamiento Oficial de illuminodes", "https://medium.com/@illuminodes/hospitals-commence-world-first-nostr-healthcare-implementation-in-el-salvador-e847215ee0b2")),
            None,
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some(("Principios de Diseño con Nostr", "https://nostrdesign.org/docs/intro/")),
            Some(("Fountain integra Nostr a su app", "https://x.com/fountain_app/status/1826293931480437194")),
            Some(("Bostr inicia prueba beta", "https://njump.me/nevent1qqswjjgtdwxrndzrh4czn38c4ez6lh4vn7cazg2tf0q44ew3ja9gg8qpr4mhxue69uhkummnw3ez6ur4vgh8wetvd3hhyer9wghxuet5qy28wumn8ghj7un9d3shjtnyv9kh2uewd9hsygqw33q7h9rx2uvgafhj4tpkcf0rj0llf4q5n2pk0y3q6ejetlc04gtc2085")),
            Some(("Nostr Safebox, una wallet virtual?", "https://tim-bouma.npub.pro/post/note10cp79wmxqhv9grhg5r0qkvnlaakt6pnky69mlgh2s5tt42q2pq8ss9y2qe/")),
            Some(("Servicios Ilegales de Cryptologia", "https://x.com/wikileaks/status/1828151621651447908")),
            Some(("Sexta ronda de donaciones OpenSats", "https://www.nobsbitcoin.com/opensats-announced-sixth-wave-of-nostr-grants/")),
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some(("Descubre nuevos relevos", "https://relays.xport.top/")),
            Some(("Relevos WoT - Redes de Confianza", "https://github.com/bitvora/wot-relay/tree/master")),
            Some(("Costos de escalar un relevo", "https://njump.me/nevent1qvzqqqqqqypzqvhpsfmr23gwhv795lgjc8uw0v44z3pe4sg2vlh08k0an3wx3cj9qqs0dcf7243utuweq7fjxlezcrs282znkr73r2fhg0c6sqakeas0hmskyekr4")),
            Some(("Controversia con NIP-44", "https://github.com/paulmillr/nip44/issues/17")),
            Some(("Minions Stack", "https://github.com/42Pupusas/minion-stack")),
            None,
            None,
            None,
        ],
    },
    MeetupDetails {
        topics: [
            Some(("Nostr + RSS", "https://github.com/plantimals/rsslay")),
            Some(("Wavlake, musica en protocols abiertos", "https://wavlake.notion.site/Wavlake-Guide-e560557ae4ba4e06ac20c06fb33556a3")),
            Some(("Lanzamiento de Pubky", "https://medium.com/@synonym_to/pubky-launch-260f36ba8fe3")),
            Some(("Relevos usando Tor", "https://njump.me/nevent1qqsd4vcrymtav7jt8jmyykv7utevq004gu4datyusvn8a7p47yhf7lsprpmhxue69uhkummnw3ezuendwsh8w6t69e3xj730qgs04xzt6ldm9qhs0ctw0t58kf4z57umjzmjg6jywu0seadwtqqc75srqsqqqqqpg48y67")),
            Some(("Un Pais construido sobre Open Source", "https://njump.me/nevent1qqs2axt6z58ta763qr8s2rrw2k2ymfzw8ayqqt5p5sy3pyux73trvpgprpmhxue69uhkummnw3ezuendwsh8w6t69e3xj730qgsrums8xku296t03nmx8ajdqjaee65nrt7v2l65ylp4hd59n62u3gsrqsqqqqqpqch0u7")),
            None,
            None,
            None,
        ],
    },
];

#[derive(Clone)]
pub struct MeetupEvent {
    pub name: &'static str,
    pub date: &'static str,
    pub link: &'static str,
}

#[derive(Clone)]
pub struct MeetupDetails {
    pub topics: [Option<(&'static str, &'static str)>; 8],
}
