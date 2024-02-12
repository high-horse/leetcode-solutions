fn main() {
    println!("{}", is_palindrome(121_i32) );
    println!("{}", is_palindrome(1121_i32) );
}


pub fn is_palindrome(x: i32) -> bool {
    let num_str: Vec<char> = x.to_string().chars().collect();

    let mut last_ind = num_str.len() -1;
    for (index, ch) in num_str.iter().enumerate() {

        if num_str[index] != num_str[last_ind] {
            return false;
        }
        last_ind -= 1;
    }

    true
}