use askama::Template;

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Component<'a> {
    username: &'a str,
}

pub fn build() -> String {
    Component { username: "" }.render().unwrap()
}

pub fn build_with_username(username: &str) -> String {
    Component { username }.render().unwrap()
}
