use std::fs;

#[allow(dead_code)]
pub fn run() {
    let input =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");
    let result = calibration_sum_1(&input);

    println!("calibration_sum_1: {}", result);

    let result = calibration_sum_2(&input);
    println!("calibration_sum_2: {}", result);
}

fn calibration_sum_1(input: &str) -> u32 {
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

fn calibration_sum_2(input: &str) -> u32 {
    // Example: "eightwothree"
    let spelled_digits: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    let mut possible_spelled_digits: Vec<&str> = Vec::new();
    let mut possible_spelled_digits_offsets: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut digits = Vec::new();
        for (char_i, char) in line.chars().enumerate() {
            println!("\n--- iteration {} for '{}' ---\n", char_i, char);
            // Update our possible spelled digit offsets
            for offset in possible_spelled_digits_offsets.iter_mut() {
                *offset += 1;
            }
            // Another way to do this, but gives a warning about unused result
            // of collect that I don't currently understand.
            // possible_spelled_digits_offsets
            //     .iter_mut()
            //     .map(|offset| *offset += 1)
            //     .collect::<Vec<_>>();

            // If the char is a digit then add it to digits
            let digit = char.to_digit(10);
            if let Some(digit) = digit {
                digits.push(digit);
            }

            // Otherwise it's an alpha char.

            /***
             * DIGIT ADD
             */

            // Add to possible_spelled_digits the spelled digits whose first
            // char matches the current char
            for &spelled_digit in spelled_digits.iter() {
                if spelled_digit.chars().nth(0) == Some(char) {
                    possible_spelled_digits.push(spelled_digit);
                    possible_spelled_digits_offsets.push(0)
                }
            }

            /***
             * DIGIT FILTER
             */

            // Filter out possible spelled digits whose char at
            // possible_spelled_digit_offset does not match current char

            // for (i, &possible_spelled_digit) in possible_spelled_digits.iter().enumerate() {
            //     let offset = *(possible_spelled_digits_offsets.iter().nth(i).unwrap());
            //     if possible_spelled_digit.chars().nth(offset) == Some(char) {
            //         // Keep it
            //     }
            // }

            // println!("--- BEFORE DIGIT FILTER ---");
            // println!("possible_spelled_digits: {:?}", possible_spelled_digits);
            // println!(
            //     "possible_spelled_digits_offsets: {:?}",
            //     possible_spelled_digits_offsets
            // );
            let mut removed_offset_indices: Vec<usize> = Vec::new();
            possible_spelled_digits = possible_spelled_digits
                .iter()
                .enumerate()
                .filter(|(i, &possible_spelled_digit)| {
                    // let offset = *(possible_spelled_digits_offsets.iter().nth(*i).unwrap());
                    let offset = possible_spelled_digits_offsets[*i];
                    let result = possible_spelled_digit.chars().nth(offset) == Some(char);
                    if !result {
                        removed_offset_indices.push(*i);
                    }
                    result
                })
                .map(|(_, &possible_spelled_digit)| possible_spelled_digit)
                .collect();
            // println!("--- AFTER DIGIT FILTER ---");
            println!("possible_spelled_digits: {:?}", possible_spelled_digits);
            // println!("removed_offset_indices: {:?}", removed_offset_indices);
            possible_spelled_digits_offsets = possible_spelled_digits_offsets
                .iter()
                .enumerate()
                .filter(|(i, _)| !removed_offset_indices.contains(i))
                .map(|(_, offset)| *offset)
                .collect();
            println!(
                "possible_spelled_digits_offsets: {:?}",
                possible_spelled_digits_offsets
            );

            /***
             * DIGIT CHECK
             */

            // Check if any offets are equal to the length of their associated
            // strs and if they are, add them to digits as a number. Keep track
            // of which indices were removed so we can then filter those out.
            let mut removed_spelled_digits_indices: Vec<usize> = Vec::new();
            for (i, &offset) in possible_spelled_digits_offsets.iter().enumerate() {
                let possible_spelled_digit = possible_spelled_digits[i];
                if offset == possible_spelled_digit.len() - 1 {
                    // Find the actual number for the spelled digit
                    let digit = spelled_digits
                        .iter()
                        .position(|&spelled_digit| possible_spelled_digit == spelled_digit)
                        .unwrap()
                        + 1;

                    digits.push(digit.try_into().unwrap());
                    removed_spelled_digits_indices.push(i);
                }
            }
            println!(
                "removed_spelled_digits_indices: {:?}",
                removed_spelled_digits_indices
            );

            // Filter out removed indices from possible_spelled_digits and
            // possible_spelled_digits_offsets
            possible_spelled_digits = possible_spelled_digits
                .iter()
                .enumerate()
                // .filter(|(i, _)| !removed_spelled_digits_indices.contains(i))
                // .map(|(_, &spelled_digit)| spelled_digit)
                .filter_map(|(i, &spelled_digit)| {
                    // if removed_spelled_digits_indices.contains(&i) {
                    //     None
                    // } else {
                    //     Some(spelled_digit)
                    // }
                    (!removed_spelled_digits_indices.contains(&i)).then_some(spelled_digit)
                })
                .collect();

            possible_spelled_digits_offsets = possible_spelled_digits_offsets
                .iter()
                .enumerate()
                .filter_map(|(i, &spelled_digit)| {
                    (!removed_spelled_digits_indices.contains(&i)).then_some(spelled_digit)
                })
                .collect();

            println!("--- AFTER DIGIT CHECK ---");
            println!("possible_spelled_digits: {:?}", possible_spelled_digits);
            println!(
                "possible_spelled_digits_offsets: {:?}",
                possible_spelled_digits_offsets
            );
            println!("digits: {:?}", digits);
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        println!("digits: {:?}", digits);
        println!("first: {}, last: {}", first_digit, last_digit);

        let mut line_concatenation = String::new();
        line_concatenation.push_str(&first_digit.to_string());
        line_concatenation.push_str(&last_digit.to_string());
        println!("line_concatenation: {line_concatenation}");
        sum += line_concatenation.parse::<u32>().unwrap();
    }

    println!("sum: {sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, calibration_sum_1(input))
    }

    #[test]
    fn day1_part2() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        // let input = "threeightwo";

        assert_eq!(281, calibration_sum_2(input))
    }
}
