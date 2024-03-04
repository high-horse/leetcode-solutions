#[allow(unused)]
fn main() {
    println!("Hello, world!");
    // let tokens: Vec<i32> = vec![100];
    // let power: i32 = 50;
    
    // let tokens: Vec<i32> = vec![200,100];
    // let power: i32 = 150;
    
    let tokens: Vec<i32> = vec![100,200,300,400];
    let power: i32 = 200;
    
    println!("score => {:?}", bag_of_tokens_score(tokens, power));
}

use std::cmp;

#[allow(unused)]
pub fn bag_of_tokens_score( tokens: Vec<i32>,  power: i32) -> i32 {
    let mut tokens: Vec<i32> = tokens;
    let mut power = power;
    
    println!("{:?}", tokens);
    tokens.sort();
    println!("{:?}", tokens);
    let length = tokens.len();
    let mut left = 0;
    let mut right = length-1;
    
    let mut score: i32 = 0;
    let mut max_score: i32 = 0;
    
    while (left <= right) {
    println!();
    println!("left => {left}, right => {right}, length => {length}");
        println!("score => {score}, max_score => {max_score}");
        if tokens[left] <= power {
            println!("if");
            power -= tokens[left];
            left += 1;
            score += 1;
            max_score = cmp::max(max_score, score);
        }
        else if  score > 0 {
            println!("else if");
            power += tokens[right];
            right -= 1;
            score -= 1;
        }
        else {
            break
        }
        
    }
    max_score
}
