use anyhow::{bail, Result};
use chrono::NaiveTime;

struct Teacher {}

type TimeRange = (NaiveTime, NaiveTime);

pub struct Grid {
    time_values: [Option<TimeRange>; 7],
}

impl Grid {
    pub fn new(time_values: [Option<TimeRange>; 7]) -> Result<Self> {
        // Validate time ranges
        for time_val in time_values.iter() {
            if let &Some((v1, v2)) = time_val {
                if v1.gt(&v2) {
                    bail!("Invalid time range: {:?} - {:?}", v1, v2);
                }
            }
        }
        Ok(Grid { time_values })
    }

    pub fn free_at(&self, day: &Day, time_range: &TimeRange) -> bool {
        match self.time_values[*day as usize] {
            Some((my_start, my_end)) => {
                let (start, end) = time_range;
                my_start.lt(start) && (my_end.lt(end) || my_end.eq(end))
                    || (my_start.eq(end) || my_start.gt(end))
            }
            None => true,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Day {
    MONDAY = 0,
    TUESDAY = 1,
    WEDNESDAY = 2,
    THURSDAY = 3,
    FRIDAY = 4,
    SATURDAY = 5,
    SUNDAY = 6,
}

pub struct Schedule {
    grid: Grid,
    teacher: Teacher,
    id: String,
    subject: Subject,
}

pub struct Subject {
    name: String,
    subject_id: String,
}
