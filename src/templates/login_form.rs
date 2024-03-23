use askama::Template;

#[derive(Template)]
#[template(path = "login_form.html")]
pub struct Component<'a> {
    message: &'a str,
}

pub fn build(message: &str) -> String {
    Component { message }.render().unwrap()
}
