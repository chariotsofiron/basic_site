use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct Component<'a> {
    username: &'a str,
}

pub fn build(username: &str) -> String {
    Component { username }.render().unwrap()
}
