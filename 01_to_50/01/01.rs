// Two Sum: 01 Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

use std::collections::HashMap;

fn main() {
   println!("{:?}",sum_to_target([1,2,3,4].to_vec(), 6)); 
}

fn sum_to_target (nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut complement: i32;

    for (index, number) in nums.into_iter().enumerate() {
        complement = target - number;
        if map.contains_key(&number) {
            return [index as i32, *map.get(&number).unwrap()].to_vec();
        } else {
            map.insert(complement, index as i32);
        
        }
    };
    unreachable!();
}
