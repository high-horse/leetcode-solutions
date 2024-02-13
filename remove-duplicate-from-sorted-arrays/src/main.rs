#[allow(unused)]
fn main () {
    let mut data_arr: Vec<i32> = vec![1,1,2];
    let ans = remove_duplicates(&mut data_arr);
    println!("{ans}");
}
 
#[allow(unused)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let mut copy: Vec<i32> = vec![];
    
    for item in nums.iter() {
        if !copy.contains(item) {
            copy.push(*item);
        }
    }
    
    nums.clear();
    for item in copy.iter() {
        nums.push(*item);
    } 
    
    copy.len() as i32
}