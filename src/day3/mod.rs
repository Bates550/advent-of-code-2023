use std::fs;

#[allow(dead_code)]
pub fn run() {
    let input =
        fs::read_to_string("src/day3/input.txt").expect("Should have been able to read the file");

    let result = problem_1(&input);
    println!("problem 1: {}", result);

    // let result = problem_2(&input);
    // println!("problem_2: {}", result);
}

#[derive(Debug)]
struct PotentialPartNumber {
    x: u16,
    y: u16,
    len: u16,
    num: u16,
}

#[derive(Debug)]
struct Schematic {
    layout: String,
    width: u16,
    height: u16,
}

impl Schematic {
    pub fn new(input: &str) -> Schematic {
        let layout = String::from(input);
        let width = layout.find('\n').unwrap() - 1;
        // println!("width: {:?}", width);
        // println!("layout: {:?}", layout);

        let single_line_layout: String = layout.split('\n').collect();
        // println!("single_line_layout: {:?}", single_line_layout);

        let height = single_line_layout.len() / width - 1;
        // println!("height: {:?}", height);

        let s = Schematic {
            layout: single_line_layout,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        };
        println!("schematic: {:?}", s);

        s
    }

    fn at(self: &Self, x: u16, y: u16) -> char {
        // Origin is top left. +x is right. +y is down.
        // o-> +x
        // |
        // v +y
        // 0-based coords. So (0, 0) is first char.

        if x > self.width {
            panic!(
                "Attempted to access ({:?}, {:?}), but x: {:?} is greather than width: {:?}",
                x, y, x, self.width
            );
        }
        if y > self.height {
            panic!(
                "Attempted to access ({:?}, {:?}), but y: {:?} is greather than height: {:?}",
                x, y, y, self.height
            );
        }

        let n = (y * self.width) + x + 1;
        let result = self.layout.chars().nth(n.try_into().unwrap()).unwrap();

        result
    }

    pub fn part_numbers(self: &Self) -> Vec<u16> {
        /***
         * Parse layout for all potential part numbers
         */

        // In other words, parse for all numbers. We'll determine in a later
        // step, which ones are actually part numbers based on their adjacency
        // to symbols.

        let mut potential_part_numbers: Vec<PotentialPartNumber> = Vec::new();
        let mut partial_part_number = String::new();
        for (i, char) in self.layout.chars().enumerate() {
            println!("char: {:?}", char);

            if let Some(_) = char.to_digit(10) {
                // If we have a digit, add it to partial_part_number
                partial_part_number.push(char);
            } else {
                // Otherwise no digit
                if partial_part_number.len() > 0 {
                    // If we were tracking a digit, it's now done and we can add
                    // it to potential_part_numbers.

                    println!("i: {:?}", i);
                    let x = i % usize::from(self.width + 1) - partial_part_number.len();
                    let y = i / usize::from(self.width + 1);
                    potential_part_numbers.push(PotentialPartNumber {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        len: partial_part_number.len().try_into().unwrap(),
                        num: partial_part_number.parse::<u16>().unwrap(),
                    });

                    // Clear out partial_part_number to track the next one
                    partial_part_number.clear();
                }
            }
            println!("partial_part_number: {:?}", partial_part_number);
            println!("potential_part_numbers: {:?}", potential_part_numbers);
        }

        /***
         * Determine which of the potential part numbers are actual part numbers
         */

        Vec::new()
    }
}

fn problem_1(input: &str) -> u32 {
    /***
     * Parse schematic
     */

    let schematic = Schematic::new(&input);

    // println!("{:?}", schematic.at(3, 1));

    let part_numbers = schematic.part_numbers();
    // println!("part_numbers: {:?}", part_numbers);

    // for line in input.lines() {}
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        input
    }

    fn small_input() -> &'static str {
        let input = "\
467..114..
...*......
..35..633.";
        //         let input = "\
        // 0123456789
        // 2468024680";

        input
    }

    #[test]
    fn day3_part1() {
        assert_eq!(8, problem_1(&small_input()));
    }

    // #[test]
    // fn day3_part2() {
    //     assert_eq!(2286, problem_2(input()));
    // }
}
