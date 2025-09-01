use crate::component;
use crate::date;
use crate::internal::*;

pub fn get() -> Html {
  let teachings = Teaching::load_most_recent(5);
  let links = vec![
    component::Link::new(
      "Friends Library",
      "https://www.friendslibrary.com",
      "Market Street Fellowship has been so impacted by the writings of the early (not modern!) Society of Friends (1650-1800), that we put together a website where hundreds of their books are available for free, in a variety of different text and audio formats.",
    ),
    component::Link::new(
      "Ancient Path",
      "https://hender.blog",
      "More teachings and posts (in text and audio) from Jason Henderson on a variety of different subjects.",
    ),
    component::Link::new(
      "Gertrude",
      "https://gertrude.app",
      "Jared Henderson and Miciah Henderson have spent years building what we believe to be the safest parental control software in existence for Apple computers. There is also a corresponding Gertrude iPhone app that plugs some of the holes that Apple's Screen Time feature misses.",
    ),
  ];

  let (year, month, day) = date::current_date_parts();
  // let (year, month, day) = (2026, 5, 22); // <-- test dates

  let banner_html = if date::is_during_spring_gathering(year, month, day) {
    include_str!("assets/html/ongoing-banner.html")
      .replace("{%gathering_year%}", &year.to_string())
      .replace(
        "{%session_times%}",
        &date::gathering_session_times(year, month, day),
      )
  } else if date::show_spring_conf_banner(year, month, day) {
    include_str!("assets/html/upcoming-banner.html").replace(
      "{%conf_upcoming%}",
      &date::spring_conf_banner_text(year, month, day),
    )
  } else {
    String::new()
  };

  let podcast_links = include_str!("assets/html/podcast-links.html");
  let html = include_str!("assets/html/index.en.html")
    .replace("{%head%}", &html::head(None, Lang::English))
    .replace("{%banner%}", &banner_html)
    .replace("{%hero%}", &component::Hero::new(Lang::English).html())
    .replace("{%podcast_links%}", podcast_links)
    .replace(
      "{%audios%}",
      &teachings
        .into_iter()
        .map(|t| component::Audio { teaching: t }.html())
        .collect::<Vec<_>>()
        .join("\n"),
    )
    .replace(
      "{%links%}",
      &links
        .into_iter()
        .map(|l| l.html())
        .collect::<Vec<_>>()
        .join("\n"),
    );

  Html::new(&html)
}
