use std::time::{SystemTime, UNIX_EPOCH};

const YEAR_DELTAS: &[u8; 401] = &[
    0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
    8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 14,
    15, 15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 20, 20,
    21, 21, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, // 100
    25, 25, 25, 25, 25, 26, 26, 26, 26, 27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 29, 29, 30, 30, 30,
    30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33, 34, 34, 34, 34, 35, 35, 35, 35, 36, 36, 36,
    36, 37, 37, 37, 37, 38, 38, 38, 38, 39, 39, 39, 39, 40, 40, 40, 40, 41, 41, 41, 41, 42, 42, 42,
    42, 43, 43, 43, 43, 44, 44, 44, 44, 45, 45, 45, 45, 46, 46, 46, 46, 47, 47, 47, 47, 48, 48, 48,
    48, 49, 49, 49, // 200
    49, 49, 49, 49, 49, 50, 50, 50, 50, 51, 51, 51, 51, 52, 52, 52, 52, 53, 53, 53, 53, 54, 54, 54,
    54, 55, 55, 55, 55, 56, 56, 56, 56, 57, 57, 57, 57, 58, 58, 58, 58, 59, 59, 59, 59, 60, 60, 60,
    60, 61, 61, 61, 61, 62, 62, 62, 62, 63, 63, 63, 63, 64, 64, 64, 64, 65, 65, 65, 65, 66, 66, 66,
    66, 67, 67, 67, 67, 68, 68, 68, 68, 69, 69, 69, 69, 70, 70, 70, 70, 71, 71, 71, 71, 72, 72, 72,
    72, 73, 73, 73, // 300
    73, 73, 73, 73, 73, 74, 74, 74, 74, 75, 75, 75, 75, 76, 76, 76, 76, 77, 77, 77, 77, 78, 78, 78,
    78, 79, 79, 79, 79, 80, 80, 80, 80, 81, 81, 81, 81, 82, 82, 82, 82, 83, 83, 83, 83, 84, 84, 84,
    84, 85, 85, 85, 85, 86, 86, 86, 86, 87, 87, 87, 87, 88, 88, 88, 88, 89, 89, 89, 89, 90, 90, 90,
    90, 91, 91, 91, 91, 92, 92, 92, 92, 93, 93, 93, 93, 94, 94, 94, 94, 95, 95, 95, 95, 96, 96, 96,
    96, 97, 97, 97, 97, // 400 + 1
];

fn get_date_from_epoch_timestamp(timestamp: u64) -> (u64, u64, u64) {
    let days = timestamp.div_euclid(86_400);
    let offset_days = days.checked_add(719_163).unwrap();
    let offset_days = offset_days.checked_add(365).unwrap();
    let year_div_400 = offset_days.div_euclid(146_097);
    let cycle = offset_days.rem_euclid(146_097);
    let mut year_mod_400 = cycle / 365;
    let mut ordinal0 = cycle % 365;
    let delta = YEAR_DELTAS[year_mod_400 as usize] as u64;
    if ordinal0 < delta {
        year_mod_400 -= 1;
        ordinal0 += 365 - delta;
    } else {
        ordinal0 -= delta;
    }
    let ordinal = ordinal0 + 1;
    let year = year_div_400 * 400 + year_mod_400;
    let is_leap_year = year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0));
    let days_in_year = if is_leap_year {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month = 0;
    let mut total_days = 0;
    while total_days < ordinal {
        total_days += days_in_year[month];
        month += 1;
    }
    let day = days_in_year[month - 1] - (total_days - ordinal);
    (year, month as u64, day)
}

enum DateFormat {
    DEFAULT,
    ISO8601,
}

struct Date {
    year: u64,
    month: u64,
    day: u64,
}

impl Date {
    pub fn from_epoch_timestamp(timestamp: u64) -> Date {
        let (year, month, day) = get_date_from_epoch_timestamp(timestamp);
        Date {
            year,
            month,
            day,
        }
    }

    pub fn now() -> Date {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time before Unix epoch");
        let secs = now.as_secs();
        let (year, month, day) = get_date_from_epoch_timestamp(secs);
        Date {
            year,
            month,
            day,
        }
    }

    pub fn to_string(&self, date_format: Option<DateFormat>) -> String{
        match date_format {
            None | Some(DateFormat::DEFAULT) | Some(DateFormat::ISO8601)  => {
                format!("{}-{:02}-{:02}", self.year, self.month, self.day)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_get_date_from_epoch_timestamp() {
        let date = Duration::from_secs(932515200);
        let (year, month, day) = get_date_from_epoch_timestamp(date.as_secs());
        assert_eq!(year, 1999);
        assert_eq!(month, 7);
        assert_eq!(day, 21);
    }

    #[test]
    fn test_from_epoch_timestamp() {
        let date = Date::from_epoch_timestamp(932515200);
        assert_eq!(date.year, 1999);
        assert_eq!(date.month, 7);
        assert_eq!(date.day, 21);
    }

    #[test]
    fn test_to_string() {
        let date = Date::from_epoch_timestamp(932515200);
        let none = date.to_string(None);
        let default = date.to_string(Some(DateFormat::DEFAULT));
        let iso8601 = date.to_string(Some(DateFormat::ISO8601));
        assert_eq!(none, "1999-07-21");
        assert_eq!(default, "1999-07-21");
        assert_eq!(iso8601, "1999-07-21");
    }
}