fn main() {
    println!("Hello, world!");

    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let longest = longest_common_prefix(strs);
    println!("{}", longest);

    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let longest = longest_common_prefix(strs);
    println!("{}", longest);

}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut common = strs[0].clone();

    for s in &strs[1..] {
        while !s.starts_with(&common) {
            common.pop();
            if common.is_empty() {
                return common;
            }
        }
    }

    common
}
