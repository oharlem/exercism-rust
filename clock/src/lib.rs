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
        let minutes = Clock::recalc_to_minutes(hours, minutes);
        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    fn recalc_to_minutes(hours: i32, minutes: i32) -> i32 {
        let carry_hours = minutes.div_euclid(60);
        let m = minutes.rem_euclid(60);
        let h = (hours + carry_hours).rem_euclid(24);

        h * 60 + m
    }
}
