// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::num;

const EXPECTED_TIME: i32 = 40;
const LAYER_TIME: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    return EXPECTED_TIME
}`

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    return number_of_layers * LAYER_TIME
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
