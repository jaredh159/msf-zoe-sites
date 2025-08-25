use crate::internal::*;

#[derive(rocket::form::FromForm)]
pub struct NewTeachingForm<'r> {
  pub title: String,
  pub speaker: String,
  pub context: String,
  pub audio_file: rocket::fs::TempFile<'r>,
  pub duration: i64,
  pub date: String,
}

#[rocket::get("/add-teaching-5b2e3090?<success>&<error>")]
pub fn form(success: Option<bool>, error: Option<bool>) -> Html {
  let flash_html = if success == Some(true) {
    flash_message(true, "Teaching added successfully!")
  } else if error == Some(true) {
    flash_message(false, "Error saving teaching. Please try again.")
  } else {
    String::new()
  };

  let html = include_str!("assets/html/add-teaching.html")
    .replace("{%head%}", &html::head(Some("Add Teaching"), Lang::English))
    .replace("{%flash_message%}", &flash_html);

  Html::new(&html)
}

#[rocket::post("/add-teaching-5b2e3090", data = "<form>")]
pub async fn submit(
  form: rocket::form::Form<NewTeachingForm<'_>>,
  cookies: &rocket::http::CookieJar<'_>,
) -> Result<rocket::response::Redirect, rocket::http::Status> {
  let _ = dotenvy::dotenv();
  let expected_auth = match std::env::var("ADMIN_AUTH_TOKEN") {
    Ok(token) => token,
    Err(_) => return Err(rocket::http::Status::InternalServerError),
  };
  if let Some(auth_cookie) = cookies.get("auth") {
    if auth_cookie.value() != expected_auth {
      return Err(rocket::http::Status::Unauthorized);
    }
  } else {
    return Err(rocket::http::Status::Unauthorized);
  }

  let file_size = form.audio_file.len();
  let filename = generate_filename(&form.date, &form.title);

  let teaching = Teaching {
    title: form.title.clone(),
    speaker: form.speaker.clone(),
    context: if form.context.is_empty() { None } else { Some(form.context.clone()) },
    filename: filename.clone(),
    filesize: file_size as i64,
    duration: form.duration,
    date: form.date.clone(),
    lang: Lang::English,
    ..Default::default()
  };

  if let Some(temp_path) = form.audio_file.path() {
    match crate::s3::upload_audio_file(temp_path.to_str().unwrap(), &filename).await {
      Ok(_) => match teaching.save() {
        Ok(_) => Ok(Redirect::to("/add-teaching-5b2e3090?success=true")),
        Err(_) => Ok(Redirect::to("/add-teaching-5b2e3090?error=true&id=1")),
      },
      Err(_) => Ok(Redirect::to("/add-teaching-5b2e3090?error=true&id=2")),
    }
  } else {
    Ok(Redirect::to("/add-teaching-5b2e3090?error=true&id=3"))
  }
}

fn generate_filename(date: &str, title: &str) -> String {
  let title = normalize_part_numbers(title);
  format!(
    "{}-{}.mp3",
    date.replace("-", ""),
    title
      .replace(" ", "-")
      .replace(",", "")
      .replace(".", "")
      .replace("(", "")
      .replace(")", "")
      .chars()
      .filter(|c| c.is_alphanumeric() || *c == '-')
      .collect::<String>()
      .replace("---", "-")
      .replace("--", "-")
      .trim_matches('-')
  )
}

fn normalize_part_numbers(title: &str) -> String {
  // Split the title into words and look for part number patterns
  let words: Vec<&str> = title.split_whitespace().collect();
  let mut result_words = Vec::new();
  let mut i = 0;

  while i < words.len() {
    let word_lower = words[i].to_lowercase();

    // Check if current word is a part indicator and next word is a number
    if (word_lower == "part" || word_lower == "pt" || word_lower == "pt.")
      && i + 1 < words.len()
      && words[i + 1].chars().all(|c| c.is_ascii_digit())
    {
      result_words.push("pt");
      result_words.push(words[i + 1]);
      i += 2; // Skip the number word since we processed it
    } else {
      result_words.push(words[i]);
      i += 1;
    }
  }

  result_words.join(" ")
}

pub fn normalize_title(title: &str) -> String {
  // Split the title into words and look for part number patterns
  let words: Vec<&str> = title.split_whitespace().collect();
  let mut result_words = Vec::new();
  let mut i = 0;

  while i < words.len() {
    let word_lower = words[i].to_lowercase();

    // Check if current word is a part indicator and next word is a number
    if (word_lower == "part" || word_lower == "pt" || word_lower == "pt.")
      && i + 1 < words.len()
      && words[i + 1].chars().all(|c| c.is_ascii_digit())
    {
      // Add ", pt. N" format
      if !result_words.is_empty() {
        // Join the existing words, then add comma and pt. part
        let prefix = result_words.join(" ");
        return format!("{}, pt. {}", prefix, words[i + 1]);
      } else {
        result_words.push("pt.");
        result_words.push(words[i + 1]);
        i += 2;
        continue;
      }
    } else {
      result_words.push(words[i]);
      i += 1;
    }
  }

  result_words.join(" ")
}

