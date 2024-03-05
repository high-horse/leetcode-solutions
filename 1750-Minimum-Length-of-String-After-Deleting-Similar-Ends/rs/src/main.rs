#[allow(unused)]
fn main() {
    let s = String::from("ca");
    println!("{}", minimum_length(s));
    let s = String::from("cabaabac");
    println!("{}", minimum_length(s));
    let s = String::from("aabccabba");
    println!("answer => {}", minimum_length(s));
}


#[allow(unused)]
pub fn minimum_length(s: String) -> i32 {

    let mut left  = 0;
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut right  = s_vec.len() - 1 ;
    
    while left < right {
        let same_ch = s_vec[left];
        if s_vec[left] == s_vec[right] {
            let same_ch = s_vec[left];
            while left<=right && s_vec[left] == same_ch {
                left += 1;
            }
            while left<=right && s_vec[right] == same_ch {
                right -= 1;
            }
            
        } else {
            break;
        }
    }
    
    (right-left +1) as i32
}
