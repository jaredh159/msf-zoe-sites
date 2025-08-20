use crate::internal::*;

#[derive(Debug, Clone)]
pub struct Audio {
  pub teaching: Teaching,
}

impl Audio {
  pub fn html(&self) -> String {
    let template = include_str!("audio.html");
    template
      .replace("{%title%}", &self.teaching.title)
      .replace("{%speaker%}", &self.teaching.speaker)
      .replace("{%date%}", &self.teaching.short_date())
      .replace("{%url%}", &self.teaching.url())
      .replace("{%duration%}", &self.teaching.human_duration())
  }
}
