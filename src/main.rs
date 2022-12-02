use std::fs;

fn main() {
    let content = fs::read_to_string("./src/01/input.txt").expect("Not able to read input");
    let mut groups : Vec<i32> =
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

    let len = groups.len();
    let mut i = len - 1;
    let mut sum = 0;
    while i > len - 4 {
        sum += groups[i];
        i = i - 1;
    }

    println!("{sum}");
}
