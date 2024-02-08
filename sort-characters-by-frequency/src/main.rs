fn main() {
    println!("{:?}", frequency_sort("tree".to_string()));
    println!("{:?}", frequency_sort("cccaaa".to_string()));
    println!("{:?}", frequency_sort("Aabb".to_string()));
}


pub fn frequency_sort(s: String) -> String {
    let mut count: Vec<(char, usize)> = Vec::new();

    for ch in s.chars() {
        match count.iter_mut().find(|(c, _)| *c == ch ) {
            Some((_, val)) => *val += 1,
            None => count.push((ch, 1))
        }
    }
    // println!("{:?}", count);

    count.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
    // println!("{:?}",count);

    let mut res: String = String::from("");
    for (k, v) in count{
        // println!("{:?} {}", k, v);
        for times in 0..v {
            res.push(k.clone());
            // println!("{}",res);
        }
    }

    res
}