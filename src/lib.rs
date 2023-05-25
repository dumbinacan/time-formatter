pub mod format_time;
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::format_time::TimeFormatter;

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
        let test_time = Duration::from_secs(2);
        let result = "2 seconds";
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
        let test_time = Duration::from_secs(3690);
        let result = "1 hour, 1 minute, and 30 seconds";
        assert_eq!(TimeFormatter::new().format(test_time), result);
    }
}