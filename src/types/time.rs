use core::{fmt, str::FromStr};

use chrono::{NaiveTime, Timelike};

pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl From<NaiveTime> for Time {
    fn from(time: NaiveTime) -> Self {
        Time {
            hour: u8::try_from(time.hour()).expect("hour out of range"),
            minute: u8::try_from(time.minute()).expect("minute out of range"),
            second: u8::try_from(time.second()).expect("second out of range"),
            nanosecond: time.nanosecond(),
        }
    }
}

impl FromStr for Time {
    type Err = core::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 && parts.len() != 4 {
            return Err(core::fmt::Error);
        }

        let hour: u8 = u8::from_str(parts[0]).map_err(|_| core::fmt::Error)?;
        let minute: u8 = u8::from_str(parts[1]).map_err(|_| core::fmt::Error)?;
        let second: u8 = u8::from_str(parts[2]).map_err(|_| core::fmt::Error)?;
        let nanosecond = if parts.len() == 4 {
            u32::from_str(parts[3]).map_err(|_| core::fmt::Error)?
        } else {
            0
        };

        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }
}

impl From<Time> for NaiveTime {
    fn from(time: Time) -> Self {
        NaiveTime::from_hms_nano_opt(
            u32::from(time.hour),
            u32::from(time.minute),
            u32::from(time.second),
            time.nanosecond,
        )
        .expect("Invalid time conversion")
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.nanosecond == 0 {
            write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        } else {
            write!(
                f,
                "{:02}:{:02}:{:02}.{:09}",
                self.hour, self.minute, self.second, self.nanosecond
            )
        }
    }
}
