use indicatif::ProgressIterator;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {}", answer);
}

// represents one entry in a list like
// seed-to-soil map:
// 50 98 2
#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl Mapping {
    fn map_source_to_dest(mappings: &Vec<Mapping>, source: usize) -> usize {
        let mapping = mappings.iter().find(|mapping| {
            mapping.source_start <= source && mapping.source_start + mapping.len > source
        });
        match mapping {
            None => source,
            Some(Mapping {
                dest_start,
                source_start,
                ..
            }) => {
                let offset = source - source_start;
                return dest_start + offset;
            }
        }
    }
}

#[derive(Debug)]
struct Mappings {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Mappings {
    fn get_location_for_seed(&self, seed: usize) -> usize {
        // not sure if there is some pipeline operator or way to use iter to apply all these transforms slightly less imperatively?
        let soil = Mapping::map_source_to_dest(&self.seed_to_soil, seed);
        let fertilizer = Mapping::map_source_to_dest(&self.soil_to_fertilizer, soil);
        let water = Mapping::map_source_to_dest(&self.fertilizer_to_water, fertilizer);
        let light = Mapping::map_source_to_dest(&self.water_to_light, water);
        let temperature = Mapping::map_source_to_dest(&self.light_to_temperature, light);
        let humidity = Mapping::map_source_to_dest(&self.temperature_to_humidity, temperature);
        let location = Mapping::map_source_to_dest(&self.humidity_to_location, humidity);
        // println!(
        //     "seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}",
        //     seed, soil, fertilizer, water, light, temperature, humidity, location
        // );
        location
    }
}

type Seeds = Vec<usize>;

fn parse_input(input: &str) -> (Seeds, Mappings) {
    let mut lines = input.trim().lines();
    let seeds: Seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut seed_to_soil: Vec<Mapping> = Vec::new();
    let mut soil_to_fertilizer: Vec<Mapping> = Vec::new();
    let mut fertilizer_to_water: Vec<Mapping> = Vec::new();
    let mut water_to_light: Vec<Mapping> = Vec::new();
    let mut light_to_temperature: Vec<Mapping> = Vec::new();
    let mut temperature_to_humidity: Vec<Mapping> = Vec::new();
    let mut humidity_to_location: Vec<Mapping> = Vec::new();

    // parse mappings
    let mut current_mappings = &mut seed_to_soil;
    for line in lines {
        if line == "" {
            continue;
        }

        if line.ends_with(":") {
            match line {
                "seed-to-soil map:" => current_mappings = &mut seed_to_soil,
                "soil-to-fertilizer map:" => current_mappings = &mut soil_to_fertilizer,
                "fertilizer-to-water map:" => current_mappings = &mut fertilizer_to_water,
                "water-to-light map:" => current_mappings = &mut water_to_light,
                "light-to-temperature map:" => current_mappings = &mut light_to_temperature,
                "temperature-to-humidity map:" => current_mappings = &mut temperature_to_humidity,
                "humidity-to-location map:" => current_mappings = &mut humidity_to_location,
                _ => {
                    panic!("Unhandled map line: {}", line);
                }
            }
            continue;
        }

        if line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            let numbers: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();
            let dest_start = numbers.get(0).expect("destination range start").to_owned();
            let source_start = numbers.get(1).expect("source range start").to_owned();
            let len = numbers.get(2).expect("length of range").to_owned();
            current_mappings.push(Mapping {
                dest_start,
                source_start,
                len,
            })
        }
    }

    (
        seeds,
        Mappings {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn get_answer(input: &str) -> usize {
    let (seeds, mappings) = parse_input(input);
    let seed_start = seeds.iter().step_by(2);
    let seed_len = seeds.iter().skip(1).step_by(2);
    let result = seed_start
        .zip(seed_len)
        .progress()
        .flat_map(|(start, len)| *start..(*start + *len))
        .map(|seed| mappings.get_location_for_seed(seed))
        .min()
        .expect("closest location");
    result
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(get_answer(input), 46);
    }
}
