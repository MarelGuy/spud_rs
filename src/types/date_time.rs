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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_creation() {
        let date: Date = Date::new(2023, 3, 15).unwrap();
        let time: Time = Time::new(12, 30, 45, 500_000_000).unwrap();

        let datetime = DateTime::new(date, time);

        assert_eq!(datetime.date, date);
        assert_eq!(datetime.time, time);
    }

    #[test]
    fn test_datetime_from_naive_date() {
        let naive_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
        let naive_time: NaiveTime = NaiveTime::from_hms_nano_opt(12, 30, 45, 500_000_000).unwrap();
        let naive_datetime: NaiveDateTime = NaiveDateTime::new(naive_date, naive_time);

        let datetime: Result<DateTime, SpudError> = DateTime::try_from(naive_datetime);

        assert!(datetime.is_ok());
        assert_eq!(
            datetime.unwrap().to_string(),
            "2023-03-15 12:30:45.500000000"
        );
    }

    #[test]
    fn test_datetime_from_str() {
        let datetime_str: &str = "2023-03-15 12:30:45.500000000";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(datetime_str);

        assert!(datetime.is_ok());
        assert_eq!(datetime.unwrap().to_string(), datetime_str);
    }

    #[test]
    fn test_datetime_from_str_invalid() {
        let invalid_str: &str = "2023-13-15 12:30:45";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-02-30 12:30:45";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-03-15 25:00:00";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-03-15 12:60:00";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-03-15 12:30:60";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-03-15 12:30:45.1000000000";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());

        let invalid_str: &str = "2023-03-15 12:30";
        let datetime: Result<DateTime, fmt::Error> = DateTime::from_str(invalid_str);

        assert!(datetime.is_err());
    }

    #[test]
    fn test_datetime_to_naive_date_time() {
        let date: Date = Date::new(2023, 3, 15).unwrap();
        let time: Time = Time::new(12, 30, 45, 500_000_000).unwrap();

        let datetime = DateTime::new(date, time);
        let naive_datetime: Result<NaiveDateTime, SpudError> = NaiveDateTime::try_from(datetime);

        assert!(naive_datetime.is_ok());
        assert_eq!(
            naive_datetime.unwrap().to_string(),
            "2023-03-15 12:30:45.500"
        );
    }
}
