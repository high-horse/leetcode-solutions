#[allow(unused)]
fn main () {
    let nums: Vec<i32> = vec![3,2,4];
    let target: i32 = 6;
    
    let res: Vec<i32> = two_sum(nums, target);
    println!("{:?}", res);
}
 
#[allow(unused)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let lenth = nums.len();
    
    for (i, item) in nums.iter().enumerate() {
    
        for j in (i+1)..lenth {
            if nums[i] + nums[j] == target {
                vec![i as i32, j as i32 ];
                res.push(i as i32);
                res.push(j as i32);
                
                return res
            }
        }
    }
    res
}

