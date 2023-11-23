use std::collections::HashMap;
struct Solution;
impl Solution {
    fn add_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = result.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                result.insert(i as i32, num);
            }
        }
        vec![]
    }
}

fn main() {
    println!("Hello, world!");
}
