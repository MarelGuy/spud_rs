use core::{fmt, str::FromStr};

use chrono::{Datelike, NaiveDate};

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Date {
            year: u16::try_from(date.year()).expect("Invalid year"),
            month: u8::try_from(date.month()).expect("Invalid month"),
            day: u8::try_from(date.day()).expect("Invalid day"),
        }
    }
}

impl FromStr for Date {
    type Err = fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 3 {
            return Err(fmt::Error);
        }

        let year: u16 = u16::from_str(parts[0]).map_err(|_| fmt::Error)?;
        let month: u8 = u8::from_str(parts[1]).map_err(|_| fmt::Error)?;
        let day: u8 = u8::from_str(parts[2]).map_err(|_| fmt::Error)?;

        Ok(Date { year, month, day })
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        NaiveDate::from_ymd_opt(
            i32::from(date.year),
            u32::from(date.month),
            u32::from(date.day),
        )
        .expect("Invalid date conversion")
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
