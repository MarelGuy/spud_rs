use core::fmt::Display;

use chrono::{Datelike, NaiveDate};

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Date {
            year: u16::try_from(date.year()).unwrap(),
            month: u8::try_from(date.month()).unwrap(),
            day: u8::try_from(date.day()).unwrap(),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
