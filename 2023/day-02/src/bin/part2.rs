#[derive(Debug)]
struct Game {
    game_num: u64,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u64,
    green: u64,
    blue: u64,
}

fn main() {
    // Game 1: 8 green, 4 red, 4 blue; 1 green, 6 red, 4 blue; 7 red, 4 green, 1 blue; 2 blue, 8 red, 8 green
    let input = include_str!("../../part1-input.txt");
    // TODO extract to `parse_input.rs`
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let colon = line.find(":").unwrap();
            let game_num: u64 = (line[5..colon]).parse().unwrap();
            let sets = &line[(colon + 2)..];
            let sets: Vec<Set> = sets
                .split("; ")
                .map(|set| {
                    let set: Set = set.split(", ").fold(
                        Set {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |acc, cubes| {
                            let mut iter = cubes.split(" ");
                            let num: u64 = iter.next().unwrap().trim().parse().unwrap();
                            let rgb: &str = iter.next().unwrap();
                            return match rgb {
                                "red" => Set { red: num, ..acc },
                                "green" => Set { green: num, ..acc },
                                "blue" => Set { blue: num, ..acc },
                                _ => acc,
                            };
                        },
                    );
                    set
                })
                .collect();
            Game { game_num, sets }
        })
        .collect();

    let answer: u64 = games
        .iter()
        .map(|Game { sets, .. }| {
            let min_cubes: Set = sets.iter().fold(
                Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, set| {
                    let red = if set.red > acc.red { set.red } else { acc.red };
                    let green = if set.green > acc.green {
                        set.green
                    } else {
                        acc.green
                    };
                    let blue = if set.blue > acc.blue {
                        set.blue
                    } else {
                        acc.blue
                    };
                    Set { red, green, blue }
                },
            );
            min_cubes.red * min_cubes.green * min_cubes.blue
        })
        .sum();

    println!("answer {}", answer);
}
