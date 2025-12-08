use std::fmt::{self, Debug};
use std::fmt::{Display, Result};

pub struct Clock {
    pub minutes: i32,
}

// сутки 1440 минум
impl Clock {
    pub fn new(hour: i32, minute: i32) -> Self {
        Self {
            minutes: Clock::normolize(minute + hour * 60),
        }
    }

    pub fn normolize(minutes: i32) -> i32 {
        minutes.rem_euclid(24 * 60)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Clock::normolize(self.minutes + minutes),
        }
    }

    pub fn get_as_collection(&self) -> (i32, i32) {
        let hour: i32 = &self.minutes / 60;
        let minute: i32 = &self.minutes % 60;

        (hour, minute)
    }

    pub fn fmt_hour(hour: i32) -> String {
        if hour < 10 {
            format!("0{hour}")
        } else {
            format!("{hour}")
        }
    }

    pub fn fmt_minute(minute: i32) -> String {
        if minute < 10 {
            format!("0{minute}")
        } else {
            format!("{minute}")
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        let (hour, minute) = self.get_as_collection();

        write!(f, "{}:{}", Clock::fmt_hour(hour), Clock::fmt_minute(minute))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let (hour, minute) = self.get_as_collection();
        let (hour_o, minute_o) = other.get_as_collection();
        hour == hour_o && minute == minute_o
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        let (hour, minute) = self.get_as_collection();

        f.debug_tuple("").field(&hour).field(&minute).finish()
    }
}
