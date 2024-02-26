use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct Component {}

pub fn build() -> String {
    Component {}.render().unwrap()
}
