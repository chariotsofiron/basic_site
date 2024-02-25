use askama::Template;

pub mod home;
pub mod login;
pub mod navbar;
pub mod signup;

/// Base template.
#[derive(Template)]
#[template(path = "base.html")]
pub struct Base<'a> {
    navbar: &'a str,
    content: &'a str,
}

pub fn base(navbar: &str, content: &str) -> String {
    Base { navbar, content }.render().unwrap()
}
