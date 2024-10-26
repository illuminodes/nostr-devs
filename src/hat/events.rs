use crate::consts::{MeetupDetails, MeetupEvent, MEETUP_DETAILS, MEETUP_EVENTS};

pub async fn event_page(axum::extract::Path(route): axum::extract::Path<String>) -> EventTemplate {
    let index = route.parse::<usize>().unwrap();
    let meetup = MEETUP_EVENTS[index].clone();
    let topics = MEETUP_DETAILS[index].clone();
    EventTemplate { meetup, topics }
}

#[derive(askama::Template)]
#[template(path = "event.html")]
pub struct EventTemplate {
    meetup: MeetupEvent,
    topics: MeetupDetails,
}
