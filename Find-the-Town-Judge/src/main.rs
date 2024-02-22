use std::collections::HashMap;

#[allow(unused)]
fn main () {
    let n: i32 = 2;
    let trust: Vec<Vec<i32>> = vec![vec![1,2]];
    
    let n: i32 = 3;
    // let trust: Vec<Vec<i32>> = vec![vec![1,3],vec![2,3]];
    // let trust: Vec<Vec<i32>> = vec![vec![1,2],vec![2,3]];
    // let trust: Vec<Vec<i32>> = vec![vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![4,3]];
    let trust: Vec<Vec<i32>> = vec![vec![1,3],vec![2,3],vec![3,1]]; let n: i32 = 3;
    println!("{}", find_judge(n, trust));
}
 
 
#[allow(unused)]
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if trust.len() < 1 {
        if n == 1 {
            return 1_i32
        }
        return -1_i32
    }
    let mut trust_map: HashMap<i32,  Vec<i32>> = HashMap::new();
    let mut people: Vec<i32> = vec![];
    
    for item in trust {
        if ! people.iter().any(|x| *x==item[0]) {
            people.push(item[0]);
        }
        if ! people.iter().any(|x| *x==item[1]) {
            people.push(item[1]);
        }
        let key = item[1] ; 
        if let Some(values) = trust_map.get_mut(&key){
            values.push(item[0]);
        } else {
            trust_map.insert(item[1], vec![item[0]]);
        }
    }
    let to_be_trusted_by  = people.len()  -1;
    
    for (key, val) in trust_map.iter() {
        if val.len() == to_be_trusted_by {
            if trust_map.values().any(|v| v.contains(&key)) {
                continue;
            }
            return *key;
        }
    }
    -1_i32
}

 