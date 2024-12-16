use std::fs::File;
use std::io::Read;

fn main() {
    let mut file_str = String::new();
    File::open("./src/input")
        .unwrap()
        .read_to_string(&mut file_str)
        .unwrap();

    let mut counter = 0;

    for line in file_str.lines() {
        let numbers: Vec<i32> = line.split_whitespace().map(|v| v.parse().unwrap()).collect();

        let mut diff = numbers[1] - numbers[0];
        let mut death = false;
        'inner: for i in 2..numbers.len() {
            let current_diff = numbers[i] - numbers[i-1];


            if (current_diff - diff).abs() > 1 || current_diff - diff == 0 {
                death = true;
                break 'inner
            } else {
                diff = current_diff
            }
        }

        if !death {
            counter += 1
        }
    }

    println!("{}", counter)
}
