use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|i| i.to_string())
        .map(|i| i.chars().map(|c| u8::from_str(&c.to_string()).unwrap() != 0).collect())
        .collect()
}

fn bit_counts(nums: &Vec<Vec<bool>>) -> Vec<u64> {
    nums.iter()
        .fold(
            vec![0u64; nums[0].len()],
            |acc, item| acc.iter().zip(item.iter()).map(|(a, b)| a+(*b as u64)).collect()
        )
}

fn gamma(nums: &Vec<Vec<bool>>) -> Vec<bool> {
    bit_counts(&nums)
        .iter()
        .map(|n| n > &u64::try_from(nums.len()/2).unwrap())
        .collect()
}

fn epsilon(nums: &Vec<Vec<bool>>) -> Vec<bool> {
    bit_counts(&nums)
        .iter()
        .map(|n| n < &u64::try_from(nums.len()/2).unwrap())
        .collect()
}

fn part_1(nums: &Vec<Vec<bool>>) -> u64 {
    to_u64(gamma(nums).as_slice()) * to_u64(epsilon(nums).as_slice())
}

fn filter_candidates(nums: &Vec<Vec<bool>>, position: usize, use_most: bool) -> Vec<bool> {
    if nums.len() == 1 {
        return nums[0].clone();
    }

    let ones = nums
        .iter()
        .map(|n| n[position])
        .filter(|n| *n)
        .count();
    let zeroes = nums.len() - ones;

    let most = ones >= zeroes;

    let remaining_nums = nums.iter()
        .filter(|n| n[position] == (most == use_most))
        .cloned()
        .collect();

    return filter_candidates(&remaining_nums, position + 1, use_most);
}

fn ox_rating(nums: &Vec<Vec<bool>>) -> Vec<bool> {
    filter_candidates(&nums, 0, true)
}

fn co_rating(nums: &Vec<Vec<bool>>) -> Vec<bool> {
    filter_candidates(&nums, 0, false)
}

fn part_2(nums: &Vec<Vec<bool>>) -> u64 {
    to_u64(ox_rating(&nums).as_slice()) * to_u64(co_rating(&nums).as_slice())
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let nums: Vec<Vec<bool>> = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

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

fn to_u64(slice: &[bool]) -> u64 {
    slice.iter().fold(0, |acc, &b| acc*2 + (b as u64) as u64)
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::to_u64;
    use super::gamma;
    use super::epsilon;
    use super::part_1;
    use super::ox_rating;
    use super::co_rating;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let nums = parse_input(input);
        assert_eq!(to_u64(gamma(&nums).as_slice()), 22);
        assert_eq!(to_u64(epsilon(&nums).as_slice()), 9);
        assert_eq!(part_1(&nums), 198);
        assert_eq!(to_u64(ox_rating(&nums).as_slice()), 23);
        assert_eq!(to_u64(co_rating(&nums).as_slice()), 10);
        assert_eq!(part_2(&nums), 230);
    }
}
