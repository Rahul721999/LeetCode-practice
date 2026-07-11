#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Typical case, one duplicate
    let nums = vec![1, 3, 4, 2, 2];
    let res = Solution::find_duplicate(nums);
    assert_eq!(res, 2);
}

#[test]
fn test2() {
    // Duplicate is the smallest value
    let nums = vec![1, 1, 2, 3, 4, 5];
    let res = Solution::find_duplicate(nums);
    assert_eq!(res, 1);
}

#[test]
fn test3() {
    // Duplicate is the largest value
    let nums = vec![5, 3, 4, 2, 5, 1];
    let res = Solution::find_duplicate(nums);
    assert_eq!(res, 5);
}
