#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use internal::*;
use rocket::http::ContentType;

pub mod component;
pub mod html;
pub mod teaching;
pub mod time;

#[rocket::launch]
fn rocket() -> _ {
  rocket::build().mount("/", rocket::routes![index, css, logo, check])
}

#[rocket::get("/")]
fn index() -> Html {
  let teachings = Teaching::load_most_recent(5);
  let template = include_str!("assets/index.en.html");
  let html = template.replace(
    "{%audios%}",
    &teachings
      .into_iter()
      .map(|t| component::Audio { teaching: t }.html())
      .collect::<Vec<_>>()
      .join("\n"),
  );
  Html::new(&html)
}

#[rocket::get("/styles.css")]
fn css() -> (ContentType, &'static str) {
  (ContentType::CSS, include_str!("assets/output.css"))
}

#[rocket::get("/msf-logo.webp")]
fn logo() -> (ContentType, &'static [u8]) {
  (
    ContentType::new("image", "webp"),
    include_bytes!("assets/msf-logo.webp"),
  )
}

#[rocket::get("/check")]
fn check() -> &'static str {
  "OK"
}

mod internal {
  pub use crate::html::*;
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::NaiveDateTime;
}
