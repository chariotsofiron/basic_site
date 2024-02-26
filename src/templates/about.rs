use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct Component<'a> {
    name: &'a str,
}

pub fn build(name: &str) -> String {
    Component { name }.render().unwrap()
}
