use std::collections::HashMap;
#[allow(unused)]
fn main () {
    let nums: Vec<i32>= vec![4,3,1,1,3,3,2];
    let k: i32 = 3;
    let nums: Vec<i32>= vec![5,5,4];
    let k = 1;
    let res = find_least_num_of_unique_ints(nums, k);
}
 
#[allow(unused)]
pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let dup_arr: Vec<i32> = arr;
    let length = dup_arr.len();
    let mut count_pointer : HashMap<i32, i32> = HashMap::new();
    let mut i=0;
    while i < length{
        if let Some(value) = count_pointer.get_mut(&dup_arr[i]) {
            *value += 1;
        } else {
            count_pointer.insert(dup_arr[i], 1);
        }
        i+=1;
    }
    let mut values: Vec<i32> = count_pointer.values().cloned().collect();
    println!("{:?}", values);
    values.sort();
    println!("{:?}", values);
    let result = consume_vector(values.clone(), k);
    println!("{:?}", result);
    
    let mut count: i32 = 0;
    for item in result{
        if item != 0 {
            count +=1;
        }
    }
    println!("{:?}", count);
    count
    // todo!()
}
 
#[allow(unused)]
fn consume_vector(mut vec: Vec<i32>, num: i32) -> Vec<i32> {
    let mut sum = 0;
    for i in 0..vec.len() {
        if sum + vec[i] <= num {
            sum += vec[i];
            vec[i] = 0;
        } else if sum + vec[i] > num && sum < num {
            let diff = num - sum;
            vec[i] -= diff;
            sum += diff;
        }
        if sum >= num {
            break;
        }
    }
    vec
}