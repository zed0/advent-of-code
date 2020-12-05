use std::fs;
use std::env;
use std::time::SystemTime;

fn seat_to_id(seat: &str) -> u32 {
    let (row, col) = seat.split_at(7);
        8 * u32::from_str_radix(&row, 2).unwrap()
            + u32::from_str_radix(&col, 2).unwrap()
}

fn input_to_binary(input: &str) -> String {
    input.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1")
}

fn find_missing(seat_ids: &Vec<u32>) -> u32 {
    let before_min = seat_ids.iter().min().unwrap() - 1;
    let max = seat_ids.iter().max().unwrap();
    let sum: u32 = seat_ids.iter().sum();
    let expected_sum = (max * (max+1))/2 - ((before_min * (before_min+1))/2);
    return expected_sum - sum;
}


fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();


    let binary_input = input_to_binary(&input);
    let ids: Vec<u32> = binary_input.lines()
        .map(|line| seat_to_id(line))
        .collect();
    let part_1_ans = ids.iter().max().unwrap();
    let part_1_time = SystemTime::now();
    let part_2_ans = find_missing(&ids);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::seat_to_id;
    use super::input_to_binary;

    fn example1() -> String {
        String::from("FBFBBFFRLR")
    }

    #[test]
    fn example1a() {
        let seat = input_to_binary(&example1());
        assert_eq!(seat_to_id(&seat), 357);
    }

    fn example2() -> String {
        String::from(
"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
        )
    }

    #[test]
    fn example2a() {
        let binary_input = input_to_binary(&example2());
        let ids: Vec<u32> = binary_input.lines()
            .map(|line| seat_to_id(line))
            .collect();
        assert_eq!(ids.iter().max().unwrap(), &820);
    }
}
