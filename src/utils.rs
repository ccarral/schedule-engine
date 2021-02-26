use crate::grid::Grid;
use anyhow::Result;
use chrono::NaiveTime;

#[allow(dead_code)]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn instance_grid_1() -> Result<Grid<u32, u32>> {
    Grid::new(
        1,
        [
            Some((
                NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
            )),
            None,
            None,
            None,
            Some((
                NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
            )),
            None,
            None,
        ],
        0,
    )
}

pub fn instance_grid_2() -> Result<Grid<u32, u32>> {
    Grid::new(
        2,
        [
            Some((
                NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("10:00", "%H:%M").unwrap(),
            )),
            None,
            None,
            None,
            Some((
                NaiveTime::parse_from_str("07:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            )),
            None,
            None,
        ],
        0,
    )
}

pub fn instance_grid_3() -> Result<Grid<u32, u32>> {
    Grid::new(
        3,
        [
            Some((
                NaiveTime::parse_from_str("08:30", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("11:00", "%H:%M").unwrap(),
            )),
            None,
            Some((
                NaiveTime::parse_from_str("12:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("13:45", "%H:%M").unwrap(),
            )),
            None,
            None,
            None,
            None,
        ],
        0,
    )
}

pub fn instance_grid_4() -> Result<Grid<u32, u32>> {
    Grid::new(
        3,
        [
            None,
            Some((
                NaiveTime::parse_from_str("16:30", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("18:30", "%H:%M").unwrap(),
            )),
            None,
            Some((
                NaiveTime::parse_from_str("16:30", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("18:30", "%H:%M").unwrap(),
            )),
            None,
            None,
            None,
        ],
        0,
    )
}
pub fn instance_grid_5() -> Result<Grid<u32, u32>> {
    Grid::new(
        3,
        [
            None,
            Some((
                NaiveTime::parse_from_str("18:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("21:00", "%H:%M").unwrap(),
            )),
            None,
            None,
            Some((
                NaiveTime::parse_from_str("07:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
            )),
            None,
            None,
        ],
        0,
    )
}
