use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Component<'a> {
    name: &'a str,
}

pub fn build(name: &str) -> String {
    Component { name }.render().unwrap()
}
