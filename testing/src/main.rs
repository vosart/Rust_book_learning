//use core::num;

use core::num;
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // let mut map1: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    // let mut result = Vec::new();

    // for (index, curr) in nums.iter().enumerate() {
    //     let needed_elem = target - curr;

    //     if map1.contains_key(&needed_elem) {
    //         result.push(map1[&needed_elem]);
    //         result.push(index as i32);
    //     }
    //     map1.insert(*curr, index as i32);
    // } 
    // result
    // let mut nums: Vec<(usize, i32)> = 
    //     nums.into_iter()
    //         .enumerate()
    //         .collect::<Vec<(usize, i32)>>();
    
    // nums.sort_unstable_by_key(|&(_, n)| n);

    // for (k, (i, ni)) in nums.iter().enumerate() {
    //     match nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) 
    //     {
    //         Ok(jj) => return vec![*i as i32, nums[jj+k+1].0 as i32],
    //         Err(_) => {}
    //     } 
    // }
    // unreachable!("Error: this place should be unreacheble");
    // return vec![0,0];
    // }
        let mut map1: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (index, curr) in nums.into_iter().enumerate() {
            let required = target - curr;

            if map1.contains_key(&required) {
                return vec![map1[&required], index as i32];
            }
            map1.insert(curr, index as i32);
        }
        vec![]
    }

fn main() {
    let nums = vec![2, 11, 15, 4, 0, 23, 7, 11];
    let target = 9;
    println!("{:?}", two_sum(nums, target));
}
