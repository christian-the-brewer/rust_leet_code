// 217 Contains Duplicate 
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

use std::collections::HashSet;

fn main() {
   println!("{}",hash_the_set([1,2,3,4,3].to_vec())); 
}

//brute forcing it with nested loop
fn brute_force(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if &nums[j] == &nums[i] {
               return true;
            }
        }
    }
    false 
}

//sorting vector and then comparing each number to its neighbor
fn sort_first(nums: Vec<i32>) -> bool {
    //sort
    false
}

//using hashset to check if value has already been added to set
fn hash_the_set(nums: Vec<i32>) -> bool {
    let mut hash_set: HashSet<i32> = HashSet::new();

    for num in nums {
        if !hash_set.insert(num) {
            return true;
        }
    }
    false
}
