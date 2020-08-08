use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Clock::normalize_mins(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Clock::normalize_mins(self.minutes + minutes),
        }
    }

    fn normalize_mins(minutes: i32) -> i32 {
        minutes.rem_euclid(24 * 60)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.minutes / 60) % 24;
        let minutes = self.minutes % 60;
        write!(f, "{:>02}:{:>02}", hours, minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.minutes)
    }
}
