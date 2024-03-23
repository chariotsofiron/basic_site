use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
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
