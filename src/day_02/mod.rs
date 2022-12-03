use std::fs;
use std::path::Path;

// A X - Rock
// B Y - Paper
// C Z - Scissors
//
// X awards 1pt
// Y awards 2pt
// Z awards 3pt
//
// 0pt for losing
// 3pt for draw
// 6pt for winning

fn get_input() -> String {
    let input = Path::new("./day_02/input.txt");
    fs::read_to_string(input).unwrap()
}

fn points_for_scenario(raw_scenario: &str) -> u32 {
    match raw_scenario {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

#[allow(dead_code)]
pub fn solution1() -> u32 {
    get_input().split("\n").fold(0, |accum, item| {
        accum + points_for_scenario(item)
    })
}

#[allow(dead_code)]
pub fn solution2() {
    panic!("Not implemented");
}
