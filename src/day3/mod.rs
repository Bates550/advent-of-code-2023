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
        let width = layout.find('\n').unwrap();
        // println!("width: {:?}", width);
        // println!("layout: {:?}", layout);

        let single_line_layout: String = layout.split('\n').collect();
        // println!("single_line_layout: {:?}", single_line_layout);

        let height = single_line_layout.len() / width;
        // println!("height: {:?}", height);

        let s = Schematic {
            layout: single_line_layout,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        };
        // println!("schematic: {:?}", s);

        s
    }

    fn at(self: &Self, x: u16, y: u16) -> char {
        // Origin is top left. +x is right. +y is down.
        // o-> +x
        // |
        // v +y
        // 1-based coords. So (1, 1) is first char.

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

        let n = ((y - 1) * self.width) + (x - 1);
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
                    println!("width: {:?}", self.width);
                    println!("len: {:?}", partial_part_number.len());
                    println!("num: {:?}", partial_part_number);
                    let x = i % usize::from(self.width + 1) - partial_part_number.len() + 2;
                    let y = i / usize::from(self.width) + 1;
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
            // println!("partial_part_number: {:?}", partial_part_number);
            // println!("potential_part_numbers: {:?}", potential_part_numbers);
        }

        /***
         * Determine which of the potential part numbers are actual part numbers
         */

        let mut part_numbers: Vec<u16> = Vec::new();
        for potential_part_number in potential_part_numbers {
            let start_x = potential_part_number.x;
            let start_y = potential_part_number.y;
            let end_x = potential_part_number.x + potential_part_number.len - 1;
            let end_y = potential_part_number.y;
            let very_left = start_x == 1;
            let very_right = end_x == self.width;
            let very_top = start_y == 1;
            let very_bottom = end_y == self.height;
            println!(
                "num: {:?}, start: ({:?}, {:?}), end: ({:?}, {:?})",
                potential_part_number.num, start_x, start_y, end_x, end_y
            );

            let mut found = false;

            // If we're not on the very top row, check the top
            if !very_top && !found {
                println!("checking top");
                for x in start_x..end_x + 1 {
                    let y = potential_part_number.y - 1;
                    let char = self.at(x, y);
                    println!("char {:?} at ({:?}, {:?})", char, x, y);
                    if char != '.' {
                        part_numbers.push(potential_part_number.num);
                        found = true;
                        break;
                    }
                }
            }

            // If we're not on the very bottom row, check the bottom
            if !very_bottom && !found {
                println!("checking bottom");
                for x in start_x..end_x + 1 {
                    let y = potential_part_number.y + 1;
                    let char = self.at(x, y);
                    println!("char {:?} at ({:?}, {:?})", char, x, y);
                    if char != '.' {
                        part_numbers.push(potential_part_number.num);
                        found = true;
                        break;
                    }
                }
            }

            // If we're not on the very right column, check the right
            if !very_right && !found {
                println!("checking right");
                let x = end_x + 1;
                let y = potential_part_number.y;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left column, check the left
            if !very_left && !found {
                println!("checking left");
                let x = start_x - 1;
                let y = potential_part_number.y;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left and not on the very top, check the
            // top left corner
            if !very_left && !very_top && !found {
                println!("checking top left");
                let x = start_x - 1;
                let y = start_y - 1;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very right and not on the very top, check the
            // top right corner
            if !very_right && !very_top && !found {
                println!("checking top right");
                let x = end_x + 1;
                let y = end_y - 1;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left and not on the very bottom, check the
            // bottom left corner
            if !very_left && !very_bottom && !found {
                println!("checking bottom left");
                let x = start_x - 1;
                let y = start_y + 1;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very right and not on the very bottom, check the
            // bottom right corner
            if !very_right && !very_bottom && !found {
                println!("checking bottom right");
                let x = end_x + 1;
                let y = start_y + 1;
                let char = self.at(x, y);
                println!("char {:?} at ({:?}, {:?})", char, x, y);
                if char != '.' {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            println!("part_numbers: {:?}", part_numbers);
        }

        part_numbers
    }
}

fn problem_1(input: &str) -> u32 {
    /***
     * Parse schematic
     */

    let schematic = Schematic::new(&input);

    // println!("{:?}", schematic.at(3, 1));

    let part_numbers = schematic.part_numbers();

    let sum: u16 = part_numbers.iter().sum();

    u32::from(sum)
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

        input
    }

    #[test]
    fn schematic_at() {
        let input = "\
123
456
789";

        let schematic = Schematic::new(input);

        // 0-based
        // assert_eq!('1', schematic.at(0, 0));
        // assert_eq!('2', schematic.at(1, 0));
        // assert_eq!('3', schematic.at(2, 0));
        // assert_eq!('4', schematic.at(0, 1));
        // assert_eq!('5', schematic.at(1, 1));
        // assert_eq!('6', schematic.at(2, 1));
        // assert_eq!('7', schematic.at(0, 2));
        // assert_eq!('8', schematic.at(1, 2));
        // assert_eq!('9', schematic.at(2, 2));

        // 1-based
        assert_eq!('1', schematic.at(1, 1));
        assert_eq!('2', schematic.at(2, 1));
        assert_eq!('3', schematic.at(3, 1));
        assert_eq!('4', schematic.at(1, 2));
        assert_eq!('5', schematic.at(2, 2));
        assert_eq!('6', schematic.at(3, 2));
        assert_eq!('7', schematic.at(1, 3));
        assert_eq!('8', schematic.at(2, 3));
        assert_eq!('9', schematic.at(3, 3));
    }

    #[test]
    fn day3_part1() {
        assert_eq!(502, problem_1(&small_input()));
        assert_eq!(4361, problem_1(&input()));
    }

    // #[test]
    // fn day3_part2() {
    //     assert_eq!(2286, problem_2(input()));
    // }
}
