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
  rocket::build().mount("/", rocket::routes![index, audios, css, logo, refresh_check])
}

#[rocket::get("/")]
fn index() -> Html {
  let teachings = Teaching::load_most_recent(5);
  let links = vec![
    component::Link::new(
      "Friends Library",
      "https://www.friendslibrary.com",
      "Market Street Fellowship has been so impacted by the writings of the early (not modern!) Society of Friends (1650-1800), that we put together a website where hundreds of their books are available for free, in a variety of different text and audio formats."
    ),
    component::Link::new(
      "Gertrude",
      "https://gertrude.app",
      "Jared Henderson and Miciah Henderson have spent years building what we believe to be the safest parental control software in existence for Apple computers. There is also a corresponding Gertrude iPhone app that plugs some of the holes that Apple's Screen Time feature misses."
    ),
    component::Link::new(
      "Ancient Path",
      "https://hender.blog",
      "More teachings and posts (in text and audio) from Jason Henderson on a variety of different subjects."
    ),
  ];
  
  let html = include_str!("assets/index.en.html")
    .replace(
      "{%audios%}",
      &teachings
        .into_iter()
        .map(|t| component::Audio { teaching: t }.html())
        .collect::<Vec<_>>()
        .join("\n"),
    )
    .replace(
      "{%links%}",
      &links
        .into_iter()
        .map(|l| l.html())
        .collect::<Vec<_>>()
        .join("\n"),
    );
  Html::new(&html)
}

#[rocket::get("/audios")]
fn audios() -> Html {
  let teachings = Teaching::load_all();
  let html = include_str!("assets/audios.html").replace(
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

#[rocket::get("/check-9e328da2")]
fn refresh_check() -> rocket::response::status::NoContent {
  rocket::response::status::NoContent
}

// helpers

mod internal {
  pub use crate::html::*;
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::NaiveDateTime;
}
