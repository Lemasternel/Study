#!/usr/bin/env rust-script

use std::collections::HashMap;

fn main() {

}

//https://leetcode.com/problems/two-sum?envType=problem-list-v2&envId=02blwr75
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut candidates: HashMap<i32, usize> = HashMap::new(); // Index is the value we need to complete the sum.
                                                                  // Value stores the actual index.
    
        for (index, value) in nums.iter().enumerate() {
            let value_needed = target - value;

            // Check if the value we need is already in the candidates.
            if let Some(&index_matched) = candidates.get(&value_needed) {
                return vec![index_matched as i32, index as i32];
            }

            candidates.insert(*value, index);
        }

        vec![]

}