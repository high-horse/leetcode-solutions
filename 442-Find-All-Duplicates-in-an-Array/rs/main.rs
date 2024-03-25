pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut dup_arr: Vec<i32> = vec![];
    let mut num_arr = nums.clone();

    for num in 0..nums.len() {
        let index = num_arr[num].abs() as usize - 1;

        if num_arr[index] < 0 {
            dup_arr.push(index as i32 + 1);
        }
        num_arr[index] *= -1;
    }

    dup_arr
}


fn main() {
        let nums: Vec<i32> = vec![4,3,2,7,8,2,3,1];
        find_duplicates(nums);
}
