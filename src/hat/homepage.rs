use crate::consts::{MeetupEvent, MEETUP_EVENTS};

const INTRO_TEXT: &str = r#"
    Somos una comunidad que busca crear espacios públicos para discutir y aprender sobre la red Nostr, 
    un protocolo de comunicación descentralizado, seguro y abierto.
"#;
pub async fn homepage() -> HomepageTemplate {
    HomepageTemplate {}
}

#[derive(askama::Template)]
#[template(path = "home.html")]
pub struct HomepageTemplate;

impl HomepageTemplate {
    fn event_list() -> Vec<MeetupEvent> {
        let mut list = MEETUP_EVENTS.to_vec();
        list.reverse();
        list
    }
    fn intro_text() -> &'static str {
        INTRO_TEXT
    }
}
