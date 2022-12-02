use std::fs;

fn main() {
    let content = fs::read_to_string("./src/01/input.txt").expect("Not able to read input");
    let group_max : i32 = content.split("\n\n").map(|group| {
        let foods = group.split("\n");
        foods.fold(0, |accum, item| {
            let item_int : i32 = match item.parse() {
                Ok(i) => i,
                _ => 0
            };
            accum + item_int
        })
    }).max().unwrap();
    println!("{group_max}");
}
