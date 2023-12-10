use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut index_value_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    for (index, current) in nums.iter().enumerate() {
        let required = target - current;

        if index_value_map.contains_key(&required) {
            return vec![index_value_map[&required], index as i32];
        }
        index_value_map.insert(*current, index as i32);
    }
    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 3, 5, 7, 1], 9));
}
