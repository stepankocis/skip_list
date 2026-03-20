use rand::prelude::*;
fn main() {
    let original = vec![1,2,5,6,7,12,13];

    let levels = construct(original);
    let number_being_found = 7;
    let where_it_is = finding(number_being_found, &levels);
    println!("{} je na indexu {}", number_being_found, where_it_is);

}

fn construct(original: Vec<i32>) -> Vec<Vec<(i32, i32)>> {
    let mut levels: Vec<Vec<(i32, i32)>> = vec![vec![];8];
    let mut rng = rand::rng();

    for i in 0..original.len() {
        levels[0].push((original[i] as i32, i as i32));
        let kolik_pater = rng.random_range(0..8);
        for j in 1..kolik_pater {
            let prev_i = levels[j-1].len() - 1;
            levels[j].push((original[i] as i32, prev_i as i32));
        }
    }
    return levels;
}

fn finding(number_being_found: i32, levels: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut current_level = levels.len() - 1;
    while current_level > 0 && levels[current_level].is_empty() {
        current_level -= 1;
    }
    if levels[current_level].is_empty() {
        return -1;
    }
    let mut current_index = 0;
    loop {
        let value = levels[current_level][current_index].0;
        let next_level_index = levels[current_level][current_index].1;
        if value == number_being_found {
            let mut temp_level = current_level;
            let mut temp_index = current_index;
            while temp_level > 0 {
                temp_index = levels[temp_level][temp_index].1 as usize;
                temp_level -= 1;
            }
            return levels[0][temp_index].1;
        }

        if value < number_being_found {
            if current_index + 1 < levels[current_level].len() && levels[current_level][current_index + 1].0 <= number_being_found {
                current_index += 1;
            } else if current_level > 0 {
                current_index = next_level_index as usize;
                current_level -= 1;
            } else {
                return -1;
            } 
        } else if current_level > 0 {
            if current_index == 0 {
                current_index = 0;
            } else {
                current_index = levels[current_level][current_index - 1].1 as usize;
            }
            current_level -= 1;
        } else {
            return -1;
        }
    }
}
