use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // calculate hours
        // calculate minutes
        // show with padding
        let mut h = &self.minutes / 60;
        h = h % 24;
        let m = &self.minutes % 60;
        write!(f, "{:02}:{:02}", h, m)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: &self.minutes + minutes,
        }
    }
}
