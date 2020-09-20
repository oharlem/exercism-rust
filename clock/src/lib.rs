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
        let mut c = Clock {
            hours,
            minutes: 0,
        };
        let (h, m) = c.get_updated_time(minutes);
        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = self.get_updated_time(minutes);
        Self {
            hours: h,
            minutes: m,
        }
    }

    fn get_updated_time(&self, m2: i32) -> (i32, i32) {
        let total_minutes = &self.minutes + m2;
        let mut m = total_minutes;
        m %= 60;
        if m < 0 {
            m = 60 + m;
        }

        // adjust hours for rolling over minutes
        let h_rollover = total_minutes / 60;
        let mut h = &self.hours + h_rollover;
        if total_minutes < 0 && total_minutes % 60 != 0 {
            h -= 1;
        }

        h %= 24;
        if h < 0 {
            h = 24 + h;
        }

        (h, m)
    }
}
