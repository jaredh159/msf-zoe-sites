use crate::internal::*;

#[derive(Debug, Clone)]
pub struct Teaching {
  pub id: i64,
  pub title: String,
  pub speaker: String,
  pub context: Option<String>,
  pub filename: String,
  pub filesize: i64,
  pub duration: i64,
  pub date: String,
  pub text_url: Option<&'static str>,
  pub lang: Lang,
}

impl Teaching {
  pub fn load_most_recent(n: usize) -> Vec<Self> {
    Self::load_with_query("SELECT * FROM teachings ORDER BY date DESC LIMIT ?", [n])
  }

  pub fn load_all() -> Vec<Self> {
    Self::load_with_query("SELECT * FROM teachings ORDER BY date ASC", [])
  }

  pub fn datetime(&self) -> NaiveDateTime {
    time::parse_sqlite_datetime(&self.date).unwrap()
  }

  pub fn short_date(&self) -> String {
    time::format_short_date(self.datetime())
  }

  pub fn rfc2822_date(&self) -> String {
    time::format_rfc2822(self.datetime())
  }

  pub fn human_duration(&self) -> String {
    time::format_duration(self.duration)
  }

  pub fn url(&self) -> String {
    format!(
      "https://msf-assets.nyc3.digitaloceanspaces.com/{}-audios/{}",
      if self.lang == Lang::English { "msf" } else { "zoe" },
      self.filename
    )
  }

  fn load_with_query<P>(query: &str, params: P) -> Vec<Self>
  where
    P: rusqlite::Params,
  {
    let conn = Connection::open("teachings.db").expect("Failed to open database");
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt
      .query_map(params, |row| {
        Ok(Self {
          id: row.get(0)?,
          title: row.get(1)?,
          speaker: row.get(2)?,
          context: row.get(3)?,
          filename: row.get(4)?,
          filesize: row.get(5)?,
          duration: row.get(6)?,
          date: row.get(7)?,
          text_url: None,
          lang: Lang::English,
        })
      })
      .unwrap();

    rows.collect::<Result<Vec<_>, _>>().unwrap()
  }
}

impl Default for Teaching {
  fn default() -> Self {
    Self {
      id: 0,
      title: String::new(),
      speaker: "Jason Henderson".to_string(),
      context: None,
      filename: String::new(),
      filesize: 0,
      duration: 0,
      date: String::new(),
      text_url: None,
      lang: Lang::Spanish,
    }
  }
}
