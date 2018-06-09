#[cfg(test)]
#[macro_use]
extern crate proptest;

extern crate chrono;

use chrono::prelude::*;
use std::convert::From;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Parse,
    InvalidDate,
}

impl From<ParseIntError> for Error {
    fn from(_e: ParseIntError) -> Error {
        Error::Parse
    }
}

fn numeric_month(month: impl AsRef<str>) -> Option<u32> {
    let num = match month.as_ref() {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => return None,
    };
    Some(num)
}

pub fn parse(humandate: impl AsRef<str>) -> Result<chrono::NaiveDate, Error> {
    let parts: Vec<String> = humandate
        .as_ref()
        .split_whitespace()
        .map(String::from)
        .collect();
    if parts.len() != 4 {
        return Err(Error::Parse);
    }
    let day_human = &parts[0];
    if day_human.len() != 4 {
        return Err(Error::Parse);
    }
    let month_human = &parts[2];
    let year = parts[3].parse::<i32>()?;
    let (day_str, _suffix) = day_human.split_at(2);

    let day = day_str.parse::<u32>()?;
    let month = match numeric_month(month_human) {
        Some(val) => val,
        None => return Err(Error::Parse),
    };

    // Avoid chrono panic: 'No such local time'
    // see chrono/src/offset/mod.rs:145:34
    NaiveDate::from_ymd_opt(year, month, day).ok_or(Error::InvalidDate)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(
            parse("15th of May 2015"),
            Ok(NaiveDate::from_ymd(2015, 5, 15))
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("50th of May 2015"), Err(Error::InvalidDate));
    }

    proptest! {
    #[test]
    fn doesnt_crash(ref s in "\\PC*") {
        parse(s);
    }

    #[test]
    fn handles_invalid_words(ref s in "([0-9a-z]{1,5} ){3}[0-9a-z]{1,5}") {
        let _ = parse(s);
    }

    #[test]
    fn parses_all_valid_dates(ref s in "([0-2](1st|2nd|3rd|[4-9]th)|10th|20th|30th|31st) of (January) -?(0|[1-9][0-9]{4})") {
        parse(s).unwrap();
    }

    #[test]
    fn parses_date_back_to_original(y in 0i32..10000,
                                    m in 1u32..13, d in 1u32..32) {
        println!("y = {}, m = {}, d = {}", y, m, d);
        let month = match m {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        };

        let suffix = match d {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        let date = parse(&format!("{:02}{} of {} {:02}", d, suffix, month, y));
        prop_assert_eq!(NaiveDate::from_ymd_opt(y, m, d).ok_or(Error::InvalidDate), date);
    }
    }
}
