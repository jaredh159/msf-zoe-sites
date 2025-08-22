#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use internal::*;
use rocket::http::ContentType;

pub mod cached;
pub mod component;
pub mod html;
pub mod podcast;
pub mod teaching;
pub mod time;

#[rocket::launch]
fn rocket() -> _ {
  rocket::build().mount(
    "/",
    rocket::routes![
      index,
      audios,
      css,
      logo_webp,
      logo_svg,
      apple_podcasts,
      google_podcasts,
      overcast,
      spotify,
      rss_png,
      podcast_xml,
      refresh_check,
    ],
  )
}

#[rocket::get("/")]
fn index() -> Cached<Html> {
  let teachings = Teaching::load_most_recent(5);
  let links = vec![
    component::Link::new(
      "Friends Library",
      "https://www.friendslibrary.com",
      "Market Street Fellowship has been so impacted by the writings of the early (not modern!) Society of Friends (1650-1800), that we put together a website where hundreds of their books are available for free, in a variety of different text and audio formats.",
    ),
    component::Link::new(
      "Gertrude",
      "https://gertrude.app",
      "Jared Henderson and Miciah Henderson have spent years building what we believe to be the safest parental control software in existence for Apple computers. There is also a corresponding Gertrude iPhone app that plugs some of the holes that Apple's Screen Time feature misses.",
    ),
    component::Link::new(
      "Ancient Path",
      "https://hender.blog",
      "More teachings and posts (in text and audio) from Jason Henderson on a variety of different subjects.",
    ),
  ];

  let html = include_str!("assets/html/index.en.html")
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
  Cached::new(Html::new(&html), Cache::ONE_HOUR)
}

#[rocket::get("/audios")]
fn audios() -> Cached<Html> {
  let teachings = Teaching::load_all();
  let html = include_str!("assets/html/audios.html").replace(
    "{%audios%}",
    &teachings
      .into_iter()
      .map(|t| component::Audio { teaching: t }.html())
      .collect::<Vec<_>>()
      .join("\n"),
  );
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
  pub use crate::html::*;
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::NaiveDateTime;
}
