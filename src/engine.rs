/**
 * src/engine.rs
 * Copyright (c) 2021 Carlos Carral <carloscarral13@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use crate::grid::{Grid, Pool, Schedule};
use anyhow::{bail, Result};
use core::fmt::Debug;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EngineParams<Id: Eq + Clone, D> {
    /// List of grids that serve as a starting point for the algorithm.
    /// It will try to merge these grids into a valid schedule and then
    /// perform the combinatorial analysis.
    pub seeds: Vec<Grid<Id, D>>,
    /// Number of grids that will conform a schedule.
    pub bound: usize,

    /// List of pools over which the algorithm will perform the
    /// combinatorial analysis.
    pub pool_list: Vec<Pool<Id, D>>,
}

/// Given a list of schedule pools, the engine generates all possible combinations
/// of pools that satisfy an upper bound and finds all valid paths from these pools.
pub fn engine_main<Id: Eq + Clone + Debug, D: Clone + Debug>(
    params: EngineParams<Id, D>,
) -> Result<Vec<Schedule<Id, D>>> {
    if params.pool_list.len() < params.bound {
        bail!("Bound can't be larger than length of pool list.");
    }

    // Check that pools don't have repeated id's
    for (i, pool) in params.pool_list.iter().enumerate() {
        for (j, pool_inner) in params.pool_list.iter().enumerate() {
            if j == i {
                continue;
            }

            if pool_inner.pool_id == pool.pool_id {
                // Found repeated pool, return error
                bail!("Found repeated pool id. Pool id must be unique for any given pool");
            }
        }
    }

    // Check that all seeds are coompatible with one another

    let mut master_schedule = Schedule::<Id, D>::new();

    for s in params.seeds {
        master_schedule.try_merge(&s)?;
    }

    // Generate combinations
    let combinations = params.pool_list.iter().combinations(params.bound);

    let mut valid_schedules = vec![];

    for mut c in combinations {
        stack_main(&mut master_schedule, &mut c, &mut valid_schedules)?;
    }

    Ok(valid_schedules)
}

fn stack_main<K: Eq + PartialEq + Clone + Debug, V: Clone + Debug>(
    master_schedule: &mut Schedule<K, V>,
    combination: &mut Vec<&Pool<K, V>>,
    schedule_list: &mut Vec<Schedule<K, V>>,
) -> Result<()> {
    // Suppose we have a combination of pools  or "stack" {A,B,C} such that
    // A -> [a1, a2, ...]
    // B -> [b1, b2, ...]
    // C -> [c1, c2, ...]
    //
    // (each level is what is refered to as a "stack level")
    //
    // Then all we have to do is find all valid traversals of A->B->C
    // i.e: { [a1,b2,c2], [a2,b1,c1], ...}

    if let Some(current_stack_level) = combination.pop() {
        for grid in current_stack_level.grid_list.iter() {
            // Descend down each grid
            let mut schedule = (*master_schedule).clone();

            // This should never error as it's the first schedule merged
            schedule.try_merge(grid).unwrap();
            stack_recursive(combination, &mut schedule, schedule_list);
        }
    } else {
        bail!("Empty stack error.");
    }

    Ok(())
}

fn stack_recursive<K: Eq + PartialEq + Clone + ToOwned + Debug, V: Clone + ToOwned + Debug>(
    combination: &mut Vec<&Pool<K, V>>,
    schedule: &mut Schedule<K, V>,
    schedule_list: &mut Vec<Schedule<K, V>>,
) -> Option<()> {
    if let Some(current_stack_level) = combination.pop() {
        for grid in current_stack_level.grid_list.iter() {
            if schedule.try_merge(grid).is_ok() {
                if stack_recursive(combination, schedule, schedule_list).is_some() {
                    // Succesful path already cloned into @schedule_list and call stack
                    // is unwinding. Trim last added Grid and continue iteration.
                    schedule.remove_last_added();
                }
            }
        }

        // Push back stack level
        combination.push(current_stack_level);
        return None;
    } else {
        schedule_list.push(schedule.to_owned());
        return Some(());
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_engine() {
        // let schedule = Schedule::new();
        let vec_a1 = [
            "19:00".to_string(),
            "20:30".to_string(),
            "".to_string(),
            "".to_string(),
            "19:00".to_string(),
            "20:30".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        let grid_a1 = Grid::from_vec(1, vec_a1, "%H:%M", 1).unwrap();

        let vec_a2 = [
            "".to_string(),
            "".to_string(),
            "10:00".to_string(),
            "11:30".to_string(),
            "".to_string(),
            "".to_string(),
            "10:00".to_string(),
            "11:30".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let grid_a2 = Grid::from_vec(1, vec_a2, "%H:%M", 2).unwrap();

        let mut pool_a = Pool::new(1);
        pool_a.push(grid_a1);
        pool_a.push(grid_a2);

        let vec_b1 = [
            "13:00".to_string(),
            "15:00".to_string(),
            "".to_string(),
            "".to_string(),
            "13:00".to_string(),
            "15:00".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        let grid_b1 = Grid::from_vec(2, vec_b1, "%H:%M", 1).unwrap();

        let vec_b2 = [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "18:00".to_string(),
            "20:00".to_string(),
            "".to_string(),
            "".to_string(),
            "9:00".to_string(),
            "11:00".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let grid_b2 = Grid::from_vec(2, vec_b2, "%H:%M", 2).unwrap();

        let vec_b3 = [
            "7:00".to_string(),
            "9:00".to_string(),
            "".to_string(),
            "".to_string(),
            "7:00".to_string(),
            "9:00".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let grid_b3 = Grid::from_vec(2, vec_b3, "%H:%M", 3).unwrap();

        let mut pool_b = Pool::new(2);

        pool_b.push(grid_b1);
        pool_b.push(grid_b2);
        pool_b.push(grid_b3);

        let params = EngineParams {
            seeds: vec![],
            bound: 2,
            pool_list: vec![pool_a, pool_b],
        };

        let result = engine_main(params);

        let schedule_list = result.unwrap();

        assert_eq!(schedule_list.len(), 6);
    }
}
