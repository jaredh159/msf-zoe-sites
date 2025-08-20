use rocket::{http::ContentType, response::Responder};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Html {
  Html::new(include_str!("assets/index.en.html"))
}

#[get("/styles.css")]
fn css() -> (ContentType, &'static str) {
  (ContentType::CSS, include_str!("assets/output.css"))
}

#[get("/msf-logo.webp")]
fn logo() -> (ContentType, &'static [u8]) {
  (
    ContentType::new("image", "webp"),
    include_bytes!("assets/msf-logo.webp"),
  )
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, css, logo, check])
}

#[derive(Responder)]
#[response(content_type = "text/html")]
struct Html {
  content: String,
}

impl Html {
  fn new(content: &str) -> Self {
    let mut html = Self { content: content.to_string() };
    if cfg!(debug_assertions) {
      html.content.push_str(include_str!("assets/refresh.html"));
    }
    html
  }
}

#[get("/check")]
fn check() -> &'static str {
  "OK"
}
