use crate::grid::{Grid, Pool, Schedule};
use crate::log;
use anyhow::{bail, Result};
use core::fmt::Debug;
use itertools::Itertools;

pub struct EngineParams<Id: Eq + Clone, D> {
    pub seeds: Vec<Grid<Id, D>>,
    pub bound: usize,
    pub pool_list: Vec<Pool<Id, D>>,
}

pub fn engine_main<Id: Eq + Clone + Debug, D: Clone + Debug>(
    params: EngineParams<Id, D>,
) -> Result<Vec<Schedule<Id, D>>> {
    // Check that all seeds are coompatible with one another

    let mut master_schedule = Schedule::<Id, D>::new();

    for s in params.seeds {
        master_schedule.try_merge(&s)?;
    }

    // Generate combinations
    let combinations = params.pool_list.iter().combinations(params.bound);

    let mut valid_schedules = vec![];

    for mut c in combinations {
        stack_main(&mut c, &mut valid_schedules)?;
    }

    Ok(valid_schedules)
}

fn stack_main<K: Eq + PartialEq + Clone + Debug, V: Clone + Debug>(
    combination: &mut Vec<&Pool<K, V>>,
    schedule_list: &mut Vec<Schedule<K, V>>,
) -> Result<()> {
    if let Some(current_stack_level) = combination.pop() {
        for grid in current_stack_level.grid_list.iter() {
            // log!("{:?}", &combination);
            // descend down each grid
            let mut schedule = Schedule::<K, V>::new();

            // this should never error as it's the first schedule merged
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
        // log!("{:?}", &combination);
        // If end_of_stack is true, we reached the end of the stack,
        // therefore any succesfully merged grid at this
        // level is a succesful path and should be cloned and pushed to @schedule_list
        let end_of_stack = combination.len() == 0;

        for grid in current_stack_level.grid_list.iter() {
            if schedule.try_merge(grid).is_ok() {
                if end_of_stack {
                    schedule_list.push(schedule.to_owned());
                    return Some(());
                } else {
                    // Recursive call.
                    if stack_recursive(combination, schedule, schedule_list).is_some() {
                        // Succesful path already cloned into @schedule_list and call stack
                        // is unwinding. Trim last added Grid and continue iteration.
                        schedule.remove_last_added();
                    }
                }
            }
        }

        // Push back stack level

        combination.push(current_stack_level);
    }
    None
}
