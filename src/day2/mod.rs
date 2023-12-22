use std::fs;

#[allow(dead_code)]
pub fn run() {
    let input =
        fs::read_to_string("src/day2/input.txt").expect("Should have been able to read the file");

    let result = possible_games(&input);
    println!("possible_games: {}", result);

    let result = power_of_games(&input);
    println!("power_of_games: {}", result);
}

#[derive(Debug)]
struct Game {
    id: u16,
    r: Vec<u16>,
    g: Vec<u16>,
    b: Vec<u16>,
}

#[derive(Debug)]
struct FewestPossibleGameConfig {
    r: u16,
    g: u16,
    b: u16,
}

fn parse_games(input: &str) -> Vec<Game> {
    /***
     * Parse games
     */

    // Example line: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let mut games: Vec<Game> = Vec::new();

    // Each game is on its own line in the input. Here we parse each game into a
    // Game struct
    for line in input.lines() {
        // The game id will between "Game " and a ":"
        let id_end = line.find(":").unwrap();
        let id = &line["Game ".len()..id_end];
        // println!("id: {}", id);

        // Each game will have multiple sets of cube pulls. We'll keep track of
        // each pull of each color. Note that we're not keeping track of each
        // color in relation to when the other colors were pulled, just that
        // there was a cube pull in which a given color was pulled.
        let mut r: Vec<u16> = Vec::new();
        let mut g: Vec<u16> = Vec::new();
        let mut b: Vec<u16> = Vec::new();
        let cube_pulls = &line[id_end + 2..];
        // println!("cube_pulls: {}", cube_pulls);
        for mut cube_pull in cube_pulls.split(';') {
            cube_pull = cube_pull.trim();
            // println!("cube_pull: {cube_pull}");

            for mut color_pull in cube_pull.split(',') {
                color_pull = color_pull.trim();
                // println!("color_pull: {color_pull}");

                if color_pull.contains("red") {
                    let color = "red";
                    let end = color_pull.find(color).unwrap();
                    let num_color = &color_pull[..end - 1];
                    // println!("num of {}: {}", color, num_color);
                    r.push(num_color.parse::<u16>().unwrap());
                } else if color_pull.contains("blue") {
                    let color = "blue";
                    let end = color_pull.find(color).unwrap();
                    let num_color = &color_pull[..end - 1];
                    // println!("num of {}: {}", color, num_color);
                    b.push(num_color.parse::<u16>().unwrap());
                } else if color_pull.contains("green") {
                    let color = "green";
                    let end = color_pull.find(color).unwrap();
                    let num_color = &color_pull[..end - 1];
                    // println!("num of {}: {}", color, num_color);
                    g.push(num_color.parse::<u16>().unwrap());
                } else {
                    panic!("Could not find a color in color_pull: {color_pull}");
                }
            }
        }

        games.push(Game {
            id: id.parse::<u16>().unwrap(),
            r: r,
            g: g,
            b: b,
        });
        // println!("{:?}", games);
    }

    games
}

fn possible_games(input: &str) -> u16 {
    let games = parse_games(input);

    /***
     * Determine ids of "possible" games
     */

    // "Possible" games are games in which no color pull was greater than the
    // following: 12 red cubes, 13 green cubes, and 14 blue cubes.
    const MAX_R: u16 = 12;
    const MAX_G: u16 = 13;
    const MAX_B: u16 = 14;
    let mut possible_game_ids: Vec<u16> = Vec::new();

    for game in games {
        // Assume game is possible until proven otherwise.
        let mut possible = true;
        for c in game.r {
            if c > MAX_R {
                possible = false;
                break;
            }
        }
        for c in game.g {
            if c > MAX_G {
                possible = false;
                break;
            }
        }
        for c in game.b {
            if c > MAX_B {
                possible = false;
                break;
            }
        }

        if possible {
            possible_game_ids.push(game.id);
        }
    }

    /***
     * Sum the ids of possible games
     */

    let sum: u16 = possible_game_ids.iter().sum();

    sum
}

fn power_of_games(input: &str) -> u32 {
    let games = parse_games(input);

    /***
     * Determine fewest number of cubes of each color to make a game possible
     */

    let mut game_configs: Vec<FewestPossibleGameConfig> = Vec::new();
    for game in games {
        let mut max_r: u16 = 0;
        let mut max_g: u16 = 0;
        let mut max_b: u16 = 0;

        for c in game.r {
            if c > max_r {
                max_r = c;
            }
        }
        for c in game.g {
            if c > max_g {
                max_g = c;
            }
        }
        for c in game.b {
            if c > max_b {
                max_b = c;
            }
        }

        game_configs.push(FewestPossibleGameConfig {
            r: max_r,
            g: max_g,
            b: max_b,
        })
    }
    // println!("{:?}", game_configs);

    /***
     * Determine "power" of each fewest possible game config
     */

    // "Power" is defined as the number of r, g, and b multiplied together.

    let mut powers: Vec<u32> = Vec::new();
    for game_config in game_configs {
        let power = game_config.r * game_config.g * game_config.b;
        powers.push(power.into());
    }

    /***
     * Sum powers
     */

    let sum: u32 = powers.iter().sum();

    sum
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

        //         let input = "\
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(8, possible_games(input));
    }

    #[test]
    fn day2_part2() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        //         let input = "\
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(2286, power_of_games(input));
    }
}
