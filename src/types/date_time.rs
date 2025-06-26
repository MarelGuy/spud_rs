use core::{fmt, str::FromStr};
use std::error::Error;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::types::{Date, Time};

pub struct DateTime {
    date: Date,
    time: Time,
}

impl TryFrom<NaiveDateTime> for DateTime {
    type Error = Box<dyn Error>;
    fn try_from(date_time: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(DateTime {
            date: Date::try_from(date_time.date())?,
            time: Time::try_from(date_time.time())?,
        })
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

impl TryFrom<DateTime> for NaiveDateTime {
    type Error = Box<dyn Error>;

    fn try_from(date_time: DateTime) -> Result<Self, Self::Error> {
        Ok(NaiveDateTime::new(
            NaiveDate::try_from(date_time.date)?,
            NaiveTime::try_from(date_time.time)?,
        ))
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
