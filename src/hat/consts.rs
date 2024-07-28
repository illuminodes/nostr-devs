#[cfg(debug_assertions)]
pub const CSS_STYLESHEET: &str = "../styles/output.css";

#[cfg(not(debug_assertions))]
pub const CSS_STYLESHEET: &str = "../styles/prod.css";

pub const TCP_ADDRESS: &str = "0.0.0.0:4200";
pub const INTRO_TEXT: &str = r#"
    Somos una comunidad que busca crear espacios públicos para discutir y aprender sobre la red Nostr, 
    un protocolo de comunicación descentralizado, seguro y abierto.
"#;
pub const GITHUB_LINK: &str = "https://github.com/illuminodes/nostr-devs";
pub const X_LINK: &str = "https://twitter.com/illuminodes";
pub const PRIMAL_LINK: &str =
    "https://primal.net/p/npub1dmnzphvk097ahcpecwfeml08xw8sg2cj4vux55m5xalqtzz9t78q6k3kv6";

pub const MEETUP_LINK: &str = "https://www.eventbrite.co/e/nostr-dev-reunion-el-salvador-aprender-y-conocer-y-desarrollar-tickets-962480684637?utm-campaign=social&utm-content=attendeeshare&utm-medium=discovery&utm-term=listing&utm-source=cp&aff=ebdsshcopyurl";
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
    // MeetupEvent {
    //     name: "Meetup #5",
    //     date: "2024-08-28",
    //     link: "event/4",
    // },
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
    // MeetupDetails {
    //     topics: [
    //         Some(("Nostr y la Educacion", "https://www.nobsbitcoin.com/education/")),
    //         Some(("Nostr y la Salud", "https://www.nobsbitcoin.com/health/")),
    //         Some(("Nostr y la Economia", "https://www.nobsbitcoin.com/economy/")),
    //         Some(("Nostr y la Politica", "https://www.nobsbitcoin.com/politics/")),
    //         Some(("Nostr y la Cultura", "https://www.nobsbitcoin.com/culture/")),
    //         Some(("Nostr y la Tecnologia", "https://www.nobsbitcoin.com/technology/")),
    //         Some(("Nostr y la Ciencia", "https://www.nobsbitcoin.com/science/")),
    //         Some(("Nostr y la Filosofia", "https://www.nobsbitcoin.com/philosophy/")),
    //     ],
    // },
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
