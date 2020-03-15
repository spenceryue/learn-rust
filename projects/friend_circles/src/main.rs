// use std::io;
mod solution;
use solution::find_circle_num;

fn main() {
    let input = "[[1,0,0,1],[0,1,1,0],[0,1,1,1],[1,0,1,1]]";
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Couldn't read input!");

    let parsed: Vec<Vec<i32>> = serde_json::from_str(&input).expect("Couldn't parse to JSON.");

    println!("Input received: {:?}", parsed);

    println!("Solution: {}", find_circle_num(parsed));
}
