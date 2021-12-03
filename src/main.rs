use std::fs;
// mod day1;
// use day1::{part1, part2};
fn main() {
    let contents = read();
    // part1::finale(contents);
}

fn read() -> Vec<u16> {
    let file = fs::read_to_string("input.txt").ok().unwrap();
    let contents: Vec<u16> = file.lines().map(|x| x.parse().unwrap()).collect();
    contents
}