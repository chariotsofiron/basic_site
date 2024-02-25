use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Component;

pub fn build() -> String {
    Component {}.render().unwrap()
}
