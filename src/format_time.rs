use std::{
    collections::HashMap,
    time::Duration,
};
use lazy_static::lazy_static;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(usize)]
pub enum TimeUnit {
    Nanosecond  = 0,
    Microsecond = 1,
    Millisecond = 2,
    Second      = 3,
    Minute      = 4,
    Hour        = 5,
    Day         = 6,
}
const TIMEUNIT_STRING: [&str; 7] = ["Nanosecond", "Microsecond", "Millisecond", "Second", "Minute", "Hour", "Day"];
const TIME_NANOSECOND: [u128; 7] = [1, 1000, 1000000, 1000000000, 60000000000, 3600000000000, 3600000000000 * 24];
/* something to consider if I want to avoid as usize all the time
lazy_static! {
    static ref TIME_IN_NANOSECONDS: HashMap<TimeUnit, u128> = HashMap::from([
        (TimeUnit::Nanosecond, 1),
        (TimeUnit::Microsecond, 1000),
        (TimeUnit::Millisecond, 1000000),
        (TimeUnit::Second,      1000000000),
        (TimeUnit::Minute, 1000000000 * 60),     // Minute = (Second * 60)
        (TimeUnit::Hour, 1000000000 * 60 * 60),    // Hour = (Minute * 60)
        (TimeUnit::Day, 1000000000 * 60 * 60 * 24), // Day = (Hour * 24)
    ]);
}
*/ 

pub struct TimeFormatter {
    max: TimeUnit,
    min: TimeUnit,
}

impl TimeFormatter {

    /// Creates a new TimeFormatter with default values
    pub fn new() -> Self {
        TimeFormatter {
            max: TimeUnit::Hour,
            min:  TimeUnit::Millisecond,
        }
    }

    /// Creates a TimeFormatter from given values
    pub fn from(max_timeunit: TimeUnit, min_timeunit: TimeUnit) -> Self {
        let mut confirmed_max: TimeUnit = max_timeunit; // are these really necessary?
        let mut confirmed_min: TimeUnit = min_timeunit;
        if max_timeunit < min_timeunit {
            confirmed_min = max_timeunit;
            confirmed_max = min_timeunit;
        }

        TimeFormatter {// make sure max > min
            max: confirmed_max,
            min: confirmed_min,
        }
    }

    /// change the max timeunit allowed in formatting
    pub fn set_max(&mut self, max_timeunit: TimeUnit) -> bool {
        if max_timeunit > self.min { // is max > min?
            self.max = max_timeunit;
            return true
        }
        false
    }

    /// change the min timeunit allowed in formatting
    pub fn set_min(&mut self, min_timeunit: TimeUnit) -> bool {
        if min_timeunit < self.max { // is min < max?
            self.min = min_timeunit;
            return true
        }
        false
    }

    // TODO
    // Options for preferred largest measuring unit and
    // levels of precision or smallest measuring unit "4 days 6 hours 20 seconds and 14 microseconds"
    pub fn format(&self, duration: Duration) -> String {
        let mut formatted_time: HashMap<TimeUnit, String> = HashMap::new();  

        for i in (self.min as usize + 1..=self.max as usize).rev() {

            let mut number: u128 = duration.as_nanos();
            let unit = String::from(TIMEUNIT_STRING[i as usize]) + "s";

            if i == self.max as usize {
                number /= TIME_NANOSECOND[i as usize]; 
            }
            else { number %= TIME_NANOSECOND[i as usize]; }
            // TODO
            // HashMap <TimeUnit, String>  storing the value for each TimeUnit
            if number > 0 { formatted_time.fuck_you( &format!("{} {}, ", number, unit)); }
        }

        // need a form to check which will be the last format! call so it will have and .
        let number = duration.as_nanos() % TIME_NANOSECOND[self.min as usize];
        let unit = String::from(TIMEUNIT_STRING[self.min as usize]) + "s";
        if number > 0 { formatted_time += &format!("and {} {}.", number, unit); }
        
        formatted_time
    }

} // impl TimeFormatter
