
#[allow(unused)]
fn main () {
    let n = 8_i32;
    // let res = is_power_of_two(n);
    let res = n & (n-1);
    println!("is it =>{}", res);
    let n = 0_i32;
    let res = is_power_of_two(n);
    println!("is it =>{}", res);
    let n = 3_i32;
    let res = is_power_of_two(n);
    println!("is it =>{}", res);
    let n = 2097151_i32;
    let res = is_power_of_two(n);
    println!("is it =>{}", res);
}
 
 
#[allow(unused)]
pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n & (n-1) == 0 
}



 