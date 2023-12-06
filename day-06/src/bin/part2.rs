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

fn parse_input(input: &str) -> Race {
    let mut lines = input.trim().lines();
    let time: u64 = lines.next().unwrap()[5..].replace(" ", "").parse().unwrap();
    let distance_record: u64 = lines.next().unwrap()[9..].replace(" ", "").parse().unwrap();
    Race {
        time,
        distance_record,
    }
}

fn get_answer(input: &str) -> u64 {
    let race = parse_input(input);
    println!("race {:?}", race);
    let mut num_winners = 0;
    // probably a more efficient way would be to search outwards (in both directions)
    // from the middle of the range and stop searching once we are no longer finding winners.
    for hold_time in 1..race.time {
        let time_remaining = race.time - hold_time;
        let distance = time_remaining * hold_time;
        // println!(
        //     "hold_time {}, time_remaining {}, distance {}",
        //     hold_time, time_remaining, distance
        // );
        if distance > race.distance_record {
            num_winners += 1;
        }
    }
    num_winners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let answer = get_answer(input);
        assert_eq!(answer, 71503);
    }
}
