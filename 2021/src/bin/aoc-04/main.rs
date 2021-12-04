use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    checked: bool,
    num: u64,
}

type Board = Vec<Vec<Cell>>;

fn parse_input(input: &str) -> (Vec<u64>, Vec<Board>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let sequence = parts[0]
        .split(",")
        .map(|i| u64::from_str(i).unwrap())
        .collect();

    let boards = parts[1..].iter()
        .map(|board_input|
             board_input
             .lines()
             .map(|line|
                  line
                  .split_whitespace()
                  .map(|i| Cell {checked: false, num: u64::from_str(i).unwrap()})
                  .collect()
             )
             .collect()
        )
        .collect();

    (sequence, boards)
}

fn board_complete(board: &Board) -> bool {
    let row_complete = board.iter()
        .any(|row| row.iter().all(|cell| cell.checked));
    let col_complete = (0..board[0].len())
        .any(|i| board.iter().all(|row| row[i].checked));
    row_complete || col_complete
}

fn board_score(board: &Board) -> u64 {
    board.iter()
        .map(|row| row.iter()
             .filter(|cell| !cell.checked)
             .map(|cell| cell.num)
             .sum::<u64>()
        )
        .sum()
}

fn next_board(board: &Board, num: u64) -> Board {
    let mut next = board.clone();
    for row in &mut next {
        for cell in row.iter_mut() {
            if cell.num == num {
                cell.checked = true;
            }
        }
    }
    return next;
}

fn part_1(sequence: &Vec<u64>, mut boards: Vec<Board>) -> u64 {
    for &num in sequence {
        boards = boards.iter().map(|board| next_board(board, num)).collect();
        let complete = boards.iter()
            .find(|board| board_complete(board));
        if complete.is_some() {
            return board_score(complete.unwrap()) * num;
        }
    }
    panic!("No bingo");
}

fn part_2(sequence: &Vec<u64>, mut boards: Vec<Board>) -> u64 {
    for &num in sequence {
        boards = boards.iter().map(|board| next_board(board, num)).collect();
        if boards.len() > 1 {
            boards = boards.iter().filter(|board| !board_complete(board)).cloned().collect();
        }
        else if board_complete(&boards[0]) {
            return board_score(&boards[0]) * num;
        }
    }
    panic!("No bingo");
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (sequence, boards) = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&sequence, boards.clone());
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&sequence, boards.clone());
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
    use super::parse_input;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (sequence, boards) = parse_input(input);
        assert_eq!(part_1(&sequence, boards.clone()), 4512);
        assert_eq!(part_2(&sequence, boards.clone()), 1924);
    }
}
