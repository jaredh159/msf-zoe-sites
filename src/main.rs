use rocket::response::Responder;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Html {
  Html::new("<h1>Hello, rofl!</h1>")
}

#[get("/check")]
fn check() -> &'static str {
  "OK"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, check])
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
      html.content.push_str(include_str!("refresh.html"));
    }
    html
  }
}

//
// impl Responder for Html { fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
//     todo!()
//   }
// }
