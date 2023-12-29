fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {}", answer);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.trim().lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|token| token.parse().ok())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|token| token.parse().ok())
        .collect();
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance_record)| Race {
            time: *time,
            distance_record: *distance_record,
        })
        .collect()
}

fn get_answer(input: &str) -> u64 {
    let races = parse_input(input);
    println!("races {:?}", races);
    let mut winners: Vec<u64> = vec![];
    for race in races {
        let mut num_winners = 0;
        println!("race {:?}", race);
        // probably a more efficient way would be to search outwards (in both directions)
        // from the middle of the range and stop searching once we are no longer finding winners.
        for hold_time in 1..race.time {
            let time_remaining = race.time - hold_time;
            let distance = time_remaining * hold_time;
            println!(
                "hold_time {}, time_remaining {}, distance {}",
                hold_time, time_remaining, distance
            );
            if distance > race.distance_record {
                num_winners += 1;
            }
        }
        winners.push(num_winners);
    }
    println!("winners {:?}", winners);
    winners.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let answer = get_answer(input);
        assert_eq!(answer, 288);
    }
}
