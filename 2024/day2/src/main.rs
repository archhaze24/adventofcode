use std::fs::File;
use std::io::Read;

fn main() {
    let mut file_str = String::new();
    File::open("./src/input")
        .unwrap()
        .read_to_string(&mut file_str)
        .unwrap();

    // part one
    let mut counter = 0;

    let reports: Vec<Vec<i32>> = file_str
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    for report in &reports {
        let max_diff = check_diff(report, None);

        if max_diff < 4 && max_diff > -4 && max_diff != 0 {
            counter += 1
        }
    }

    println!("part one: {}", counter);

    // part two

    let mut counter = 0;

    'outer: for report in &reports {
        let max_diff = check_diff(report, None);

        if max_diff < 4 && max_diff > -4 && max_diff != 0 {
            counter += 1
        } else {
            for skip in 0..report.len() {
                let max_diff = check_diff(report, Some(skip));
                if max_diff < 4 && max_diff > -4 && max_diff != 0 {
                    counter += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("part two: {}", counter);
}

fn check_diff(rep: &Vec<i32>, skip: Option<usize>) -> i32 {
    let mut clone = rep.clone();
    if let Some(i) = skip {
        clone.remove(i);
    }

    if !(clone.iter().is_sorted() || clone.iter().rev().is_sorted()) {
        return -5; // death
    }

    let mut max_diff = 0;

    for (index, value) in clone.iter().enumerate() {
        if index + 1 == clone.len() {
            break;
        }

        let diff = value - clone[index + 1];
        if diff == 0 {
            max_diff = 0;
            break;
        }
        max_diff = max_diff.max(diff.abs()) // since it can be decreasing
    }

    max_diff
}
