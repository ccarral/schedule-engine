#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use anyhow::Result;
use chrono::NaiveTime;
use schedule_engine::engine;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn instance_grid_1() -> Result<engine::Grid> {
    engine::Grid::new([
        Some((
            NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
        )),
        None,
        None,
        None,
        None,
        None,
        None,
    ])
}

#[wasm_bindgen_test]
fn raises_invalid_time() {
    let grid = engine::Grid::new([
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
    ]);

    assert!(grid.is_err());

    let grid2 = instance_grid_1();

    assert!(grid2.is_ok());
}

#[wasm_bindgen_test]
fn test_free_at() {
    let grid = instance_grid_1().unwrap();
    assert!(!grid.free_at(
        &engine::Day::MONDAY,
        &(
            NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
        )
    ));
    assert!(grid.free_at(
        &engine::Day::TUESDAY,
        &(
            NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
        )
    ));
}
