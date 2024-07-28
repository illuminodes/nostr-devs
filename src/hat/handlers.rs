use axum::extract::Path;

use crate::{html::{HomepageTemplate, EventTemplate}, consts::{MEETUP_EVENTS, MEETUP_DETAILS}};

pub async fn homepage() -> HomepageTemplate {
    HomepageTemplate {}
}

pub async fn events(
    Path(route): Path<String>,
    ) ->  EventTemplate {
    let index = route.parse::<usize>().unwrap();
    let meetup = MEETUP_EVENTS[index].clone();
    let topics = MEETUP_DETAILS[index].clone();
    EventTemplate { meetup, topics }
}


