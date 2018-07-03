extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate proptest;

use chrono::NaiveDate;

static MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    InvalidDateError,
    InvalidDayError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(T: std::num::ParseIntError) -> Error {
        Error::ParseError
    }
}

fn parse_month(month: &str) -> Option<usize> {
    MONTH_NAMES.iter().position(|&elem| elem == month).map(|day| day + 1)
}

fn parse_day(day_with_ordinal: &str) -> Result<u32, Error> {
    let day: u32 = day_with_ordinal
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()?;
    let ordinal = day_with_ordinal
        .chars()
        .skip_while(|c| c.is_digit(10))
        .collect::<String>();

    match (day, ordinal.as_ref()) {
        (1, "st")
        | (2, "nd")
        | (3, "rd")
        | (4...20, "th")
        | (21, "st")
        | (22, "nd")
        | (23, "rd")
        | (24...30, "th")
        | (31, "st") => Ok(day),
        _ => Err(Error::InvalidDayError),
    }
}

/// Parses a human-readable date format:
/// 4th of July 2018
/// 2nd of March 2006
/// 3rd of November 1975
pub fn parse_date(humandate: impl AsRef<str>) -> Result<chrono::NaiveDate, Error> {
    let parts: Vec<&str> = humandate.as_ref().split_whitespace().collect();
    if parts.len() != 4 {
        return Err(Error::ParseError);
    }
    let day: u32 = parse_day(parts[0])?;

    // TODO: parts[1] == of

    let month: u32 = parse_month(parts[2]).ok_or_else(|| Error::ParseError)? as u32;
    let year: i32 = parts[3].parse()?;
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::InvalidDateError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            parse_date("04th of September 2015"),
            Ok(NaiveDate::from_ymd(2015, 9, 4))
        );
        assert_eq!(
            parse_date("05th of September 2015"),
            Ok(NaiveDate::from_ymd(2015, 9, 5))
        );
        assert_eq!(
            parse_date("1st of September 2015"),
            Ok(NaiveDate::from_ymd(2015, 9, 1))
        );
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref s in "\\PC*") {
            parse_date(s);
        }

        #[test]
        fn handles_invalid_words(ref s in "([0-9a-z]{1,5} ){3}[0-9a-z]{1,5}") {
            let _ = parse_date(s);
        }

        #[test]
        fn parses_date_back_to_original(y in 0i32..10000, m in 0usize..12, d in 1u32..32) {
            let human_month = MONTH_NAMES[m];
            let ordinal = match d {
                1 | 21 | 31 => "st",
                2 | 22 => "nd",
                3 | 23 => "rd",
                _ => "th",
            };
            let date_string = format!("{}{} of {} {}", d, ordinal, human_month,y);
            println!("{}", date_string);
            assert_eq!(parse_date(date_string), chrono::NaiveDate::from_ymd_opt(y, (m + 1) as u32, d).ok_or_else(|| Error::InvalidDateError));
        }
    }
}