fn flash_message(is_success: bool, message: &str) -> String {
  let (bg_color, border_color, text_color) = if is_success {
    ("bg-green-100", "border-green-400", "text-green-700")
  } else {
    ("bg-red-100", "border-red-400", "text-red-700")
  };

  format!(
    r#"<div class="mb-6 p-4 {} border {} {} rounded-md flash-message">{}</div>"#,
    bg_color, border_color, text_color, message
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_filename() {
    let test_cases = vec![
      ("2024-08-25", "Simple Title", "20240825-Simple-Title.mp3"),
      (
        "2023-12-01",
        "Title with Spaces",
        "20231201-Title-with-Spaces.mp3",
      ),
      (
        "2024-01-15",
        "Title, with Commas.",
        "20240115-Title-with-Commas.mp3",
      ),
      (
        "2024-02-28",
        "Title (with parens)",
        "20240228-Title-with-parens.mp3",
      ),
      (
        "2024-03-10",
        "Title -- with -- dashes",
        "20240310-Title-with-dashes.mp3",
      ),
      (
        "2024-04-05",
        "Title...with...dots",
        "20240405-Titlewithdots.mp3",
      ),
      (
        "2024-05-20",
        "UPPERCASE Title",
        "20240520-UPPERCASE-Title.mp3",
      ),
      (
        "2024-06-30",
        "Mix3d C4se & Numb3rs!",
        "20240630-Mix3d-C4se-Numb3rs.mp3",
      ),
      (
        "2024-07-15",
        "   Leading and Trailing Spaces   ",
        "20240715-Leading-and-Trailing-Spaces.mp3",
      ),
      (
        "2024-08-01",
        "Special@Chars#Here$",
        "20240801-SpecialCharsHere.mp3",
      ),
      (
        "2024-09-12",
        "- Starting with dash -",
        "20240912-Starting-with-dash.mp3",
      ),
      (
        "2024-10-31",
        "Multiple    Spaces",
        "20241031-Multiple-Spaces.mp3",
      ),
      (
        "2024-11-15",
        "Seeking the Will",
        "20241115-Seeking-the-Will.mp3",
      ),
      // Part number normalization tests
      (
        "2024-12-01",
        "Teaching Series Part 1",
        "20241201-Teaching-Series-pt-1.mp3",
      ),
      (
        "2024-12-02",
        "Teaching Series Part 2",
        "20241202-Teaching-Series-pt-2.mp3",
      ),
      (
        "2024-12-03",
        "Teaching Series Pt. 3",
        "20241203-Teaching-Series-pt-3.mp3",
      ),
      (
        "2024-12-04",
        "Teaching Series Pt 4",
        "20241204-Teaching-Series-pt-4.mp3",
      ),
      (
        "2024-12-05",
        "Teaching Series part 5",
        "20241205-Teaching-Series-pt-5.mp3",
      ),
      (
        "2024-12-06",
        "Teaching Series pt. 6",
        "20241206-Teaching-Series-pt-6.mp3",
      ),
      (
        "2024-12-07",
        "Teaching Series pt 7",
        "20241207-Teaching-Series-pt-7.mp3",
      ),
      (
        "2024-12-08",
        "Teaching Series PART 8",
        "20241208-Teaching-Series-pt-8.mp3",
      ),
      (
        "2024-12-09",
        "Teaching Series PT. 9",
        "20241209-Teaching-Series-pt-9.mp3",
      ),
      (
        "2024-12-10",
        "Teaching Series PT 10",
        "20241210-Teaching-Series-pt-10.mp3",
      ),
    ];

    for (date, title, expected) in test_cases {
      let result = generate_filename(date, title);
      assert_eq!(
        result, expected,
        "Failed for date: '{}', title: '{}'",
        date, title
      );
    }
  }

  #[test]
  fn test_normalize_title() {
    let test_cases = vec![
      ("Topic Part 1", "Topic, pt. 1"),
      ("Topic Part 2", "Topic, pt. 2"),
      ("Topic Pt. 3", "Topic, pt. 3"),
      ("Topic Pt 4", "Topic, pt. 4"),
      ("Topic part 5", "Topic, pt. 5"),
      ("Topic pt. 6", "Topic, pt. 6"),
      ("Topic pt 7", "Topic, pt. 7"),
      ("Topic PART 8", "Topic, pt. 8"),
      ("Topic PT. 9", "Topic, pt. 9"),
      ("Topic PT 10", "Topic, pt. 10"),
      ("Simple Title", "Simple Title"),
      ("Title with Spaces", "Title with Spaces"),
    ];

    for (input, expected) in test_cases {
      let result = normalize_title(input);
      assert_eq!(result, expected, "Failed for input: '{}'", input);
    }
  }
}
