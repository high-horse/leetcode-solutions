fm main() {
	println!("Hello workd");
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut hasSeenNumber = nums.clone();
        for &num in nums.iter() {
            if hasSeenNumber[num as usize] == -1 {
                return num;
            }
            hasSeenNumber[num as usize] = -1;
        }
        return -1;
}
