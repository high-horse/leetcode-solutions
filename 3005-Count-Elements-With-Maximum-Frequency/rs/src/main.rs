fn main() {
    println!("Hello, world!");
}


use std::collections::HashMap;

    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for val in nums {
            let count = map.entry(val).or_insert(0);
            *count +=1;
        }

        let mut count = 0;
        let mut max = -1;

        for &freq in map.values() {
            max = max.max(freq);
        }

        for &freq in map.values() {
            if freq == max {
                count += max;
            }
        }

        count
    }
