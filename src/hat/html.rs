use askama::Template;

use crate::consts::{MeetupDetails, MeetupEvent, INTRO_TEXT, MEETUP_EVENTS};

#[derive(Template)]
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

#[derive(Template)]
#[template(path = "event.html")]
pub struct EventTemplate {
    pub meetup: MeetupEvent,
    pub topics: MeetupDetails,
}
