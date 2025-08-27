use internal::*;

pub mod add_teaching;
pub mod cached;
pub mod component;
pub mod date;
pub mod html;
pub mod img;
pub mod msf;
pub mod podcast;
pub mod s3;
pub mod teaching;
pub mod time;
pub mod zoe;

#[rocket::launch]
fn rocket() -> _ {
  rocket::build()
    .configure(
      rocket::Config::figment()
        .merge(("limits.form", "100MiB"))
        .merge(("limits.file", "100MiB"))
        .merge(("limits.data-form", "100MiB")),
    )
    .mount(
      "/",
      rocket::routes![
        msf_home,
        zoe_home,
        audios,
        gathering_details,
        css,
        podcast_xml,
        robots_txt,
        refresh_check,
        add_teaching::form,
        add_teaching::submit,
        img::logo_webp,
        img::logo_svg,
        img::apple_podcasts,
        img::overcast,
        img::spotify,
        img::rss_png,
        img::favicon_ico,
        img::favicon_16x16,
        img::favicon_32x32,
      ],
    )
    .register("/", rocket::catchers![not_found])
}

#[rocket::get("/")]
fn msf_home() -> Cached<Html> {
  Cached::new(msf::get(), Cache::ONE_HOUR)
}

#[rocket::get("/zoe")]
fn zoe_home() -> Cached<Html> {
  Cached::new(zoe::get(), Cache::ONE_HOUR)
}

#[rocket::get("/audios")]
fn audios() -> Cached<Html> {
  let mut teachings = Teaching::load_all();
  teachings.reverse();
  let html = include_str!("assets/html/audios.html")
    .replace("{%head%}", &html::head(Some("Audios"), Lang::English))
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
    .replace(
      "{%head%}",
      &html::head(Some("Spring Gathering Details"), Lang::English),
    )
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

#[rocket::get("/robots.txt")]
fn robots_txt() -> Cached<(ContentType, &'static str)> {
  Cached::new(
    (ContentType::Plain, "User-agent: *\nAllow: /\n"),
    Cache::ONE_WEEK,
  )
}

#[rocket::get("/check-9e328da2")]
fn refresh_check() -> rocket::response::status::NoContent {
  rocket::response::status::NoContent
}

#[rocket::catch(404)]
fn not_found(req: &rocket::Request) -> Cached<Html> {
  let is_spanish = req
    .headers()
    .get_one("host")
    .map(|host| host == "zoecostarica.com")
    .unwrap_or(false);

  let html = if is_spanish {
    include_str!("assets/html/404.html")
      .replace(
        "{%head%}",
        &html::head(Some("No Encontrado"), Lang::Spanish),
      )
      .replace("{%home_link%}", "/zoe")
      .replace("{%home_text%}", "inicio")
      .replace("{%title%}", "Página No Encontrada")
      .replace(
        "{%message%}",
        "Lo sentimos, no se pudo encontrar la página que estás buscando.",
      )
  } else {
    include_str!("assets/html/404.html")
      .replace("{%head%}", &html::head(Some("Not Found"), Lang::English))
      .replace("{%home_link%}", "/")
      .replace("{%home_text%}", "home")
      .replace("{%title%}", "Page Not Found")
      .replace(
        "{%message%}",
        "Sorry, the page you are looking for could not be found.",
      )
  };

  Cached::new(Html::new(&html), Cache::ONE_HOUR)
}

// helpers

mod internal {
  pub use crate::cached::*;
  pub use crate::component;
  pub use crate::html::{self, *};
  pub use crate::teaching::*;
  pub use crate::time;
  pub use chrono::{NaiveDateTime, Utc};
  pub use rocket::http::ContentType;
  pub use rocket::response::{Responder, Result as ResponseResult};
  pub use rocket::{Request, response::Redirect};
  pub use rusqlite::{Connection, Result};
}
