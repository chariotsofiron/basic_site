use askama::Template;

#[derive(Template)]
#[template(path = "profile.html")]
pub struct Component {}

pub fn build() -> String {
    Component {}.render().unwrap()
}
