use day_04::part2::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file).unwrap();
    println!("{}", result);
}