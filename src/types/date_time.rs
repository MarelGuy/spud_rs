use core::{fmt, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::types::{Date, Time};

pub struct DateTime {
    date: Date,
    time: Time,
}

impl From<NaiveDateTime> for DateTime {
    fn from(date_time: NaiveDateTime) -> Self {
        DateTime {
            date: Date::from(date_time.date()),
            time: Time::from(date_time.time()),
        }
    }
}

impl FromStr for DateTime {
    type Err = core::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(core::fmt::Error);
        }

        let date = Date::from_str(parts[0])?;
        let time = Time::from_str(parts[1])?;

        Ok(DateTime { date, time })
    }
}

impl From<DateTime> for NaiveDateTime {
    fn from(date_time: DateTime) -> Self {
        NaiveDateTime::new(
            NaiveDate::from(date_time.date),
            NaiveTime::from(date_time.time),
        )
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
