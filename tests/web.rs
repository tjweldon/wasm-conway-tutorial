//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;


#[cfg(test)]
pub fn setup_scenario(length: u32, width: u32, cells: &[(u32, u32)]) -> Universe {
    let mut universe = Universe::new();
    universe.set_height(length);
    universe.set_width(width);
    universe.set_cells(cells);
    universe
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    setup_scenario(6, 6, &[(1,2), (2,3), (3,1), (3,2), (3,3)])
}
#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    setup_scenario(6, 6, &[(2,1), (2,3), (3,2), (3,3), (4,2)])
}

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
pub fn test_setup() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}



#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
pub fn test_count_zero_neighbours() {
    let universe = setup_scenario(5, 5, &[]);

    assert_eq!(universe.count_neighbours(2, 2), 0)
}

#[wasm_bindgen_test]
pub fn test_count_one_horizontal_neighbours() {
    let universe = setup_scenario(5, 5, &[(2, 1)]);

    assert_eq!(universe.count_neighbours(2, 2), 1)
}

#[wasm_bindgen_test]
pub fn test_count_one_vertical_neighbours() {
    let universe = setup_scenario(5, 5, &[(1, 2)]);

    assert_eq!(universe.count_neighbours(2, 2), 1)
}

#[wasm_bindgen_test]
pub fn test_count_one_diagonal_neighbours() {
    let universe = setup_scenario(5, 5, &[(1, 1)]);

    assert_eq!(universe.count_neighbours(2, 2), 1)
}

pub fn test_count_mixed_neighbours() {
    let universe = setup_scenario(5, 5, &[(1, 1), (2, 1), (1, 2)]);

    assert_eq!(universe.count_neighbours(2, 2), 3)
}

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
