use askama::Template;

#[derive(Template)]
#[template(path = "signup_form.html")]
pub struct Component<'a> {
    username: &'a str,
    username_message: &'a str,
    password_message: &'a str,
}

pub fn build_with_error_message(
    username: &str,
    username_message: &str,
    password_message: &str,
) -> String {
    Component {
        username,
        username_message,
        password_message,
    }
    .render()
    .unwrap()
}
