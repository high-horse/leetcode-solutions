
#[allow(unused)]
fn main () {
    let (left , right) = ( 5_i32, 7_i32);
    let result = range_bitwise_and(left, right);
    println!("{:?}", result);
    println!("");
    println!("");
    let (left , right) = ( 0_i32, 0_i32);
    let result = range_bitwise_and(left, right);
    println!("{:?}", result);
    println!("");println!("");
    
    let (left , right) = ( 1_i32, 2147483647_i32);
    let result = range_bitwise_and(left, right);
    println!("{:?}", result);
}
 
 
#[allow(unused)]
pub fn range_bitwise_and_(left: i32, right: i32) -> i32 {
    if left == right {
        return left;
    }
    let mut res:i32 = left;
    for i in (left..=right) {
        res = res & i;
    }

    res
}

#[allow(unused)]
pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {

    while left < right {
        println!("{}", right);
        right &= right-1;
        
    }
    // for i in (left..=right) {
    //     res = res & i;
    // }

    right
}


 