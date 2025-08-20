use rocket::response::Responder;

#[derive(Responder)]
#[response(content_type = "text/html")]
pub struct Html {
  content: String,
}

impl Html {
  pub fn new(content: &str) -> Self {
    let mut html = Self { content: content.to_string() };
    if cfg!(debug_assertions) {
      html.content.push_str(include_str!("assets/refresh.html"));
    }
    html
  }
}
