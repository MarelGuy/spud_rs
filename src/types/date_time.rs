use core::fmt::{Display, Formatter, Result};

use chrono::NaiveDateTime;

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

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
