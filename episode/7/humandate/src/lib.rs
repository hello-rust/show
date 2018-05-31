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
    let month_human = &parts[2];
    let year = parts[3].parse::<i32>()?;
    let (day_str, _suffix) = day_human.split_at(2);

    let day = day_str.parse::<u32>()?;
    let month = match month_human.as_ref() {
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
        "Dezember" => 12,
        _ => unreachable!(),
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
}
