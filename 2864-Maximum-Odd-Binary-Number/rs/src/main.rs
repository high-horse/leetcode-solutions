fn main() {
    println!("{:?}", maximum_odd_bunary_number("100110010"));
}

    pub fn maximum_odd_binary_number(s: String) -> String {
        let sth_str = s;
        let len_s = sth_str.len();
        let mut zeros = Vec::new();
        let mut ones = Vec::new();

        for char in sth_str.chars() {
            if char == '1' {
                ones.push('1');
            } else {
                zeros.push('0');
            }
        }

        match ones.len() {
            0 => "0".repeat(len_s),
            1 => "0".repeat(len_s - 1) + "1",
            _ => {
                ones.pop();
                ones.iter().collect::<String>() + &zeros.iter().collect::<String>() + "1"
            }
        }
    }
