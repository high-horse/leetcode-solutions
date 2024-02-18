#[allow(unused)]
use std::collections::BinaryHeap;
use std::cmp::Reverse;


#[allow(unused)]
fn main() {
    let n: i32 = 2; 
    let meetings: Vec<Vec<i32>> = vec![[0,10].to_vec(),[1,5].to_vec(),[2,7].to_vec(),[3,4].to_vec()];
    let answer = most_booked(n, meetings);
    println!("Answer => {answer}");
    
    let n = 3_i32;
    let meetings: Vec<Vec<i32>> = vec![vec![1,20],vec![2,10],vec![3,5],vec![4,9],vec![6,8]];
    let answer = most_booked(n, meetings);
    println!("Answer => {answer}");
}

#[allow(unused)]
pub fn most_booked(n: i32,  meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings;
    meetings.sort_unstable();

    let mut freq = vec![0; n as usize];

    let mut available: BinaryHeap<Reverse<usize>> = (0..n as usize).map(Reverse).collect();
    let mut booked: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

    for arr in meetings {
        let (start, end) = (arr[0], arr[1]);
        while !booked.is_empty()
            && booked.peek().map(|&Reverse((s, _))| s).unwrap() <= start as i64
        {
            let Reverse((_, room)) = booked.pop().unwrap();
            available.push(Reverse(room));
        }
        if let Some(Reverse(room)) = available.pop() {
            booked.push(Reverse((end as i64, room)));
            freq[room] += 1;
        } else if let Some(Reverse((free, room))) = booked.pop() {
            booked.push(Reverse((free + (end - start) as i64, room)));
            freq[room] += 1;
        }
    }

    freq.into_iter()
        .zip(0..)
        .max_by_key(|&(count, room)| (count, -room))
        .map(|(_, room)| room)
        .unwrap()
}

