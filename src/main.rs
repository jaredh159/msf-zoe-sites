use internal::*;
use rocket::http::ContentType;

mod html;
mod teaching;

#[rocket::launch]
fn rocket() -> _ {
  rocket::build().mount("/", rocket::routes![index, css, logo, check, testdb])
}

#[rocket::get("/")]
fn index() -> Html {
  Html::new(include_str!("assets/index.en.html"))
}

#[rocket::get("/test.db")]
fn testdb() -> String {
  let recent = Teaching::load_most_recent(3);
  format!(
    "{}",
    recent
      .iter()
      .map(|t| format!("{} - {}\n", t.title, t.speaker))
      .collect::<String>()
  )
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
}
