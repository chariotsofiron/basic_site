use askama::Template;

pub mod home;
pub mod navbar;

/// Main base template.
#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate<'a> {
    navbar: &'a str,
    content: &'a str,
}

pub fn build(navbar: &str, content: &str) -> String {
    MainTemplate { navbar, content }.render().unwrap()
}
