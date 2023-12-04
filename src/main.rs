use std::fs;

fn main() {
    let input =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let result = calibration_sum(&input);

    println!("{}", result);
}

fn calibration_sum(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = Vec::new();
        for char in line.chars() {
            let digit = char.to_digit(10);
            if let Some(digit) = digit {
                digits.push(digit);
            }
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        println!("digits: {:?}", digits);
        println!("first: {:?}, last: {:?}", first_digit, last_digit);

        let mut line_concatenation = String::new();
        line_concatenation.push_str(&first_digit.to_string());
        line_concatenation.push_str(&last_digit.to_string());
        println!("line_concatenation: {line_concatenation}");
        sum += line_concatenation.parse::<u32>().unwrap();
    }
    println!("sum: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, calibration_sum(input))
    }
}
