use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster {
        destinations: Vec<&'a str>,
    },
    FlipFlop {
        on: bool,
        destinations: Vec<&'a str>,
    },
    Conjunction {
        memory: HashMap<&'a str, Pulse>, // memory slots for each source
        destinations: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    fn has_destination(&self, destination: &str) -> bool {
        match self {
            Module::Broadcaster { destinations } => destinations,
            Module::FlipFlop { destinations, .. } => destinations,
            Module::Conjunction { destinations, .. } => destinations,
        }
        .contains(&destination)
    }

    fn add_source(&mut self, source: &'a str) {
        if let Module::Conjunction { memory, .. } = self {
            memory.insert(source, Pulse::Low);
        };
    }

    fn receive_pulse<S>(&mut self, pulse: Pulse, source: &'a str, send_pulse: &mut S)
    where
        S: FnMut(Pulse, &'a str /* destination */) -> (),
    {
        match self {
            Module::Broadcaster { destinations } => {
                destinations
                    .iter()
                    .for_each(|d| send_pulse(pulse.clone(), d));
            }
            Module::FlipFlop { on, destinations } => {
                if pulse == Pulse::Low {
                    let p = if *on { Pulse::Low } else { Pulse::High };
                    destinations.iter().for_each(|d| send_pulse(p.clone(), d));
                    *on = !*on;
                }
            }
            Module::Conjunction {
                memory,
                destinations,
            } => {
                memory.insert(source, pulse);
                let p = if memory.values().all(|p| p == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                destinations.iter().for_each(|d| send_pulse(p.clone(), d));
            }
        }
    }
}

fn parse_modules<'a>(input: &'a str) -> HashMap<&'a str, Module<'a>> {
    let mut modules = HashMap::new();
    let mut conjunction_modules = vec![];

    // gobble the module definitions
    for line in input.trim().lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(",").map(|s| s.trim()).collect();
        if name == "broadcaster" {
            modules.insert(name, Module::Broadcaster { destinations });
        } else if &name[0..1] == "&" {
            conjunction_modules.push(&name[1..]);
            modules.insert(
                &name[1..],
                Module::Conjunction {
                    destinations,
                    memory: HashMap::new(),
                },
            );
        } else if &name[0..1] == "%" {
            modules.insert(
                &name[1..],
                Module::FlipFlop {
                    on: false,
                    destinations,
                },
            );
        }
    }

    // setup memory for conjunctions
    for name in conjunction_modules {
        let sources: Vec<&str> = modules
            .iter()
            .filter_map(|(source, module)| module.has_destination(&name).then_some(*source))
            .collect();
        // println!("{}'s sources are {:?}", name, sources);
        let conjunction = modules.get_mut(name).unwrap();
        for source in sources {
            conjunction.add_source(source);
        }
    }

    modules
}

fn euclidean_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        let min = a.min(b);
        let max = a.max(b);
        euclidean_gcd(min, max % min)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / euclidean_gcd(a, b)
}

struct Message<'a> {
    pulse: Pulse,
    source: &'a str,
    destination: &'a str,
}

fn get_answer(input: &str) -> u64 {
    let mut modules = parse_modules(input);
    // println!("got modules {:?}", modules);

    // need low pulse to rx...
    // there is one conjunction module that sends signal to rx, so that module would need a high pulse
    let rx_source = modules
        .keys()
        .find(|name| modules.get(*name).is_some_and(|m| m.has_destination("rx")))
        .expect("one rx source");

    // rx's source has another four conjunction sources
    // so each of these need to send a high
    // which means they each need a low input
    let target_modules = modules
        .keys()
        .filter(|name| {
            modules
                .get(*name)
                .is_some_and(|m| m.has_destination(&rx_source))
        })
        .map(|name| (*name))
        .collect::<Vec<_>>();
    println!(
        "rx_source {}, target modules {:?}",
        rx_source, target_modules
    );

    let mut high_indices = HashMap::new();
    let mut num_presses = 0;
    'outer: loop {
        num_presses += 1;
        // println!("num presses {}", num_presses);
        let mut queue = vec![Message {
            pulse: Pulse::Low,
            source: "button",
            destination: "broadcaster",
        }];
        while queue.len() > 0 {
            let Message {
                pulse,
                source,
                destination,
            } = queue.remove(0);
            // println!(
            //     "handling {} => {:?} => {} at {}",
            //     source, pulse, destination, num_presses
            // );
            if pulse == Pulse::Low && target_modules.contains(&&destination) {
                if !high_indices.contains_key(&destination) {
                    high_indices.insert(destination, num_presses);
                    if high_indices.len() == target_modules.len() {
                        break;
                    }
                }
            }
            if num_presses > 10000 {
                break 'outer;
            }
            match modules.get_mut(&destination) {
                Some(module) => module.receive_pulse(pulse, source, &mut |p, d| {
                    queue.push(Message {
                        pulse: p,
                        source: destination,
                        destination: d,
                    });
                }),
                None => { /* ignore */ }
            }
        }
    }

    println!("high indices {:?}", high_indices);

    high_indices.values().fold(
        high_indices
            .get(*target_modules.get(0).unwrap())
            .unwrap()
            .clone(),
        |acc, presses| lcm(acc, *presses),
    )

    // num_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(get_answer(input), 32000000);
    }

    #[test]
    fn example_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(get_answer(input), 11687500);
    }
}
