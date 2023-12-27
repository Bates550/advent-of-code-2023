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

fn is_symbol(c: char) -> bool {
    let mut is_number = false;
    if let Some(_) = c.to_digit(10) {
        is_number = true;
    }

    if c != '.' && !is_number {
        return true;
    }

    false
}

fn x_y_from(i: i16, width: i16, num_width: i16) -> (u16, u16) {
    let x = ((i % width) - (num_width - 1)) + 1;
    let y = (i / width) + 1;

    (x.try_into().unwrap(), y.try_into().unwrap())
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

        // println!("\n---\nChecking for potential part numbers\n---\n");
        let mut potential_part_numbers: Vec<PotentialPartNumber> = Vec::new();
        let mut partial_part_number = String::new();

        for (i, char) in self.layout.chars().enumerate() {
            // println!(
            //     "\n--- char: {:?}, i: {:?}, width: {:?}, ppn: {:?}, len: {:?} ---\n",
            //     char,
            //     i,
            //     self.width,
            //     partial_part_number,
            //     partial_part_number.len()
            // );

            // println!("i: {:?}", i);
            // println!("width: {:?}", self.width);
            // println!("len: {:?}", partial_part_number.len());
            // println!("num: {:?}", partial_part_number);

            let (x, y) = x_y_from(
                i16::try_from(i).unwrap() - 1,
                self.width.try_into().unwrap(),
                partial_part_number.len().try_into().unwrap(),
            );
            let current_x = (u16::try_from(i).unwrap() % self.width) + 1;
            let end_of_row = current_x == self.width;
            // println!("current_x: {:?}", current_x);
            if end_of_row {
                // println!("end of row: ({:?}, {:?})", x, y);
            }

            if let Some(_) = char.to_digit(10) {
                // If we have a digit, add it to partial_part_number
                // println!(
                //     "pushing {:?} to partial_part_number: {:?}",
                //     char, partial_part_number
                // );
                partial_part_number.push(char);
            } else {
                // Otherwise no digit
                if partial_part_number.len() > 0 {
                    // If we were tracking a digit, it's now done and we can add
                    // it to potential_part_numbers.

                    // println!(
                    //     "tracking {:?} as potential part number at (x, y): ({:?}, {:?})",
                    //     partial_part_number, x, y
                    // );
                    potential_part_numbers.push(PotentialPartNumber {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        len: partial_part_number.len().try_into().unwrap(),
                        num: partial_part_number.parse::<u16>().unwrap(),
                    });

                    // Clear out partial_part_number to track the next one
                    partial_part_number.clear();
                } else {
                    // println!("noop");
                }
            }

            // If we're at the end of the row and we're tracking a number
            if end_of_row && partial_part_number.len() > 0 {
                // Digit is complete, add it to potential_part_numbers
                // println!(
                //     "tracking {:?} as potential part number at (x, y): ({:?}, {:?})",
                //     partial_part_number, x, y
                // );
                potential_part_numbers.push(PotentialPartNumber {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    len: partial_part_number.len().try_into().unwrap(),
                    num: partial_part_number.parse::<u16>().unwrap(),
                });

                // Clear out partial_part_number to track the next one
                partial_part_number.clear();
            }

            // println!("partial_part_number: {:?}", partial_part_number);
            // println!("potential_part_numbers: {:?}", potential_part_numbers);
        }
        // let ppns: Vec<u16> = potential_part_numbers.iter().map(|ppn| ppn.num).collect();
        // println!("potential_part_numbers: {:?}", ppns);

        /***
         * Determine which of the potential part numbers are actual part numbers
         */

        // println!("\n---\nDeterming actual parts from potentials\n---\n");
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
            // println!(
            //     "num: {:?}, start: ({:?}, {:?}), end: ({:?}, {:?})",
            //     potential_part_number.num, start_x, start_y, end_x, end_y
            // );

            let mut found = false;

            // If we're not on the very top row, check the top
            if !very_top && !found {
                // println!("checking top");
                for x in start_x..end_x + 1 {
                    let y = potential_part_number.y - 1;
                    let char = self.at(x, y);
                    // println!("char {:?} at ({:?}, {:?})", char, x, y);
                    if is_symbol(char) {
                        part_numbers.push(potential_part_number.num);
                        found = true;
                        break;
                    }
                }
            }

            // If we're not on the very bottom row, check the bottom
            if !very_bottom && !found {
                // println!("checking bottom");
                for x in start_x..end_x + 1 {
                    let y = potential_part_number.y + 1;
                    let char = self.at(x, y);
                    // println!("char {:?} at ({:?}, {:?})", char, x, y);
                    if is_symbol(char) {
                        part_numbers.push(potential_part_number.num);
                        found = true;
                        break;
                    }
                }
            }

            // If we're not on the very right column, check the right
            if !very_right && !found {
                // println!("checking right");
                let x = end_x + 1;
                let y = potential_part_number.y;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left column, check the left
            if !very_left && !found {
                // println!("checking left");
                let x = start_x - 1;
                let y = potential_part_number.y;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left and not on the very top, check the
            // top left corner
            if !very_left && !very_top && !found {
                // println!("checking top left");
                let x = start_x - 1;
                let y = start_y - 1;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very right and not on the very top, check the
            // top right corner
            if !very_right && !very_top && !found {
                // println!("checking top right");
                let x = end_x + 1;
                let y = end_y - 1;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very left and not on the very bottom, check the
            // bottom left corner
            if !very_left && !very_bottom && !found {
                // println!("checking bottom left");
                let x = start_x - 1;
                let y = start_y + 1;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }

            // If we're not on the very right and not on the very bottom, check the
            // bottom right corner
            if !very_right && !very_bottom && !found {
                // println!("checking bottom right");
                let x = end_x + 1;
                let y = start_y + 1;
                let char = self.at(x, y);
                // println!("char {:?} at ({:?}, {:?})", char, x, y);
                if is_symbol(char) {
                    part_numbers.push(potential_part_number.num);
                    found = true;
                }
            }
        }
        // println!("part_numbers: {:?}", part_numbers);

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

    let sum: u32 = part_numbers.iter().map(|&num| u32::from(num)).sum();

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
467..114.*
*..*....89
1.35..633.";

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
    fn coords_for_x_y_from() {
        // number starting on left edge:
        // "123......."
        assert_eq!((1, 1), x_y_from(2, 10, 3));

        // number starting one away from left edge:
        // ".234......"
        assert_eq!((2, 1), x_y_from(3, 10, 3));

        // number ending one away from right edge:
        // "......789."
        assert_eq!((7, 1), x_y_from(8, 10, 3));

        // number ending on right edge:
        // ".......890"
        assert_eq!((8, 1), x_y_from(9, 10, 3));

        // number ending on right edge followed by a number starting on left
        // edge:
        // ".......890123......."
        //         \n ^
        assert_eq!((8, 1), x_y_from(9, 10, 3));

        // number starting on left edge preceded by a number ending on left
        // edge:
        // ".......890123......."
        //         \n ^
        assert_eq!((1, 2), x_y_from(12, 10, 3));
    }

    #[test]
    fn day3_part1() {
        assert_eq!(592, problem_1(&small_input()));
        assert_eq!(4361, problem_1(&input()));
    }

    // #[test]
    // fn day3_part2() {
    //     assert_eq!(2286, problem_2(input()));
    // }
}
