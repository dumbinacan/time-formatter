use std::{
    collections::HashMap,
    ops::SubAssign,
    time::Duration,
};
use itertools::Itertools;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(usize)]
pub enum TimeUnit {
    Nanosecond = 0,
    Microsecond = 1,
    Millisecond = 2,
    Second = 3,
    Minute = 4,
    Hour = 5,
    Day = 6,
}
impl SubAssign<usize> for TimeUnit {
    fn sub_assign(&mut self, rhs: usize) {
        match *self as usize - rhs {
            6 => *self = TimeUnit::Day,
            5 => *self = TimeUnit::Hour,
            4 => *self = TimeUnit::Minute,
            3 => *self = TimeUnit::Second,
            2 => *self = TimeUnit::Millisecond,
            1 => *self = TimeUnit::Microsecond,
            _ => *self = TimeUnit::Nanosecond,
        }
    }
}
const TIMEUNIT_STRING: [&str; 7] = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
];
const TIME_NANOSECOND: [u128; 7] = [
    1,
    1000,
    1000000,
    1000000000,
    60000000000,
    3600000000000,
    3600000000000 * 24,
];

pub struct TimeFormatter {
    max: TimeUnit,
    min: TimeUnit,
}

impl TimeFormatter {
    /// Creates a new TimeFormatter with default values
    pub fn new() -> Self {
        TimeFormatter {
            max: TimeUnit::Hour,
            min: TimeUnit::Millisecond,
        }
    }

    /// Creates a TimeFormatter from given values
    pub fn from(max_timeunit: TimeUnit, min_timeunit: TimeUnit) -> Self {
        let mut confirmed_max: TimeUnit = max_timeunit; // are these really necessary?
        let mut confirmed_min: TimeUnit = min_timeunit;
        if max_timeunit < min_timeunit { // make sure max > min
            confirmed_min = max_timeunit;
            confirmed_max = min_timeunit;
        }

        TimeFormatter {
            max: confirmed_max,
            min: confirmed_min,
        }
    }

    /// change the max timeunit allowed in formatting
    pub fn set_max(&mut self, max_timeunit: TimeUnit) -> bool {
        if max_timeunit > self.min {
            // is max > min?
            self.max = max_timeunit;
            return true;
        }
        false
    }

    /// change the min timeunit allowed in formatting
    pub fn set_min(&mut self, min_timeunit: TimeUnit) -> bool {
        if min_timeunit < self.max {
            // is min < max?
            self.min = min_timeunit;
            return true;
        }
        false
    }

    /// return the String formatted to your specs
    pub fn format(&self, duration: Duration) -> String {
        let mut formatted_times: HashMap<usize, u128> = HashMap::new();
        let mut effective_max: TimeUnit = self.max;

        // find the largest TimeUnit within self.max that can represent duration
        while TIME_NANOSECOND[effective_max as usize] > duration.as_nanos() { effective_max -= 1; }

        let number = duration.as_nanos() / TIME_NANOSECOND[effective_max as usize];
        formatted_times.insert(effective_max as usize, number);

        for i in self.min as usize..effective_max as usize {
            let number = (duration.as_nanos() % TIME_NANOSECOND[i+1]) / TIME_NANOSECOND[i];
            if number > 0 { formatted_times.insert(i, number); }
        }
        let mut formatted_time = String::new();
        if formatted_times.len() == 1 {
            let (k, v) = formatted_times.drain().next().unwrap();
            let mut unit = " ".to_string() + TIMEUNIT_STRING[k];
            if v > 1 { unit = unit + "s"; }
            formatted_time = v.to_string() + unit.as_str();
        } else if formatted_times.len() == 2 {
            // "one unit and an other unit"
            let mut count: usize = 0;
            for (k, v) in formatted_times.drain().sorted().rev() {
                let mut unit = " ".to_owned() + TIMEUNIT_STRING[k];
                if v > 1 { unit += "s"; }
                if count == 0 { unit = unit + " and "}
                formatted_time = formatted_time + v.to_string().as_str() + unit.as_str();
                count += 1;
            }
        } else { // len > 2
            // "one unit, some, other, units, and final unit"
            let mut count: usize = 0;
            let total_units = formatted_times.len() - 1;
            for (k, v) in formatted_times.drain().sorted().rev() {
                let mut unit = " ".to_owned() + TIMEUNIT_STRING[k];
                if v > 1 { unit += "s"; }
                if count < total_units - 1 { unit = unit + ", " }
                if count == total_units - 1 { unit = unit + ", and " }
                formatted_time = formatted_time + v.to_string().as_str() + unit.as_str();
                count += 1;
            }
        }
        formatted_time
    }
} // impl TimeFormatter
