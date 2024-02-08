fn num_squares(n: i32) -> i32 {
    // Create an array to store the minimum number of perfect squares required
    // to sum up to each number from 0 to n
    let mut dp = vec![i32::MAX; (n + 1) as usize];
    
    // Base case: 0 requires 0 perfect squares
    dp[0] = 0;
    
    // Iterate from 1 to n
    for i in 1..=n {
        // Iterate through all possible perfect squares less than or equal to i
        for j in 1..=((i as f64).sqrt() as i32) {
            // Update dp[i] with the minimum of its current value and 1 (perfect square j*j) plus dp[i - j*j]
            dp[i as usize] = dp[i as usize].min(1 + dp[(i - j*j) as usize]);
        }
    }
    
    // The result is stored in dp[n]
    dp[n as usize]
}

fn main() {
    // Example usage:
    let n = 12;
    
    // println!("Minimum number of perfect squares required to sum up to {}: {}", n, num_squares(n));
    let mut dp = vec![i32::MAX; (n + 1) as usize];
    println!("{:?}", dp);
}
