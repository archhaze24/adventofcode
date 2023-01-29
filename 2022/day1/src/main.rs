use std::fs;
use std::io::Read;

fn main() {
    let mut file = fs::File::open("src/input.txt").unwrap();
    let mut input = String::new();
    let mut calories: Vec<i32> = vec![];

    file.read_to_string(&mut input).unwrap();

    let mut temp_calories: i32 = 0;

    for line in input.lines() {
        let line = line.trim();

        if !line.is_empty() {
            let line: i32 = line.parse().unwrap();
            temp_calories += line
        } else {
            calories.push(temp_calories);
            temp_calories = 0;
        }
    }

    calories.sort();

    let mut top_three_calories: i32 = 0;

    for elf in calories[calories.len() - 3..calories.len()].iter() {
        top_three_calories += elf;
    }

    println!("{top_three_calories}");
}
