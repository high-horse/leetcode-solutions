use std::collections::HashMap;


fn main() {
    let s = "III".to_string();
    let res = roman_to_int(s);
    println!("{}", res);

    let s = "LVIII".to_string();
    let res = roman_to_int(s);
    println!("{}", res);

    let s = "MCMXCIV".to_string();
    let res = roman_to_int(s);
    println!("{}", res);

}

pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let static_vals: HashMap<String, i32> = [
        ("I".to_string(), 1_i32),
        ("V".to_string(), 5_i32),
        ("X".to_string(), 10_i32),
        ("L".to_string(), 50_i32),
        ("C".to_string(), 100_i32),
        ("D".to_string(), 500_i32),
        ("M".to_string(), 1000_i32),
    ]
    .iter()
    .cloned()
    .collect();

    // println!("{:?}", static_vals);

    let mut last_char: String = "A".to_string();
    for (_i, ch) in s.chars().rev().enumerate() {
        let character: String = ch.into();

        let current_value: i32;
        if let Some(&value) = static_vals.get(&character) {
            current_value = value.clone();
        } else {
            current_value = 0_i32;
        }

        let previous_val: i32;

        if let Some(&value) = static_vals.get(&last_char) {
            previous_val = value.clone();
        } else {
            previous_val = 0_i32;
        }

        if previous_val > current_value {
            result += -current_value
        } else {
            result += current_value
        }

        // println!("current {character}, previous {last_char}");

        last_char = ch.into();
    }

    result
}
