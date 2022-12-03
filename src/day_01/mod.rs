use std::fs;

fn elven_cals() -> Vec<i32> {
    let content = fs::read_to_string("./input.txt").expect("Not able to read input");

    let mut groups =
        content
            .split("\n\n")
            .map(|group| {
                let foods = group.split("\n");
                foods.fold(0, |accum, item| {
                    let item_int : i32 = match item.parse() {
                        Ok(i) => i,
                        _ => 0
                    };
                    accum + item_int
                })
            })
            .collect::<Vec<i32>>();

    groups.sort();

    groups
}

#[allow(dead_code)]
pub fn solution1() -> i32 {
    let groups = elven_cals();
    groups[groups.len() - 1]
}

#[allow(dead_code)]
pub fn solution2() -> i32 {
    let groups = elven_cals();
    let len = groups.len();
    let mut i = len - 1;
    let mut sum = 0;
    while i > len - 4 {
        sum += groups[i];
        i = i - 1;
    }

    sum
}
