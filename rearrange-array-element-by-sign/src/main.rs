#[allow(unused)]
pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut positive: Vec<i32> = vec![];
    let mut negative: Vec<i32> = vec![];
    let mut result: Vec<i32> = vec![];
    
    for item in nums.iter() {
        if *item < 0_i32 {
            negative.push(*item);
        } else {
            positive.push(*item);
        }
    }
    
    let mut i=0;
    let length cargo ();
    while i< length {
        result.push(positive[i]);
        result.push(negative[i]);
        i += 1;
    }
    result
}

#[allow(unused)]
fn main() {
	let nums: Vec<i32> = vec![3,1,-2,-5,2,-4];
	let res = rearrange_array(nums);
	println!("{:?}", res);
	
	let nums: Vec<i32> = vec![-1,1];
	let res = rearrange_array(nums);
	println!("{:?}", res);
}