use fancy_regex::Regex;
use std::io::Error;

use crate::part1::get_num_in_string;

pub fn process(input: &str) -> Result<u32, Error> {
    let mut sum: u32 = 0;
    let parsed_text = replace_spelled_out_nums(input);
    for line in parsed_text.lines() {
        let num = get_num_in_string(line).unwrap();
        sum += num;
    }
    Ok(sum)
}

pub fn convert_spelled_out_num_to_num(str: &str) -> Result<u32, Error> {
    let num: u32 = match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid spelled out number",
            ))
        }
    };
    Ok(num)
}

pub fn replace_spelled_out_nums(str: &str) -> String {
    let regex = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let mut new_str = String::new();
    let mut last_match: usize = 0;
    for cap in regex.captures_iter(str) {
        let m = cap.unwrap().get(1).unwrap();
        let matched_text = str[m.start()..m.end()].to_string();
        if last_match < m.start() {
            new_str.push_str(&str[last_match..m.start()]);
        }
        let num = convert_spelled_out_num_to_num(&matched_text).unwrap();
        new_str.push_str(&num.to_string());
        last_match = m.end();
    }
    new_str.push_str(&str[last_match..]);

    new_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let sum = process(input).unwrap();
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_convert_spelled_out_num_to_num() {
        let num = convert_spelled_out_num_to_num("one").unwrap();
        assert_eq!(num, 1);
        let num = convert_spelled_out_num_to_num("two").unwrap();
        assert_eq!(num, 2);
        let num = convert_spelled_out_num_to_num("three").unwrap();
        assert_eq!(num, 3);
        let num = convert_spelled_out_num_to_num("four").unwrap();
        assert_eq!(num, 4);
        let num = convert_spelled_out_num_to_num("five").unwrap();
        assert_eq!(num, 5);
        let num = convert_spelled_out_num_to_num("six").unwrap();
        assert_eq!(num, 6);
        let num = convert_spelled_out_num_to_num("seven").unwrap();
        assert_eq!(num, 7);
        let num = convert_spelled_out_num_to_num("eight").unwrap();
        assert_eq!(num, 8);
        let num = convert_spelled_out_num_to_num("nine").unwrap();
        assert_eq!(num, 9);
    }

    #[test]
    fn test_replace_spelled_out_nums() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nasjdklf\n567\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\nabc\n123";
        let output = replace_spelled_out_nums(input);
        assert_eq!(
            output,
            "219\n823\nabc123xyz\nasjdklf\n567\nx2134\n49872\nz18234\n7pqrst6teen\nabc\n123"
        );
    }
}
