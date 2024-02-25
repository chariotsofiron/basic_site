use askama::Template;

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Component {}

pub fn build() -> String {
    Component {}.render().unwrap()
}
