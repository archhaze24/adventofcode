use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    // part one
    let mut file = File::open("./src/input").unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for line in file_string.lines() {
        let split: Vec<&str> = line.trim().split("   ").collect();
        vec1.push(split[0].parse().unwrap());
        vec2.push(split[1].parse().unwrap());
    }

    vec1.sort();
    vec2.sort();
    let mut answer = 0;

    for i in 0..vec1.len() {
        answer += (vec1[i] - vec2[i]).abs()
    }

    println!("part one: {}", answer);

    // part two
    let mut map1: HashMap<i32, i32> = HashMap::new();
    for value in &vec1 {
        let existing_value = map1.get(value);

        match existing_value {
            Some(v) => map1.insert(*value, v+1),
            None => map1.insert(*value, 1)
        };
    }

    let mut map2: HashMap<i32, i32> = HashMap::new();
    for value in &vec2 {
        let existing_value = map2.get(value);

        match existing_value {
            Some(v) => map2.insert(*value, v+1),
            None => map2.insert(*value, 1)
        };
    }

    let mut answer = 0;

    for key in map1.keys() {
        let right_map_value: i32 = *map2.get(key).unwrap_or(&0);
        answer += *map1.get(key).unwrap() * right_map_value * key
    }

    println!("part two: {}", answer)
}
