#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use internal::*;
use rocket::http::ContentType;

pub mod cached;
pub mod component;
pub mod date;
pub mod html;
pub mod img;
pub mod index;
pub mod podcast;
pub mod teaching;
pub mod time;

#[rocket::launch]
fn rocket() -> _ {
  rocket::build().mount(
    "/",
    rocket::routes![
      home,
      audios,
      gathering_details,
      css,
      podcast_xml,
      refresh_check,
      img::logo_webp,
      img::logo_svg,
      img::apple_podcasts,
      img::google_podcasts,
      img::overcast,
      img::spotify,
      img::rss_png,
      img::favicon_ico,
      img::favicon_16x16,
      img::favicon_32x32,
    ],
  )
}

#[rocket::get("/")]
fn home() -> Cached<Html> {
  Cached::new(index::get(), Cache::ONE_HOUR)
}

#[rocket::get("/audios")]
fn audios() -> Cached<Html> {
  let teachings = Teaching::load_all();
  let html = include_str!("assets/html/audios.html")
    .replace("{%head%}", &html::head(Some("Audios")))
    .replace(
      "{%audios%}",
      &teachings
        .into_iter()
        .map(|t| component::Audio { teaching: t }.html())
        .collect::<Vec<_>>()
        .join("\n"),
    );
  Cached::new(Html::new(&html), Cache::ONE_HOUR)
}

#[rocket::get("/gathering-details")]
fn gathering_details() -> Cached<Html> {
  let (year, month, day) = date::current_date_parts();
  let display_year = date::gathering_year_to_display(year, month, day);
  let (thursday, friday, saturday, sunday) = date::gathering_detail_dates(display_year);

  let html = include_str!("assets/html/gathering-details.html")
    .replace("{%head%}", &html::head(Some("Spring Gathering Details")))
    .replace("{%gathering_year%}", &display_year.to_string())
    .replace("{%thursday_day%}", &thursday)
    .replace("{%friday_day%}", &friday)
    .replace("{%saturday_day%}", &saturday)
    .replace("{%sunday_day%}", &sunday);
  Cached::new(Html::new(&html), Cache::ONE_HOUR)
}

#[rocket::get("/styles.css")]
fn css() -> Cached<(ContentType, &'static str)> {
  Cached::new(
    (ContentType::CSS, include_str!("assets/css/output.css")),
    Cache::ONE_HOUR,
  )
}

#[rocket::get("/podcast.xml")]
fn podcast_xml() -> Cached<(ContentType, String)> {
  Cached::new(
    (ContentType::new("application", "rss+xml"), podcast::xml()),
    Cache::ONE_MINUTE * 10,
  )
}

#[rocket::get("/check-9e328da2")]
fn refresh_check() -> rocket::response::status::NoContent {
  rocket::response::status::NoContent
}

// helpers

mod internal {
  pub use crate::cached::*;
  pub use crate::html::{self, *};
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::{Datelike, NaiveDateTime, Utc};
}
