#[allow(unused)]
fn main() {
    // let heights: Vec<i32> = vec![4,12,2,7,3,18,20,3,19];
    // let bricks: i32 = 10;
    // let ladders: i32 = 2;
    // let answer: i32 = furthest_building_1(heights, bricks, ladders);
    // println!("case 1 => {}", answer);
     
    // // heights = [14,3,19,3], bricks = 17, ladders = 0
    // let heights: Vec<i32> = vec![14,3,19,3];
    // let bricks: i32 = 17;
    // let ladders: i32 = 0;
    // let answer: i32 = furthest_building(heights, bricks, ladders);

    // println!("case 2 => {}", answer);
    
    let heights: Vec<i32> = vec![1,5,1,2,3,4,10000];
    let bricks: i32 = 4;
    let ladders: i32 = 1;
    // let answer: i32 = furthest_building(heights.clone(), bricks.clone(), ladders.clone());
    // println!("case 3 => {}", answer);
    let answer: i32 = furthest_building_2(heights.clone(), bricks.clone(), ladders.clone());

    println!("case 3 => {}", answer);
    
    
}

#[allow(unused)]
pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let mut ladders = ladders;
    let mut bricks = bricks;
    
    let mut index: i32 = 0 ;
    let mut jump: i32=0;
    let total_buildings = heights.len() as i32;
    while index < total_buildings-1{
        let mut current_building = heights[index as usize];
        let mut next_building = heights[(index + 1) as usize];
        println!("current {}, next {}",current_building, next_building);
        if current_building >= next_building {
            println!("Parkour !!!");
            jump += 1;
        } else {
            let mut diff = next_building - current_building;
            
            if bricks >= diff {
                println!("BRICKS !!!");
                bricks -= diff;
                jump += 1;
            } else if (diff > bricks) && (ladders > 0){
                println!("Ladders !!!");
                ladders -= 1;
                jump += 1;
            } else {
                break;
            }
        }
        index += 1;
    }
    // println!("The jump is {}", jump);
    jump
}

#[allow(unused)]
pub fn furthest_building_1(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    let mut jumps = vec![];
    let total_buildings = heights.len() as i32;
    let mut index: i32 = 0 ;
    while index < total_buildings-1{
        let current_building = heights[index as usize];
        let next_building = heights[(index + 1) as usize];
        println!("current {}, next {}",current_building, next_building);
        if current_building < next_building {
            let diff = next_building - current_building;
            println!("diff => {:?}", diff);
            println!("Jumps initial => {:?}", jumps);
            jumps.push(diff);
            println!("Jumps added => {:?}", jumps);
            jumps.sort();
            println!("Jumps sorted => {:?}", jumps);
            println!("ladders initial => {}", ladders);
            if jumps.len() > ladders as usize {
                println!("Jumps => {:?}", jumps);
                println!("bricks => {:?}", bricks);
                bricks -= jumps[0];
                println!("bricks => {:?}", bricks);
                jumps.remove(0);
                println!("Jumps => {:?}", jumps);
            }
            println!("ladders final => {}", ladders);
            if bricks < 0 {
                return index;
            }
        }
        println!("PARKOUR");
        index += 1;
    }
    index
}


#[allow(unused)]
pub fn furthest_building_2(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    let mut jumps = vec![];
    let total_buildings = heights.len() as i32;
    let mut index: i32 = 0 ;
    while index < total_buildings-1{
        let current_building = heights[index as usize];
        let next_building = heights[(index + 1) as usize];
        if current_building < next_building {
            let diff = next_building - current_building;
            jumps.push(diff);
            jumps.sort();
            if ladders > 0 {
                ladders -= 1;
            } else if !jumps.is_empty() {
                bricks -= jumps[0];
                jumps.remove(0);
            }
            if bricks < 0 {
                return index;
            }
        }
        index += 1;
    }
    index
}


