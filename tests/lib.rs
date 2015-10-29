extern crate sort;

use sort::stable::*;
use sort::unstable::*;

const INIT_ARR: [i32; 7] = [34,2,123,2,35,67,1];
const INIT_ARR_FOR_SLEEP_SORT: [u32; 7] = [352,75,53,234,23,245,35];
const MAX_ELEM: u32 = 352;

// bubble_sort tests
#[test]
fn bubble_sort_all() {
    let mut arr = INIT_ARR;
    bubble_sort(&mut arr);
    assert_eq!(arr, [1,2,2,34,35,67,123]);
}

#[test]
fn bubble_sort_partial() {
    let mut arr = INIT_ARR;
    bubble_sort(&mut arr[1..6]);
    assert_eq!(arr, [34,2,2,35,67,123,1]);
}

#[test]
fn bubble_sort_by_all() {
    let mut arr = INIT_ARR;
    bubble_sort_by(&mut arr, |a, b| a < b);
    assert_eq!(arr, [1,2,2,34,35,67,123]);

    bubble_sort_by(&mut arr, |a, b| b < a);
    assert_eq!(arr, [123,67,35,34,2,2,1]);
}

#[test]
fn bubble_sort_by_partial() {
    let mut arr = INIT_ARR;
    bubble_sort_by(&mut arr[1..6], |a, b| a < b);
    assert_eq!(arr, [34,2,2,35,67,123,1]);

    arr = INIT_ARR;
    bubble_sort_by(&mut arr[1..6], |a, b| b < a);
    assert_eq!(arr, [34,123,67,35,2,2,1]);
}

// heap_sort tests
#[test]
fn heap_sort_all() {
    let mut arr = INIT_ARR;
    heap_sort(&mut arr);
    assert_eq!(arr, [1,2,2,34,35,67,123]);
}

#[test]
fn heap_sort_partial() {
    let mut arr = INIT_ARR;
    heap_sort(&mut arr[1..6]);
    assert_eq!(arr, [34,2,2,35,67,123,1]);
}

#[test]
fn heap_sort_by_all() {
    let mut arr = INIT_ARR;
    heap_sort_by(&mut arr, |a, b| a < b);
    assert_eq!(arr, [1,2,2,34,35,67,123]);

    heap_sort_by(&mut arr, |a, b| b < a);
    assert_eq!(arr, [123,67,35,34,2,2,1]);
}

#[test]
fn heap_sort_by_partial() {
    let mut arr = INIT_ARR;
    heap_sort_by(&mut arr[1..6], |a, b| a < b);
    assert_eq!(arr, [34,2,2,35,67,123,1]);

    arr = INIT_ARR;
    heap_sort_by(&mut arr[1..6], |a, b| b < a);
    assert_eq!(arr, [34,123,67,35,2,2,1]);
}

// sleep_sort tests
#[test]
fn sleep_sort_all() {
    let mut arr = INIT_ARR_FOR_SLEEP_SORT;
    sleep_sort(&mut arr);
    assert_eq!(arr, [23, 35, 53, 75, 234, 245, 352]);
}

#[test]
fn sleep_sort_partial() {
    let mut arr = INIT_ARR_FOR_SLEEP_SORT;
    sleep_sort(&mut arr[1..6]);
    assert_eq!(arr, [352, 23, 53, 75, 234, 245, 35]);
}

#[test]
fn sleep_sort_by_all() {
    let mut arr = INIT_ARR_FOR_SLEEP_SORT;
    sleep_sort_by(&mut arr, |a| *a);
    assert_eq!(arr, [23, 35, 53, 75, 234, 245, 352]);

    sleep_sort_by(&mut arr, |a| MAX_ELEM - *a);
    assert_eq!(arr, [352, 245, 234, 75, 53, 35, 23]);
}

#[test]
fn sleep_sort_by_partial() {
    let mut arr = INIT_ARR_FOR_SLEEP_SORT;
    sleep_sort_by(&mut arr[1..6], |a| *a);
    assert_eq!(arr, [352, 23, 53, 75, 234, 245, 35]);

    arr = INIT_ARR_FOR_SLEEP_SORT;
    sleep_sort_by(&mut arr[1..6], |a| MAX_ELEM - a);
    assert_eq!(arr, [352, 245, 234, 75, 53, 23, 35]);
}

// insertion_sort tests
#[test]
fn insertion_sort_all() {
    let mut arr = INIT_ARR;
    insertion_sort(&mut arr);
    assert_eq!(arr, [1,2,2,34,35,67,123]);
}

#[test]
fn insertion_sort_partial() {
    let mut arr = INIT_ARR;
    insertion_sort(&mut arr[1..6]);
    assert_eq!(arr, [34,2,2,35,67,123,1]);
}

#[test]
fn insertion_sort_by_all() {
    let mut arr = INIT_ARR;
    insertion_sort_by(&mut arr, |a, b| a < b);
    assert_eq!(arr, [1,2,2,34,35,67,123]);

    insertion_sort_by(&mut arr, |a, b| b < a);
    assert_eq!(arr, [123,67,35,34,2,2,1]);
}

#[test]
fn insertion_sort_by_partial() {
    let mut arr = INIT_ARR;
    insertion_sort_by(&mut arr[1..6], |a, b| a < b);
    assert_eq!(arr, [34,2,2,35,67,123,1]);

    arr = INIT_ARR;
    insertion_sort_by(&mut arr[1..6], |a, b| b < a);
    assert_eq!(arr, [34,123,67,35,2,2,1]);
}

// cocktail_sort tests
#[test]
fn cocktail_sort_all() {
    let mut arr = INIT_ARR;
    cocktail_sort(&mut arr);
    assert_eq!(arr, [1,2,2,34,35,67,123]);
}

#[test]
fn cocktail_sort_partial() {
    let mut arr = INIT_ARR;
    cocktail_sort(&mut arr[1..6]);
    assert_eq!(arr, [34,2,2,35,67,123,1]);
}

#[test]
fn cocktail_sort_by_all() {
    let mut arr = INIT_ARR;
    cocktail_sort_by(&mut arr, |a, b| a < b);
    assert_eq!(arr, [1,2,2,34,35,67,123]);

    cocktail_sort_by(&mut arr, |a, b| b < a);
    assert_eq!(arr, [123,67,35,34,2,2,1]);
}

#[test]
fn cocktail_sort_by_partial() {
    let mut arr = INIT_ARR;
    cocktail_sort_by(&mut arr[1..6], |a, b| a < b);
    assert_eq!(arr, [34,2,2,35,67,123,1]);

    arr = INIT_ARR;
    cocktail_sort_by(&mut arr[1..6], |a, b| b < a);
    assert_eq!(arr, [34,123,67,35,2,2,1]);
}
