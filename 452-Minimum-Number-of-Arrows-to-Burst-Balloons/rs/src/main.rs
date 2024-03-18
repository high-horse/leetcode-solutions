fn main() {
    println!("Hello, world!");
	let points: Vec<Vec<i32>> = vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]];
    let points: Vec<Vec<i32>> = vec![vec![9,12],vec![1,10],vec![4,11],vec![8,12],vec![3,9],vec![6,9],vec![6,7]];
    
    println!("{}", find_min_arrow_shots(points));

}
pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {

	points.sort_unstable_by_key(|v| v[1]);

	let min = std::i32::MIN;
	let mut end = min;

	let mut arrows = 0_i32 ;

	for i in points.iter() {

	    if i[0] == min {
	        if i[1] > end {
	            arrows += 1;
	            end = i[1];
	        }
	    }

	    else if i[0] > end {
	        arrows += 1;
	        end = i[1];
	    }
	}

	arrows
}
