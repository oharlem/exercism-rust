use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let b_seconds_base: i64 = 10;
    start + Duration::seconds(b_seconds_base.pow(9))
}
