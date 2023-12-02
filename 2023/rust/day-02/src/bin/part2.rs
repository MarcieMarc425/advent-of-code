use day_02::part2::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let reuslt = process(file).unwrap();
    println!("result: {}", reuslt);
}
