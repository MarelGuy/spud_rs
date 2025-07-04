use core::{fmt, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    SpudError,
    types::{Date, Time},
};

/// A struct representing a date and time in the format YYYY-MM-DD HH:MM:SS.NS.
/// This struct can be created from chrono's `NaiveDateTime`, and can also be parsed from a string in the same format.
///
/// # Notes
/// - The `NS` (nanoseconds) part is optional. If not provided, it defaults to `0` and won't be displayed when converting to string.
/// - This struct does not handle time zones or daylight saving time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    #[must_use]
    /// Creates a new `DateTime` instance.
    pub fn new(date: Date, time: Time) -> Self {
        DateTime { date, time }
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.date.as_le_bytes();

        bytes.extend_from_slice(&self.time.as_le_bytes());

        bytes
    }
}

impl TryFrom<NaiveDateTime> for DateTime {
    type Error = SpudError;
    fn try_from(date_time: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(DateTime {
            date: Date::try_from(date_time.date())?,
            time: Time::try_from(date_time.time())?,
        })
    }
}

impl FromStr for DateTime {
    type Err = core::fmt::Error;

    /// Parses a string in the format "YYYY-MM-DD HH:MM:SS.NS" into a `DateTime` instance.
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
    type Error = SpudError;

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
