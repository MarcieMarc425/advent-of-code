use day_02::part1::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file).unwrap();
    println!("{}", "day 02 - part 1");
}
