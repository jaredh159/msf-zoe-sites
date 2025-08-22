use rocket::response::{Responder, Result};
use rocket::{Request, Response};
use chrono::Utc;

pub struct Cached<R> {
  responder: R,
  max_age: u64,
}

impl<R> Cached<R> {
  pub fn new(responder: R, max_age: u64) -> Self {
    Self { responder, max_age }
  }
}

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for Cached<R> {
  fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
    let mut response = self.responder.respond_to(request)?;
    if cfg!(debug_assertions) {
      response.set_raw_header("Cache-Control", "no-cache, no-store, must-revalidate");
    } else {
      response.set_raw_header("Cache-Control", format!("public, max-age={}", self.max_age));
    }
    response.set_raw_header("X-Served-At", Utc::now().to_rfc3339());
    Ok(response)
  }
}

pub struct Cache;

impl Cache {
  pub const ONE_MINUTE: u64 = 60;
  pub const ONE_HOUR: u64 = 60 * Self::ONE_MINUTE;
  pub const ONE_DAY: u64 = 24 * Self::ONE_HOUR;
  pub const ONE_WEEK: u64 = 7 * Self::ONE_DAY;
}
