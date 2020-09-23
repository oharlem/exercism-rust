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
        let m = Clock::recalc_time(hours * 60 + minutes);
        Self { minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = Clock::recalc_time(self.minutes + minutes);
        Self { minutes: m }
    }

    fn recalc_time(minutes: i32) -> i32 {
        let h = minutes
            .div_euclid(60)
            .rem_euclid(24);

        let m = minutes
            .rem_euclid(60);

        h * 60 + m
    }
}
