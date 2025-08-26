use crate::internal::*;

#[derive(Debug, Clone)]
pub struct Link {
  pub title: String,
  pub url: String,
  pub description: String,
}

impl Link {
  pub fn new(title: &str, url: &str, description: &str) -> Self {
    Self {
      title: title.to_string(),
      url: url.to_string(),
      description: description.to_string(),
    }
  }

  pub fn html(&self) -> String {
    let template = include_str!("link.html");
    template
      .replace("{%title%}", &self.title)
      .replace("{%url%}", &self.url)
      .replace("{%description%}", &self.description)
  }
}
