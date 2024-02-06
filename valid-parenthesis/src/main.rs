fn main() {
    let s = "()[]{}".to_string();
    let status = is_valid(s);
    println!("{}", status);
}

pub fn is_valid(s: String) -> bool {
    let mut temp_s: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            ')' => {
                if let Some('(') = temp_s.pop() {
                    continue;
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some('{') = temp_s.pop() {
                    continue;
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some('[') = temp_s.pop() {
                    continue;
                } else {
                    return false;
                }
            }
            _ => temp_s.push(ch),
        }
    }

    temp_s.is_empty()
}
