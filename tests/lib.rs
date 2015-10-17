extern crate sort;

use sort::stable::*;
use sort::unstable::*;

const INIT_ARR: [i32;7] = [34,2,123,2,35,67,1];

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
    bubble_sort_by(&mut arr, |a, b| a.cmp(b));
    assert_eq!(arr, [1,2,2,34,35,67,123]);

    bubble_sort_by(&mut arr, |a, b| b.cmp(a));
    assert_eq!(arr, [123,67,35,34,2,2,1]);
}

#[test]
fn bubble_sort_by_partial() {
    let mut arr = INIT_ARR;
    bubble_sort_by(&mut arr[1..6], |a, b| a.cmp(b));
    assert_eq!(arr, [34,2,2,35,67,123,1]);

    arr = INIT_ARR;
    bubble_sort_by(&mut arr[1..6], |a, b| b.cmp(a));
    assert_eq!(arr, [34,123,67,35,2,2,1]);
}
