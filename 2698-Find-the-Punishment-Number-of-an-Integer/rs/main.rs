impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut answer = 0;
        for i in 1..=n {
            answer += Self::calculate_punishment(i);
        }
        answer
    }

    fn calculate_punishment(n: i32) -> i32 {
        let sq = n * n;
        let sq_str = sq.to_string();

        let partitions = Self::generate_partitions(&sq_str);

        // Check each partition and calculate its sum
        for partition in partitions {
            let mut sum = 0;
            for part in partition {
                if let Ok(num) = part.parse::<i32>() {
                    sum += num;
                }
            }

            // If the sum matches n, return the square
            if sum == n {
                return sq;
            }
        }

        0
    }

    fn generate_partitions(s: &str) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut partition = Vec::new();
        Self::helper(s, 0, &mut partition, &mut result);
        result
    }

    fn helper(s: &str, start: usize, partition: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if start == s.len() {
            result.push(partition.clone());
            return;
        }

        // Loop to generate all partitions
        for i in start + 1..=s.len() {
            let substr = &s[start..i];
            partition.push(substr.to_string());
            Self::helper(s, i, partition, result);
            partition.pop(); // Backtrack
        }
    }
}
