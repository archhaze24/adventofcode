// 1st column, enemy pick
// A - rock
// B - paper
// C - scissors

// 1st part
// 2nd column, my pick
// X - rock
// Y - paper
// Z - scissors

// 2nd part
// 2nd column, outcome
// X - lose
// Y - draw
// Z - win

// scores
// 1 - rock, 2 - paper, 3 - scissors
// 0 - lost, 3 - draw, 6 - won

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    let mut score: i32 = 0;
    let mut temp_score: i32 = 0;

    file.read_to_string(&mut input).unwrap();

    // 1st part
    //
    // for line in input.lines() {
    //     let mut line = line.chars();
    //     let enemy_pick = line.next().unwrap();
    //     let my_pick = line.last().unwrap();
    //
    //     match my_pick {
    //         'X' => temp_score += 1,
    //         'Y' => temp_score += 2,
    //         'Z' => temp_score += 3,
    //         _ => {}
    //     }
    //
    //     match (my_pick, enemy_pick) {
    //         // draw
    //         ('X', 'A') => temp_score += 3,
    //         ('Y', 'B') => temp_score += 3,
    //         ('Z', 'C') => temp_score += 3,
    //         // loss
    //         ('X', 'B') => temp_score += 0,
    //         ('Y', 'C') => temp_score += 0,
    //         ('Z', 'A') => temp_score += 0,
    //         // win
    //         ('X', 'C') => temp_score += 6,
    //         ('Y', 'A') => temp_score += 6,
    //         ('Z', 'B') => temp_score += 6,
    //         (_, _) => {}
    //     }
    //
    //     score += temp_score;
    //     temp_score = 0;
    // }

    // 2nd part
    //
    for line in input.lines() {
        let mut line = line.chars();
        let enemy_pick = line.next().unwrap();
        let outcome = line.last().unwrap();

        match outcome {
            'X' => temp_score += 0,
            'Y' => temp_score += 3,
            'Z' => temp_score += 6,
            _ => {}
        }

        match (outcome, enemy_pick) {
            // loss
            ('X', 'B') => temp_score += 1,
            ('X', 'C') => temp_score += 2,
            ('X', 'A') => temp_score += 3,
            // draw
            ('Y', 'A') => temp_score += 1,
            ('Y', 'B') => temp_score += 2,
            ('Y', 'C') => temp_score += 3,
            // win
            ('Z', 'C') => temp_score += 1,
            ('Z', 'A') => temp_score += 2,
            ('Z', 'B') => temp_score += 3,
            (_, _) => {}
        }

        score += temp_score;
        temp_score = 0;
    }

    println!("{score}");
}
