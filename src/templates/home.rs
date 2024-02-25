use askama::Template;

use super::navbar;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Component;

pub fn build() -> String {
    super::build(&navbar::build(), &Component {}.render().unwrap())
}
