use std::fs;
use std::env;
use std::str::FromStr;
use itertools::Itertools;
use std::time::SystemTime;
use std::convert::TryInto;

fn part_1(nums: &Vec<u64>) -> u64 {
    nums
        .iter()
        .tuple_windows::<(&u64, &u64)>()
        .filter(|x| x.0 < x.1)
        .count()
        .try_into()
        .unwrap()
}

fn part_2(nums: &Vec<u64>) -> u64 {
    nums
        .iter()
        .tuple_windows::<(&u64, &u64, &u64, &u64)>()
        .filter(|x| (x.0 + x.1 + x.2) < (x.1 + x.2 + x.3))
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let nums: Vec<u64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|i| i.to_string())
        .map(|i| u64::from_str(&i).unwrap())
        .collect();

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&nums);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&nums);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let nums: Vec<u64> = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        assert_eq!(part_1(&nums), 7);
        assert_eq!(part_2(&nums), 5);
    }
}
