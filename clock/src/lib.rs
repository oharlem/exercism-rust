use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = minutes % 60;
        if m < 0 {
            m = 60 + m;
        }

        // adjust hours for rolling over minutes
        let h_rollover = minutes / 60;
        let mut h = hours + h_rollover;
        if minutes < 0 && minutes % 60 != 0 {
            h -= 1;
        }

        h %= 24;
        if h < 0 {
            h = 24 + h;
        }

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // recalculate new time
        /*
            let clock = Clock::new(10, 3).add_minutes(-3);
            assert_eq!(clock.to_string(), "10:00");

            let clock = Clock::new(23, 59).add_minutes(2);
            assert_eq!(clock.to_string(), "00:01");
         */

        let mut m = &self.minutes + minutes;
        m %= 60;
        if m < 0 {
            m = 60 + m;
        }

        // adjust hours for rolling over minutes
        let h_rollover = (&self.minutes + minutes) / 60;
        let mut h = self.hours + h_rollover;
        if (&self.minutes + minutes) < 0 && (&self.minutes + minutes) % 60 != 0 {
            h -= 1;
        }

        h %= 24;
        if h < 0 {
            h = 24 + h;
        }

        Self {
            hours: h,
            minutes: m,
        }
    }
}
