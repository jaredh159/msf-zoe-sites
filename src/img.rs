use crate::internal::*;

#[rocket::get("/msf-logo.webp")]
pub fn logo_webp() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/msf-logo.webp"))
}

#[rocket::get("/apple-podcasts.webp")]
pub fn apple_podcasts() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/apple-podcasts.webp"))
}

#[rocket::get("/google-podcasts.webp")]
pub fn google_podcasts() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/google-podcasts.webp"))
}

#[rocket::get("/overcast.webp")]
pub fn overcast() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/overcast.webp"))
}

#[rocket::get("/spotify.webp")]
pub fn spotify() -> Cached<(ContentType, &'static [u8])> {
  serve_image("webp", include_bytes!("assets/img/spotify.webp"))
}

#[rocket::get("/rss.png")]
pub fn rss_png() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/rss.png"))
}

#[rocket::get("/favicon.ico")]
pub fn favicon_ico() -> Cached<(ContentType, &'static [u8])> {
  Cached::new(
    (ContentType::Icon, include_bytes!("assets/img/favicon.ico")),
    Cache::ONE_WEEK,
  )
}

#[rocket::get("/favicon-16x16.png")]
pub fn favicon_16x16() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/favicon-16x16.png"))
}

#[rocket::get("/favicon-32x32.png")]
pub fn favicon_32x32() -> Cached<(ContentType, &'static [u8])> {
  serve_image("png", include_bytes!("assets/img/favicon-32x32.png"))
}

#[rocket::get("/msf-logo.svg")]
pub fn logo_svg() -> Cached<(ContentType, &'static str)> {
  Cached::new(
    (
      ContentType::new("image", "svg+xml"),
      include_str!("assets/img/logo.svg"),
    ),
    Cache::ONE_WEEK,
  )
}

fn serve_image(
  image_type: &'static str,
  bytes: &'static [u8],
) -> Cached<(ContentType, &'static [u8])> {
  Cached::new(
    (ContentType::new("image", image_type), bytes),
    Cache::ONE_WEEK,
  )
}

