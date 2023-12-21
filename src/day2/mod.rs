use std::fs;

#[allow(dead_code)]
pub fn run() {
    let input =
        fs::read_to_string("src/day2/input.txt").expect("Should have been able to read the file");

    let result = possible_games_1(&input);
    println!("possible_games_1: {}", result);

    // let result = possible_games_2(&input);
    // println!("possible_games_2: {}", result);
}

fn possible_games_1(_input: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, possible_games_1(input));
    }
}
