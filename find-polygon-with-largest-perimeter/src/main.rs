
#[allow(unused)]
fn main () {
    let nums: Vec<i32> = vec![1,12,1,2,5,50,3];
    // let nums: Vec<i32> = vec![5,5,5];
    // let nums: Vec<i32> = vec![5,5,50];
    let res = largest_perimeter(nums);
    println!("{}", res);
}
 
 
#[allow(unused)]
pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    let n = nums.len();
    nums.sort_unstable();

    let mut max_sum = -1;
    let mut sum = nums[0] as i64;
    for i in 1..n {
        let num = nums[i] as i64;
        if sum > num {
            max_sum = max_sum.max(sum + num);
        }
        sum += num;
    }
    max_sum
}



 