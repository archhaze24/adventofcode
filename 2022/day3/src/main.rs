use std::fs;
use std::io::Read;

fn main() {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut file = fs::File::open("src/input.txt").unwrap();
    let mut input = String::new();
    let mut score = 0;

    file.read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let mut char_index: usize = 0;

        for char in first.chars() {
            if second.contains(char) {
                let index = chars.iter().position(|&x| x == char).unwrap();
                char_index = index + 1;
            }
        }

        score += char_index;
    }

    println!("{score}");
}
