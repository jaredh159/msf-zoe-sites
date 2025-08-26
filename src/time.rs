use chrono::{NaiveDateTime, ParseError};

/// Parses SQLite datetime strings in the format "YYYY-MM-DD HH:MM:SS" into chrono NaiveDateTime objects
pub fn parse_sqlite_datetime(datetime_str: &str) -> Result<NaiveDateTime, ParseError> {
  NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
}

/// Formats a NaiveDateTime into short date format like "7/25/2025"
pub fn format_short_date(datetime: NaiveDateTime) -> String {
  datetime.format("%-m/%-d/%Y").to_string()
}

/// Formats duration in seconds into a time string like "4:35" or "1:22:09"
pub fn format_duration(seconds: i64) -> String {
  let hours = seconds / 3600;
  let minutes = (seconds % 3600) / 60;
  let secs = seconds % 60;

  if hours > 0 {
    format!("{}:{:02}:{:02}", hours, minutes, secs)
  } else {
    format!("{}:{:02}", minutes, secs)
  }
}

/// Formats a NaiveDateTime into RFC 2822 format string
pub fn format_rfc2822(datetime: NaiveDateTime) -> String {
  datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::NaiveDate;

  #[test]
  fn test_parse_sqlite_datetime() {
    let datetime_str = "2024-07-28 12:00:00";
    let result = parse_sqlite_datetime(datetime_str).unwrap();

    let expected = NaiveDate::from_ymd_opt(2024, 7, 28)
      .unwrap()
      .and_hms_opt(12, 0, 0)
      .unwrap();

    assert_eq!(result, expected);
  }

  #[test]
  fn test_format_short_date() {
    let datetime = NaiveDate::from_ymd_opt(2025, 12, 25)
      .unwrap()
      .and_hms_opt(12, 30, 45)
      .unwrap();

    let result = format_short_date(datetime);
    assert_eq!(result, "12/25/2025");
  }

  #[test]
  fn test_format_short_date_single_digits() {
    let datetime = NaiveDate::from_ymd_opt(2024, 1, 3)
      .unwrap()
      .and_hms_opt(9, 15, 0)
      .unwrap();

    let result = format_short_date(datetime);
    assert_eq!(result, "1/3/2024");
  }

  #[test]
  fn test_format_duration() {
    let test_cases = vec![
      (35, "0:35"),      // Less than a minute
      (275, "4:35"),     // Minutes and seconds
      (3600, "1:00:00"), // Exactly 1 hour
      (3723, "1:02:03"), // Hours with leading zeros
      (4985, "1:23:05"), // Hours, minutes, seconds
      (0, "0:00"),       // Zero seconds
      (60, "1:00"),      // Exactly 1 minute
      (3661, "1:01:01"), // More leading zeros
      (7321, "2:02:01"), // 2+ hours with leading zeros
    ];

    for (seconds, expected) in test_cases {
      let result = format_duration(seconds);
      assert_eq!(result, expected, "Failed for {} seconds", seconds);
    }
  }

  #[test]
  fn test_format_rfc2822() {
    let datetime = NaiveDate::from_ymd_opt(2024, 7, 28)
      .unwrap()
      .and_hms_opt(12, 30, 45)
      .unwrap();

    let result = format_rfc2822(datetime);
    assert_eq!(result, "Sun, 28 Jul 2024 12:30:45 GMT");
  }

  #[test]
  fn test_format_rfc2822_single_digit_day() {
    let datetime = NaiveDate::from_ymd_opt(2025, 1, 3)
      .unwrap()
      .and_hms_opt(9, 15, 0)
      .unwrap();

    let result = format_rfc2822(datetime);
    assert_eq!(result, "Fri, 03 Jan 2025 09:15:00 GMT");
  }
}
