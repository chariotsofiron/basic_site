use askama::Template;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct Component<'a> {
    username: &'a str,
    username_message: &'a str,
    password_message: &'a str,
}

pub fn build() -> String {
    Component {
        username: "",
        username_message: "",
        password_message: "",
    }
    .render()
    .unwrap()
}
