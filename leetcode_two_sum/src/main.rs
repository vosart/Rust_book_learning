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
    let nums = vec![14, 90, 34, 76, 63, 86, 6, 74, 38, 99, 57, 18, 13, 24, 3, 22, 1, 66, 45, 39, 11, 83, 36, 93, 37, 62, 33, 59, 46, 92, 97, 78, 41, 29, 64, 94, 100, 8, 9, 70, 49, 55, 75, 72, 48, 87, 26, 28, 79, 7, 42, 69, 25, 95, 52, 10, 88, 32, 16, 84, 85, 89, 54, 5, 19, 47, 50, 81, 27, 51, 23, 67, 56, 61, 17, 58, 35, 77, 44, 4, 12, 96, 20, 31, 30, 71, 65, 73, 53, 68, 40, 98, 15, 80, 2, 60, 43, 0, 91, 82];
    println!("{:?}", two_sum(nums, 119));
}
