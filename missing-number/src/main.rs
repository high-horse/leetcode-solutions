
#[allow(unused)]
fn main () {
    let nums: Vec<i32> = vec![3,0,1];
    println!("Missing Number {}", missing_number(nums));
    
    let nums: Vec<i32> = vec![1];
    println!("Missing Number {}", missing_number(nums));
    
    let nums: Vec<i32> = vec![1,2];
    println!("Missing Number {}", missing_number(nums));
}
 
 
#[allow(unused)]
pub fn missing_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    
    let end = nums[nums.len()-1];
    let mut index = 0;
    
    let mut missing: Option<i32> = None ;
    
    while index < nums.len()  {
        // println!("Pointer => {}, current => {}", pointer, nums[index]);
        if nums[index] != index as i32 {
            
            missing = Some(index as i32);
            break;
        }
        index += 1;
    }
    match missing {
        Some(value) => value,
        None => nums[nums.len()-1] +1
    }
}


 