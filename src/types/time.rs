use core::fmt::{Display, Formatter, Result};

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

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:09}",
            self.hour, self.minute, self.second, self.nanosecond
        )
    }
}
