#[allow(unused)]
pub fn first_palindrome(words: Vec<String>) -> String {
	for word in words.iter() {
	    let vec_word: Vec<char> = word.chars().collect();
	    
	    let mut first_index = 0;
	    let mut last_index = word.len()-1;
	    
	    while first_index < last_index {
	        if vec_word[first_index] != vec_word[last_index] {
	            break;
	        }
	        first_index += 1;
	        last_index -= 1;
	    }
	    if first_index >= last_index {
	        return word.to_string();
	    }
	    
	}
	"".to_string()
}

#[allow(unused)]
fn main() {
	let words: Vec<String> =  vec!["abc".to_string(),"car".to_string(),"ada".to_string(),"racecar".to_string(),"cool".to_string()];
	let words: Vec<String> = vec!["xngla".to_string(),"e".to_string(),"itrn".to_string(),"y".to_string(),"s".to_string(),"pfp".to_string(),"rfd".to_string()];
	let res: String = first_palindrome(words);
	println!("ans => {res}");
}