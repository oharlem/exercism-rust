use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = self.minutes / 60;
        let m = self.minutes % 60;

        write!(f, "{:02}:{:02}", h, m)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = Clock::recalculate_time(hours * 60 + minutes);
        Self { minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = Clock::recalculate_time(self.minutes + minutes);
        Self { minutes: m }
    }

    fn recalculate_time(minutes: i32) -> i32 {
        let mut h = minutes / 60;

        // adjust hours for rolling over minutes
        if minutes < 0 && minutes % 60 != 0 {
            h -= 1;
        }

        h %= 24;
        if h < 0 {
            h += 24;
        }

        let mut m = minutes;
        m %= 60;
        if m < 0 {
            m += 60;
        }

        h * 60 + m
    }
}
