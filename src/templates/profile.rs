use askama::Template;

use crate::models::session::Session;

#[derive(Template)]
#[template(path = "profile.html")]
pub struct Component<'a> {
    username: &'a str,
    sessions: &'a [Session],
}

pub fn build(username: &str, sessions: &[Session]) -> String {
    Component { username, sessions }.render().unwrap()
}
