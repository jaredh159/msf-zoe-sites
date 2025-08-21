use crate::internal::*;

pub fn xml() -> String {
  let mut teachings = Teaching::load_all();
  teachings.reverse();
  let items: Vec<String> = teachings.iter().map(|t| episode(t)).collect();
  let template = include_str!("assets/podcast.xml");
  template.replace("{%episodes%}", &items.join(""))
}

fn episode(teaching: &Teaching) -> String {
  let description = format!(
    "\"{}\" by {}, from Market Street Fellowship",
    teaching.title, teaching.speaker
  );
  let template = include_str!("assets/episode.xml");
  template
    .replace("{%title%}", &teaching.title)
    .replace("{%speaker%}", &teaching.speaker)
    .replace("{%description%}", &description)
    .replace("{%id%}", &teaching.id.to_string())
    .replace("{%duration%}", &teaching.duration.to_string())
    .replace("{%filesize%}", &teaching.filesize.to_string())
    .replace("{%url%}", &teaching.url())
    .replace("{%date%}", &teaching.rfc2822_date())
}
