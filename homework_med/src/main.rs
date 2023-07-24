use std::collections::HashMap;

fn average(nums: &[i32]) -> f32 {
    let sum: i32 = nums.iter().sum();
    let length = nums.len();
    sum as f32 / length as f32
    //nums.iter().sum::<i32>() as f32 / nums.len() as f32
}
fn median(nums: &mut [i32]) -> f32 {
    nums.sort();
    if (nums.len() % 2) == 0 {
        let ind_left = nums.len() / 2 + 1;
        let ind_right = nums.len() / 2;
        ((nums[ind_left] + nums[ind_right]) / 2) as f32
    } else {
        let mid = nums.len() / 2;
        nums[mid] as f32
    }
}
fn moda(nums: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute!")
}


fn main() {
    let mut seq = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43, 11018];
    
    println!("AVERAGE: {}", average(&seq));
    println!("MEDIAN: {}", median(&mut seq));
    println!("MODA: {}", moda(&seq));
    
}
