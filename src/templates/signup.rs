use askama::Template;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct Component<'a> {
    username: &'a str,
    message: &'a str,
}

pub fn build() -> String {
    Component {
        username: "",
        message: "",
    }
    .render()
    .unwrap()
}

pub fn build_with_error_message(username: &str, message: &str) -> String {
    Component { username, message }.render().unwrap()
}
