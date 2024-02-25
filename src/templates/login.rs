use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct Component<'a> {
    message: &'a str,
}

pub fn build() -> String {
    Component { message: "" }.render().unwrap()
}

pub fn build_with_error_message(message: &str) -> String {
    Component { message }.render().unwrap()
}
