use std::fs;
use std::env;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let formulae: HashMap<String, Formula> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|l| Formula::from_string(l))
        .map(|l| (l.element.clone(), l))
        .collect();

    // Part 1
    {
        let requirements: Vec<(String, i64)> = vec![("FUEL".to_string(), 1)];
        let required_ore = ore_for_requirements(requirements.clone(), &formulae);

        println!("part 1: {:?}", required_ore);
    }

    // Part 2
    {
        let target = 1_000_000_000_000;
        let mut min = 1;
        let mut max = 1_000_000_000_000;

        while min <= max {
            let guess = (min + max)/2;
            let requirements: Vec<(String, i64)> = vec![("FUEL".to_string(), guess)];
            let required_ore = ore_for_requirements(requirements.clone(), &formulae);

            match target.cmp(&required_ore) {
                std::cmp::Ordering::Less => max = guess - 1,
                std::cmp::Ordering::Greater => min = guess + 1,
                std::cmp::Ordering::Equal => {max = guess; min = guess;},
            }
        };

        println!("part 2: {:?}", max);
    }
}

fn ore_for_requirements(
    mut requirements: Vec<(String, i64)>,
    formulae: &HashMap<String, Formula>,
) -> i64 {
    let mut leftovers: HashMap<String, i64> = HashMap::new();
    let mut required_ore = 0;

    while requirements.len() != 0 {
        let (k, mut v) = requirements.pop().unwrap();

        let leftover_element = leftovers.entry(k.clone()).or_insert(0);
        if leftover_element >= &mut v {
            *leftover_element -= v;
            continue;
        } else {
            v -= *leftover_element;
            *leftover_element = 0;
        }

        let formula = formulae.get(&k).unwrap();
        let required_inputs: i64 = (v + formula.quantity - 1) / formula.quantity;
        *leftovers.entry(k).or_insert(0) += required_inputs * formula.quantity - v;

        for c in formula.components.iter() {
            let required_components = c.quantity * required_inputs;

            if c.element == "ORE" {
                required_ore += required_components;
            } else {
                requirements.push((c.element.clone(), required_components));
            }
        }
    }
    required_ore
}

#[derive(Debug)]
struct Component {
    element: String,
    quantity: i64,
}

#[derive(Debug)]
struct Formula {
    components: Vec<Component>,
    element: String,
    quantity: i64,
}

impl Formula {
    fn from_string(input: &str) -> Formula {
        let re = Regex::new(r".*?(\d+) ([A-Z]+).*?").unwrap();

        let parts: Vec<&str> = input.split("=>").collect();
        let components = parts[0].split(",")
            .map(|c| {
                let caps = re.captures(c).unwrap();
                Component{
                    element: caps[2].trim().to_string(),
                    quantity: caps[1].trim().parse().unwrap(),
                }
            })
            .collect();

        let caps = re.captures(parts[1]).unwrap();
        let element = caps[2].trim().to_string();
        let quantity = caps[1].trim().parse().unwrap();

        Formula{
            components,
            element,
            quantity,
        }
    }
}
