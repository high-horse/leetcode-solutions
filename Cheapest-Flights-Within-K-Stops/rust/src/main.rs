use std::f32;

#[allow(unused)]
fn main() {
    let n: i32 = 4;
    let flights: Vec<Vec<i32>> = vec![vec![0,1,100],vec![1,2,100],vec![2,0,100],vec![1,3,600],vec![2,3,200]];
    let src: i32 = 0;
    let dst: i32 = 3;
    let k: i32 = 1;
    let answer = find_cheapest_price(n, flights, src, dst, k);
}

#[allow(unused)]
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n_usize = n as usize;
    let src_usize = src as usize;
    let dst_usize = dst as usize;
    let mut prices = vec![f32::INFINITY; n_usize];
    prices[src_usize] = 0.0;

    for _ in 0..=k {
        let mut temp_prices = prices.clone();

        for flight in &flights {
            let s = flight[0] as usize; // source
            let d = flight[1] as usize; // destination
            let p = flight[2] as f32; // price

            if prices[s] == f32::INFINITY {
                continue;
            }

            if prices[s] + p < temp_prices[d] {
                temp_prices[d] = prices[s] + p;
            }
        }

        prices = temp_prices;
    }

    if prices[dst_usize] == f32::INFINITY {
    return -1_i32;
    } 
    return prices[dst_usize] as i32;

}
