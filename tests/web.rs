#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use anyhow::Result;
use chrono::NaiveTime;
use schedule_engine::{engine::*, grid::*, log, utils::*};
use std::collections::hash_map::HashMap;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn raises_invalid_time() {
    let grid = Grid::new(
        0,
        [
            Some((
                NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
                NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            )),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        0,
    );

    assert!(grid.is_err());

    let grid2 = instance_grid_1();

    assert!(grid2.is_ok());
}

#[wasm_bindgen_test]
fn test_free_at() {
    let grid = instance_grid_1().unwrap();
    assert!(!grid.free_at(
        &Day::MONDAY,
        &(
            NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
        )
    ));
    assert!(grid.free_at(
        &Day::TUESDAY,
        &(
            NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
        )
    ));
}

#[wasm_bindgen_test]
fn test_try_merge() {
    let grid_1 = instance_grid_1().unwrap();
    let grid_2 = instance_grid_2().unwrap();
    let grid_3 = instance_grid_3().unwrap();

    let mut schedule = Schedule::new();
    schedule.try_merge(&grid_1);

    assert!(schedule.try_merge(&grid_3).is_err());
    assert!(schedule.try_merge(&grid_2).is_ok());
    assert!(schedule.try_merge(&grid_2).is_err());

    let grid_4 = instance_grid_4().unwrap();
    let grid_5 = instance_grid_5().unwrap();

    let mut schedule_2 = Schedule::new();
    schedule_2.try_merge(&grid_5);
    assert!(schedule_2.try_merge(&grid_4).is_err());
}

#[wasm_bindgen_test]
fn test_from_vec() {
    let grid_1 = instance_grid_1().unwrap();
    let grid_2 = Grid::from_vec(
        2,
        [
            "08:00".to_string(),
            "09:00".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "08:00".to_string(),
            "09:00".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ],
        "%H:%M",
        0,
    )
    .unwrap();

    assert_eq!(grid_1.time_values, grid_2.time_values);
}

#[wasm_bindgen_test]
fn test_deserialize() {
    let time_values = [None; 7];
    let mut data = std::collections::hash_map::HashMap::<String, String>::new();

    data.insert("professor".to_string(), "Stephen Hawking".to_string());
    let mut grid = Grid::new(0, time_values, data).unwrap();
    let serialized = JsValue::from_serde(&grid).unwrap();

    log!("{:?}", grid);
    log!("{:?}", serialized);
}
