use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Clock { hours, minutes: 0 };
        c.recalculate_time(minutes);
        c
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut c = Clock {
            hours: self.hours,
            minutes: self.minutes,
        };
        c.recalculate_time(minutes);
        c
    }

    fn recalculate_time(&mut self, minutes: i32) {
        let total_minutes = &self.minutes + minutes;
        self.minutes = total_minutes;
        self.minutes %= 60;
        if self.minutes < 0 {
            self.minutes += 60;
        }

        // adjust hours for rolling over minutes
        self.hours += total_minutes / 60;
        if total_minutes < 0 && total_minutes % 60 != 0 {
            self.hours -= 1;
        }

        self.hours %= 24;
        if self.hours < 0 {
            self.hours += 24;
        }
    }
}
