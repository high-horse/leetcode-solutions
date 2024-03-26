fn main() {
    let nums : Vec<i32> = vec![7,8,9,11,12];
    println!("{}", first_missing_positive(nums));
}

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    
    for i in 0..n {
        while nums[i] >= 1 && nums[i] <= n as i32 && nums[i] != nums[(nums[i]-1) as usize] {
            let temp = nums[i];
            nums[i] = nums[(nums[i]-1) as usize];
            nums[(temp-1) as usize] = temp;
        }
    }
    for i in 0..n {
        if nums[i] != (i+1) as i32 {
            return (i + 1) as i32;
        }
    }
    (n+1) as i32
}

