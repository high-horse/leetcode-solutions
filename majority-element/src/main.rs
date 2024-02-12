use std::collections::HashMap;
 
#[allow(unused)]
pub fn majority_element_alternative(nums: Vec<i32>) -> i32 {
    let mut store: HashMap<i32, i32> = HashMap::new();
    for item in nums.iter(){
        if let Some(value) = store.get_mut(item) {
            *value += 1;
            if *value > ( nums.len() as i32 )/2{
                return *item as i32;
            }
        } else {
            store.insert(*item, 1);
        }
    }
    let mut max_val = std::i32::MIN;
    let mut last_count: i32 = 0;
    for (key, val) in store.into_iter() {
        if last_count < val {
            max_val = key;
            // println!("{} {}", key, val);    
        }
    }
    max_val
}
 
#[allow(unused)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut store: HashMap<i32, i32> = HashMap::new();
    for item in nums.iter(){
        if let Some(value) = store.get_mut(item) {
            *value += 1;
            if *value > ( nums.len() as i32 )/2{
                return *item as i32;
            }
        } else {
            store.insert(*item, 1);
        }
    }
    let mut max_val: i32 = std::i32::MIN;
    if let Some((&key, _)) = store.iter().max_by_key(|&(_, val)| val) {
        max_val = key;
    }
    max_val
}
 
#[allow(unused)]
fn main() {
    let  nums: Vec<i32> = vec![2,2,1,1,1,2,2];
    let res = majority_element(nums);
    println!("{}", res);
}