use anyhow::{bail, Result};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type TimeRange = (NaiveTime, NaiveTime);
pub type GridList<K, V> = Vec<Grid<K, V>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid<Id: Eq, D> {
    pub time_values: [Option<TimeRange>; 7],
    /// Used for checking if a schedule belongs to a pool
    pub pool_id: Id,
    /// Contains other info about a particular schedule instance
    /// i.e: { "teacher": "Stephen Hawking", "subject": "General Relativity 101"}
    data: D,
}

impl<Id, D> Grid<Id, D>
where
    Id: Eq,
{
    pub fn new(pool_id: Id, time_values: [Option<TimeRange>; 7], data: D) -> Result<Self> {
        // Validate time ranges
        for time_val in time_values.iter() {
            if let &Some((v1, v2)) = time_val {
                if v1.gt(&v2) {
                    bail!("Invalid time range: {:?} - {:?}", v1, v2);
                }
            }
        }
        Ok(Grid {
            pool_id,
            data,
            time_values,
        })
    }

    pub fn from_vec(id: Id, time_values_vec: [String; 14], fmt: &str, data: D) -> Result<Self> {
        // This function assumes time_values.len() == 14
        // and that begin-end times are ordered as such:
        // | MON | TUE | WED | THU | FRI | SAT | SUN |
        // |0 | 1|2 | 3|4 | 5|6 | 7|8 | 9|10|11|12|13|
        // where two empty strings mean no time range on that day.

        let mut time_values: [Option<TimeRange>; 7] = [None; 7];

        if time_values_vec.len() != 14 {
            bail!("Invalid slice length.");
        }

        let mut j = 0;
        for i in (0..14).step_by(2) {
            let (begin, end) = (&time_values_vec[i], &time_values_vec[i + 1]);
            if begin.is_empty() || end.is_empty() {
                if begin.is_empty() && end.is_empty() {
                    // No events planned on that day
                    j += 1;
                    continue;
                } else {
                    // Slice is malformed
                    dbg!(time_values_vec);
                    bail!("Unable to parse slice");
                }
            } else {
                time_values[j] = Some((
                    NaiveTime::parse_from_str(begin, fmt)?,
                    NaiveTime::parse_from_str(end, fmt)?,
                ));
                j += 1;
            }
        }

        Self::new(id, time_values, data)
    }

    pub fn free_at(&self, day: &Day, time_range: &TimeRange) -> bool {
        /* When checking if a schedule is compatible on a given day,
         * assuming that the time range is well formed,
         *  (start < end),
         * we must ensure the following:
         *
         * with (my_start, my_end) and (other_start,other_end)
         *
         * it should either be that
         *
         * my_end <= other_start
         *
         * i.e : (07:00, 08:00) is compatible with (08:00,10:00)
         *
         * or
         *
         * my_start >= other_end
         *
         * i.e: (07:00, 08:00) is compatible with (06:00,07:00)
         *
         *
         * */

        match self.time_values[*day as usize] {
            Some((my_start, my_end)) => {
                let (start, end) = time_range;
                my_end.le(start) || my_start.ge(end)
            }
            None => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool<Id: Eq, D> {
    pub grid_list: Vec<Grid<Id, D>>,
    pub pool_id: Id,
}

impl<Id, D> Pool<Id, D>
where
    Id: Eq,
{
    pub fn new(pool_id: Id) -> Self {
        Pool {
            grid_list: vec![],
            pool_id,
        }
    }

    pub fn push(&mut self, grid: Grid<Id, D>) {
        self.grid_list.push(grid);
    }
}

#[derive(Copy, Clone, EnumIter)]
pub enum Day {
    MONDAY = 0,
    TUESDAY = 1,
    WEDNESDAY = 2,
    THURSDAY = 3,
    FRIDAY = 4,
    SATURDAY = 5,
    SUNDAY = 6,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule<Id: Eq + Clone + ToOwned, D: Clone + ToOwned> {
    grids: Vec<Grid<Id, D>>,
}

impl<Id, D> Schedule<Id, D>
where
    Id: Eq + PartialEq + Clone + ToOwned,
    D: Clone + ToOwned,
{
    pub fn new() -> Self {
        Schedule::<Id, D> { grids: vec![] }
    }

    pub fn try_merge(&mut self, grid: &Grid<Id, D>) -> Result<()> {
        for day in Day::iter() {
            for grid_inner in &self.grids {
                if let Some(time_values) = grid_inner.time_values[day as usize] {
                    if !grid.free_at(&day, &time_values) {
                        bail!("Error: Conflicting schedules");
                    }
                }
            }
        }

        self.grids.push((*grid).clone());

        Ok(())
    }

    pub fn remove_last_added(&mut self) -> Option<Grid<Id, D>> {
        self.grids.pop()
    }
}

#[cfg(test)]
mod test {

    use crate::grid::Schedule;
    use crate::utils::*;

    #[test]
    fn test_try_merge() {
        let grid_4 = instance_grid_4().unwrap();
        let grid_5 = instance_grid_5().unwrap();

        let mut schedule_2 = Schedule::new();
        schedule_2.try_merge(&grid_5);
        assert!(schedule_2.try_merge(&grid_4).is_err());
    }
}
