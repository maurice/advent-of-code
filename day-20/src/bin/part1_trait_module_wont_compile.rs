use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Broadcaster<'a> {
    destinations: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct FlipFlop<'a> {
    on: bool,
    destinations: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
    memory: HashMap<&'a str, Pulse>, // memory slots for each source
    destinations: Vec<&'a str>,
}

trait Module<'a>
where
    Self: Sized,
{
    fn has_destination(&self, destination: &str) -> bool;
    fn receive_pulse<S>(&mut self, source: &str, send_pulse: S)
    where
        S: FnMut(Pulse, &'a str /* destination */) -> ();
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn has_destination(&self, destination: &str) -> bool {
        self.destinations.contains(&destination)
    }

    fn receive_pulse<S>(&mut self, source: &str, send_pulse: S)
    where
        Self: Sized,
        S: FnMut(Pulse, &'a str /* destination */) -> (),
    {
        // todo
    }
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn has_destination(&self, destination: &str) -> bool {
        self.destinations.contains(&destination)
    }

    fn receive_pulse<S>(&mut self, source: &str, send_pulse: S)
    where
        Self: Sized,
        S: FnMut(Pulse, &'a str /* destination */) -> (),
    {
        // todo
    }
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn has_destination(&self, destination: &str) -> bool {
        self.destinations.contains(&destination)
    }

    fn receive_pulse<S>(&mut self, source: &str, send_pulse: S)
    where
        Self: Sized,
        S: FnMut(Pulse, &'a str /* destination */) -> (),
    {
        // todo
    }
}

fn parse_modules<'a>(input: &'a str) -> HashMap<&'a str, dyn Module<'a>> {
    let mut modules = HashMap::new();
    let mut conjunction_modules = vec![];

    // gobble the module definitions
    for line in input.trim().lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(",").collect();
        if name == "broadcaster" {
            modules.insert(name, Broadcaster { destinations });
        } else if &name[0..1] == "&" {
            conjunction_modules.push(&name[1..]);
            modules.insert(
                &name[1..].to_string(),
                Conjunction {
                    destinations,
                    memory: HashMap::new(),
                },
            );
        } else if &name[0..1] == "%" {
            modules.insert(
                &name[1..].to_string(),
                FlipFlop {
                    on: false,
                    destinations,
                },
            );
        }
    }

    // setup memory for conjunctions
    for name in conjunction_modules {}

    modules
}

fn get_answer(input: &str) -> u64 {
    // let modules = parse_modules(input);

    let mut modules = HashMap::new();
    let mut conjunction_modules = vec![];

    // gobble the module definitions
    for line in input.trim().lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(",").collect();
        if name == "broadcaster" {
            modules.insert(name, Broadcaster { destinations });
        } else if &name[0..1] == "&" {
            conjunction_modules.push(&name[1..]);
            modules.insert(
                &name[1..].to_string(),
                Conjunction {
                    destinations,
                    memory: HashMap::new(),
                },
            );
        } else if &name[0..1] == "%" {
            modules.insert(
                &name[1..].to_string(),
                FlipFlop {
                    on: false,
                    destinations,
                },
            );
        }
    }

    // setup memory for conjunctions
    for name in conjunction_modules {}

    1
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
