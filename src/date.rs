use chrono::{Datelike, NaiveDate, Utc, Weekday};

pub fn spring_conf_weekend(year: i32) -> Vec<NaiveDate> {
  let memorial_day = last_monday_of_may(year);
  let friday = memorial_day - chrono::Duration::days(3);
  let saturday = memorial_day - chrono::Duration::days(2);
  let sunday = memorial_day - chrono::Duration::days(1);
  vec![friday, saturday, sunday]
}

fn last_monday_of_may(year: i32) -> NaiveDate {
  // Start from May 31st and work backwards to find the last Monday
  let mut date = NaiveDate::from_ymd_opt(year, 5, 31).unwrap();
  // If May 31st doesn't exist (shouldn't happen), try May 30th, then 29th, etc.
  while date.month() != 5 {
    date = date.pred_opt().unwrap();
  }
  // Find the last Monday by going backwards
  while date.weekday() != Weekday::Mon {
    date = date.pred_opt().unwrap();
  }
  date
}

pub fn show_spring_conf_banner(year: u32, month: u32, day: u32) -> bool {
  let current_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
  let current_year = current_date.year();
  let march_first = NaiveDate::from_ymd_opt(current_year, 3, 1).unwrap();
  let memorial_weekend = spring_conf_weekend(current_year);
  let memorial_friday = memorial_weekend[0];
  current_date >= march_first && current_date <= memorial_friday
}

pub fn current_date_parts() -> (u32, u32, u32) {
  let now_utc = Utc::now();
  // Convert to EST (UTC-5) - this is a simplification, doesn't handle DST
  let now_est = now_utc - chrono::Duration::hours(5);
  let current_date = now_est.date_naive();
  (
    current_date.year() as u32,
    current_date.month(),
    current_date.day(),
  )
}

pub fn is_during_spring_gathering(year: u32, month: u32, day: u32) -> bool {
  let current_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
  let weekend_dates = spring_conf_weekend(year as i32);

  weekend_dates.contains(&current_date)
}

pub fn gathering_session_times(year: u32, month: u32, day: u32) -> String {
  let current_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
  let weekend_dates = spring_conf_weekend(year as i32);

  if current_date == weekend_dates[0] || current_date == weekend_dates[1] {
    // Friday or Saturday: three sessions
    "Today’s sessions are at <b><i>9:30am</i></b>, <b><i>3:30pm</i></b>, and <b><i>7:00pm</i></b> (New York time).".to_string()
  } else if current_date == weekend_dates[2] {
    // Sunday: one session only
    "Today’s session is at <b><i>9:30am</i></b> (New York time).".to_string()
  } else {
    // Should not happen if called correctly, but fallback
    "Please check the schedule for session times.".to_string()
  }
}

