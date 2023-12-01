use std::io::Error;

pub fn process(input: &str) -> Result<u32, Error> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let num = get_num_in_string(line).unwrap();
        sum += num;
    }
    Ok(sum)
}

pub fn get_num_in_string(str: &str) -> Result<u32, Error> {
    // find first occurence of number
    let mut first_half_num_char: Option<char> = None;
    'outer: for (_, c) in str.chars().enumerate() {
        if c.is_numeric() {
            first_half_num_char = Some(c);
            break 'outer;
        }
    }

    // find last occurence of number
    let mut second_half_num_char: Option<char> = None;
    'outer: for (_, c) in str.chars().rev().enumerate() {
        if c.is_numeric() {
            second_half_num_char = Some(c);
            break 'outer;
        }
    }

    // if both are None, return Error
    if first_half_num_char.is_none() && second_half_num_char.is_none() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No number found in string",
        ));
    }

    // if first_half_num_char is None or second_half_num_char is None, set to the other
    if first_half_num_char.is_none() {
        first_half_num_char = second_half_num_char;
    }
    if second_half_num_char.is_none() {
        second_half_num_char = first_half_num_char;
    }

    // merge the two chars into a string
    let mut num_string = String::new();
    num_string.push(first_half_num_char.unwrap());
    num_string.push(second_half_num_char.unwrap());

    // convert string to number
    let num: u32 = num_string.parse().unwrap();
    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let sum = process(input).unwrap();
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_get_num_in_string() {
        let input1 = "1abc2";
        let result1 = get_num_in_string(input1).unwrap();
        assert_eq!(result1, 12);

        let input2 = "pqr3stu8vwx";
        let result2 = get_num_in_string(input2).unwrap();
        assert_eq!(result2, 38);

        let input3 = "a1b2c3d4e5f";
        let result3 = get_num_in_string(input3).unwrap();
        assert_eq!(result3, 15);

        let input4 = "treb7uchet";
        let result4 = get_num_in_string(input4).unwrap();
        assert_eq!(result4, 77);
    }
}
