use std::fs;
use std::env;
use std::str::FromStr;
use itertools::Itertools;
use std::time::SystemTime;

fn find_sums(nums: &Vec<u64>, target: u64, size: usize) -> Vec<Vec<u64>> {
    nums.iter()
        .cloned()
        .combinations(size)
        .filter(|i| i.iter().cloned().sum::<u64>() == target)
        .collect_vec()
}

fn print_results(nums: &Vec<Vec<u64>>) {
    nums.iter()
        .for_each(|r| {
            println!("Numbers: {:?}", r);
            println!("Product: {:?}", r.iter().cloned().fold1(|x,y| x*y).unwrap());
        });
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

    let target = 2020;
    let setup_time = SystemTime::now();
    let part_1_ans = find_sums(&nums, target, 2);
    let part_1_time = SystemTime::now();
    let part_2_ans = find_sums(&nums, target, 3);
    let part_2_time = SystemTime::now();

    println!("Part 1:");
    print_results(&part_1_ans);
    println!("Part 2:");
    print_results(&part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::find_sums;

    #[test]
    fn example1() {
        let nums: Vec<u64> = vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        assert_eq!(find_sums(&nums, 2020, 2), vec![[1721, 299]]);
    }
}
