use {{crate_name}}::part1::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file).unwrap();
    println!("{}", result);
}