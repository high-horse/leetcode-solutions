fn main() {

	println!("helo world");
}

    pub fn remove_kdigits(num: String, k: i32) -> String {
        let nums_vec: Vec<char> = num.chars().collect();
        let mut result = Vec::new();
        let mut removed = 0;

        for &value in nums_vec.iter() {
            while removed < k && !result.is_empty() && result.last().unwrap() > &value {
                result.pop();
                removed += 1;
            }
            result.push(value);
        }

        // If there are still remaining digits to be removed
        while removed < k {
            result.pop();
            removed += 1;
        }

        // Skip leading zeros
        let mut leading_zeros = 0;
        for &c in &result {
            if c == '0' {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        let final_result: String = result.iter().skip(leading_zeros).collect();
        if final_result == "" {
            return "0".to_string();
        }
        final_result
    }
