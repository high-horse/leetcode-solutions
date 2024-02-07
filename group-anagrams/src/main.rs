use std::collections::HashMap;

fn main() {
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    
    println!("{:?}", group_anagrams(strs));
}


pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hmap: HashMap<String, Vec<String>> = HashMap::new();
    
    let original = strs.clone();
    
    let mut sorted: Vec<String> = Vec::new();
    
    for i in 0..original.len(){
        let mut temp: Vec<char> = original[i].clone().chars().collect();
        // println!("{:?}", temp);
        temp.sort_unstable();
        let temp: String = temp.into_iter().collect();
        // println!("{:?}", temp);
        sorted.push(temp);
    }
    
    // println!("{:?}", sorted);
    //  println!("MAP => {:?}", hmap);
    for (index, item) in sorted.iter().enumerate(){
        // println!("{:?}", item);
        
        if let Some(val) = hmap.get_mut(item) {
            val.push(original[index].clone());
        } else {
            // map.insert(item, vec![original[index].clone()]);
            hmap.insert(item.clone(), vec![original[index].clone()]);
        }
    }
    
    let output: Vec<Vec<String>> = hmap.into_iter().map(|(_, v)| v).collect();

    
    output
    
    // todo!()
}