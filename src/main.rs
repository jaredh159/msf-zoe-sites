#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use internal::*;
use rocket::http::ContentType;

pub mod cached;
pub mod component;
pub mod date;
pub mod html;
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
      logo_webp,
      logo_svg,
      apple_podcasts,
      google_podcasts,
      overcast,
      spotify,
      rss_png,
      favicon_ico,
      favicon_16x16,
      favicon_32x32,
      podcast_xml,
      refresh_check,
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

#[rocket::get("/msf-logo.webp")]
fn logo_webp() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/msf-logo.webp"))
}

#[rocket::get("/apple-podcasts.webp")]
fn apple_podcasts() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/apple-podcasts.webp"))
}

#[rocket::get("/google-podcasts.webp")]
fn google_podcasts() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/google-podcasts.webp"))
}

#[rocket::get("/overcast.webp")]
fn overcast() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/overcast.webp"))
}

#[rocket::get("/spotify.webp")]
fn spotify() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/spotify.webp"))
}

#[rocket::get("/rss.png")]
fn rss_png() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/rss.png"))
}

#[rocket::get("/favicon.ico")]
fn favicon_ico() -> Cached<(ContentType, &'static [u8])> {
  Cached::new(
    (ContentType::Icon, include_bytes!("assets/img/favicon.ico")),
    Cache::ONE_WEEK,
  )
}

#[rocket::get("/favicon-16x16.png")]
fn favicon_16x16() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/favicon-16x16.png"))
}

#[rocket::get("/favicon-32x32.png")]
fn favicon_32x32() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/favicon-32x32.png"))
}

#[rocket::get("/msf-logo.svg")]
fn logo_svg() -> Cached<(ContentType, &'static str)> {
  Cached::new(
    (
      ContentType::new("image", "svg+xml"),
      include_str!("assets/img/logo.svg"),
    ),
    Cache::ONE_WEEK,
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

fn serve_image(
  image_type: &'static str,
  bytes: &'static [u8],
) -> Cached<(ContentType, &'static [u8])> {
  Cached::new(
    (ContentType::new("image", image_type), bytes),
    Cache::ONE_WEEK,
  )
}

mod internal {
  pub use crate::cached::*;
  pub use crate::html::{self, *};
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::{Datelike, NaiveDateTime, Utc};
}