pub fn spring_conf_banner_text(year: u32, _month: u32, _day: u32) -> String {
  let weekend = spring_conf_weekend(year as i32);
  let friday = weekend[0];
  let sunday = weekend[2];

  format!(
    "Spring Gathering May {}-{}, {}",
    friday.day(),
    sunday.day(),
    year
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Weekday;

  #[test]
  fn test_memorial_day() {
    let memorial_day = last_monday_of_may(2024);
    assert_eq!(memorial_day, NaiveDate::from_ymd_opt(2024, 5, 27).unwrap());
    assert_eq!(memorial_day.weekday(), Weekday::Mon);
    let memorial_day = last_monday_of_may(2025);
    assert_eq!(memorial_day, NaiveDate::from_ymd_opt(2025, 5, 26).unwrap());
    assert_eq!(memorial_day.weekday(), Weekday::Mon);
  }

  #[test]
  fn test_memorial_day_weekend_2024() {
    let weekend = spring_conf_weekend(2024);
    assert_eq!(weekend.len(), 3);

    // Should be May 24 (Fri), 25 (Sat), 26 (Sun)
    assert_eq!(weekend[0], NaiveDate::from_ymd_opt(2024, 5, 24).unwrap());
    assert_eq!(weekend[1], NaiveDate::from_ymd_opt(2024, 5, 25).unwrap());
    assert_eq!(weekend[2], NaiveDate::from_ymd_opt(2024, 5, 26).unwrap());

    // Verify weekdays
    assert_eq!(weekend[0].weekday(), Weekday::Fri);
    assert_eq!(weekend[1].weekday(), Weekday::Sat);
    assert_eq!(weekend[2].weekday(), Weekday::Sun);
  }

  #[test]
  fn test_memorial_day_weekend_2025() {
    let weekend = spring_conf_weekend(2025);
    assert_eq!(weekend.len(), 3);

    // Should be May 23 (Fri), 24 (Sat), 25 (Sun)
    assert_eq!(weekend[0], NaiveDate::from_ymd_opt(2025, 5, 23).unwrap());
    assert_eq!(weekend[1], NaiveDate::from_ymd_opt(2025, 5, 24).unwrap());
    assert_eq!(weekend[2], NaiveDate::from_ymd_opt(2025, 5, 25).unwrap());

    // Verify weekdays
    assert_eq!(weekend[0].weekday(), Weekday::Fri);
    assert_eq!(weekend[1].weekday(), Weekday::Sat);
    assert_eq!(weekend[2].weekday(), Weekday::Sun);
  }

  #[test]
  fn test_various_years() {
    let test_cases = vec![(2020, 25), (2021, 31), (2022, 30), (2023, 29)];
    for (year, expected_day) in test_cases {
      let memorial_day = last_monday_of_may(year);
      assert_eq!(memorial_day.day(), expected_day, "Failed for year {}", year);
      assert_eq!(memorial_day.month(), 5);
      assert_eq!(memorial_day.weekday(), Weekday::Mon);
    }
  }

  #[test]
  fn test_spring_gathering_banner_text() {
    // Test the banner text format for 2024 (month/day don't matter for banner text)
    let banner_text = spring_conf_banner_text(2024, 3, 15);
    assert_eq!(banner_text, "Spring Gathering May 24-26, 2024");

    // Test for 2025
    let banner_text_2025 = spring_conf_banner_text(2025, 4, 1);
    assert_eq!(banner_text_2025, "Spring Gathering May 23-25, 2025");
  }

  #[test]
  fn test_should_show_banner() {
    // assert!(should_show_spring_gathering_banner(2026, 3, 3)); // March 1
    // Test dates within the range (March 1 - Memorial Day Friday)
    assert!(show_spring_conf_banner(2024, 3, 1)); // March 1
    assert!(show_spring_conf_banner(2024, 4, 15)); // April 15
    assert!(show_spring_conf_banner(2024, 5, 24)); // Memorial Friday 2024

    // Test dates outside the range
    assert!(!show_spring_conf_banner(2024, 2, 28)); // Feb 28
    assert!(!show_spring_conf_banner(2024, 5, 25)); // Memorial Saturday 2024
    assert!(!show_spring_conf_banner(2024, 6, 1)); // June 1

    // Test 2025 dates
    assert!(show_spring_conf_banner(2025, 3, 1)); // March 1
    assert!(show_spring_conf_banner(2025, 5, 23)); // Memorial Friday 2025
    assert!(!show_spring_conf_banner(2025, 5, 24)); // Memorial Saturday 2025
  }

  #[test]
  fn test_is_during_spring_gathering() {
    // Test 2024 Memorial Day weekend (May 24-26)
    assert!(is_during_spring_gathering(2024, 5, 24)); // Friday
    assert!(is_during_spring_gathering(2024, 5, 25)); // Saturday
    assert!(is_during_spring_gathering(2024, 5, 26)); // Sunday

    // Test dates outside the weekend
    assert!(!is_during_spring_gathering(2024, 5, 23)); // Thursday before
    assert!(!is_during_spring_gathering(2024, 5, 27)); // Monday (Memorial Day)
    assert!(!is_during_spring_gathering(2024, 6, 1)); // June

    // Test 2025 Memorial Day weekend (May 23-25)
    assert!(is_during_spring_gathering(2025, 5, 23)); // Friday
    assert!(is_during_spring_gathering(2025, 5, 24)); // Saturday
    assert!(is_during_spring_gathering(2025, 5, 25)); // Sunday
    assert!(!is_during_spring_gathering(2025, 5, 22)); // Thursday before
  }

  #[test]
  fn test_gathering_session_times() {
    // Test 2024 Memorial Day weekend (May 24-26)
    let friday_text = gathering_session_times(2024, 5, 24);
    assert!(friday_text.contains("Today’s sessions are"));
    assert!(friday_text.contains("9:30am"));
    assert!(friday_text.contains("3:30pm"));
    assert!(friday_text.contains("7:00pm"));

    let saturday_text = gathering_session_times(2024, 5, 25);
    assert!(saturday_text.contains("Today’s sessions are"));
    assert!(saturday_text.contains("9:30am"));
    assert!(saturday_text.contains("3:30pm"));
    assert!(saturday_text.contains("7:00pm"));

    let sunday_text = gathering_session_times(2024, 5, 26);
    assert!(sunday_text.contains("Today’s session is"));
    assert!(sunday_text.contains("9:30am"));
    assert!(!sunday_text.contains("3:30pm"));
    assert!(!sunday_text.contains("7:00pm"));
  }
}
