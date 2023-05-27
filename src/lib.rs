pub mod format_time;
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::format_time::TimeFormatter;
    use crate::format_time::TimeUnit;
    const SECOND: u64 = 1;
    const MINUTE: u64 = SECOND * 60;
    const HOUR:   u64 = MINUTE * 60;
    const DAY:    u64 = HOUR   * 24;

    #[test]
    fn nanosec() {
        let test_time = Duration::from_nanos(1);
        let result = "1 nanosecond";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn millisec() {
        let test_time = Duration::from_millis(1);
        let result = "1 millisecond";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn microsec() {
        let test_time = Duration::from_micros(1);
        let result = "1 microsecond";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn second() {
        let test_time = Duration::from_secs(1);
        let result = "1 second";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn minute() {
        let test_time = Duration::from_secs(60);
        let result = "1 minute";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn one_and_a_half_minutes() {
        let test_time = Duration::from_secs(90);
        let result = "1 minute and 30 seconds";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn one_hour_one_minute_and_thirty_seconds() {
        let test_time = Duration::from_secs(HOUR) + Duration::from_secs(MINUTE) + Duration::from_secs(30 * SECOND);
        let result = "1 hour, 1 minute, and 30 seconds";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }

    #[test]
    fn three_days_two_hours_one_minute_and_thirty_seconds() {
        let test_time = Duration::from_secs(3 * DAY) + Duration::from_secs(2 * HOUR) + Duration::from_secs(MINUTE) + Duration::from_secs(30 * SECOND);
        let result = "3 days, 2 hours, 1 minute, and 30 seconds";
        assert_eq!(TimeFormatter::from(TimeUnit::Day, TimeUnit::Nanosecond).format(test_time), result);
    }
}
