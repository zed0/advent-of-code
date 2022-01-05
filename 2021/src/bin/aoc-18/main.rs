use std::fs;
use std::env;
use std::time::SystemTime;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Item {
    value: u32,
    path: Vec<Direction>
}

fn parse_line(line: &str) -> Vec<Item> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    line.chars().for_each(|c: char| match c {
        '[' => {path.push(Direction::Left);},
        ']' => {path.pop();},
        ',' => {path.pop(); path.push(Direction::Right);},
        _ => result.push(Item{
            value: c.to_digit(10).unwrap(),
            path: path.clone()
        }),
    });
    result
}

fn parse_input(input: &str) -> Vec<Vec<Item>> {
    input.trim()
        .lines()
        .map(parse_line)
        .collect()
}

fn path_value(path: &Vec<Direction>) -> u32 {
    path.iter()
        .map(|dir| match dir {
            Direction::Left => 3,
            Direction::Right => 2,
        })
        .product()
}

fn magnitude(items: &Vec<Item>) -> u32 {
    items.iter()
        .map(|item| path_value(&item.path) * item.value)
        .sum()
}

fn explode(items: &Vec<Item>) -> Option<Vec<Item>> {
    let mut result = items.clone();
    let pos = result.iter().position(|item| item.path.len() > 4)?;
    if pos > 0 {
        result[pos - 1].value += result[pos].value;
    }
    if pos + 2 < result.len() {
        result[pos + 2].value += result[pos + 1].value;
    }
    result.splice(
        pos..=pos+1,
        vec![Item{
            value: 0,
            path: result[pos].path[..result[pos].path.len() - 1].into(),
        }]
    );
    Some(result)
}

fn split(items: &Vec<Item>) -> Option<Vec<Item>> {
    let mut result = items.clone();
    let pos = result.iter().position(|item| item.value >= 10)?;
    result.splice(
        pos..=pos,
        vec![
            Item{
                value: result[pos].value / 2,
                path: [result[pos].path.clone(), vec![Direction::Left]].concat(),
            },
            Item{
                value: (result[pos].value+1) / 2,
                path: [result[pos].path.clone(), vec![Direction::Right]].concat(),
            }
        ]
    );
    Some(result)
}

fn reduce(items: &Vec<Item>) -> Vec<Item> {
    let mut current = items.clone();
    loop {
        let exploded = explode(&current);
        if exploded.is_some() {
            current = exploded.unwrap();
            continue
        }
        let split = split(&current);
        if split.is_some() {
            current = split.unwrap();
            continue
        }
        break
    }
    current
}

fn add(left: &Vec<Item>, right: &Vec<Item>) -> Vec<Item> {
    let result = [
        left.iter().map(|item| {let mut new = item.clone(); new.path.insert(0, Direction::Left); new}).collect::<Vec<Item>>(),
        right.iter().map(|item| {let mut new = item.clone(); new.path.insert(0, Direction::Right); new}).collect::<Vec<Item>>(),
    ].concat();

    reduce(&result)
}

fn part_1(items: &Vec<Vec<Item>>) -> u32 {
    magnitude(&items.iter().cloned().reduce(|acc, item| add(&acc, &item)).unwrap())
}

fn part_2(items: &Vec<Vec<Item>>) -> u32 {
    items.iter()
        .permutations(2)
        .map(|nums| magnitude(&add(&nums[0], &nums[1])))
        .max()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let map = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map);
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
    use super::magnitude;
    use super::explode;
    use super::split;
    use super::add;
    use super::Item;
    use super::Direction::*;
    use super::part_1;
    use super::part_2;

    #[test]
    fn example1() {
        use super::Item;
        let input =
"[1,1]
[2,2]
[3,3]
[4,4]";
        let pairs = parse_input(input);
        assert_eq!(
            pairs,
            vec![
                vec![Item{value: 1, path: vec![Left]}, Item{value: 1, path: vec![Right]}],
                vec![Item{value: 2, path: vec![Left]}, Item{value: 2, path: vec![Right]}],
                vec![Item{value: 3, path: vec![Left]}, Item{value: 3, path: vec![Right]}],
                vec![Item{value: 4, path: vec![Left]}, Item{value: 4, path: vec![Right]}],
            ]
        );
        assert_eq!(
            pairs.iter().map(magnitude).collect::<Vec<u32>>(),
            vec![5, 10, 15, 20]
        );
    }

    #[test]
    fn example2() {
        let input =
"[[1,2],[[3,4],5]]
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
[[[[1,1],[2,2]],[3,3]],[4,4]]
[[[[3,0],[5,3]],[4,4]],[5,5]]
[[[[5,0],[7,4]],[5,5]],[6,6]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let pairs = parse_input(input);
        assert_eq!(
            pairs.iter().map(magnitude).collect::<Vec<u32>>(),
            vec![143, 1384, 445, 791, 1137, 3488]
        );
    }

    #[test]
    fn example3() {
        let input_text =
"[[[[[9,8],1],2],3],4]
[7,[6,[5,[4,[3,2]]]]]
[[6,[5,[4,[3,2]]]],1]
[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]
[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let output_text =
"[[[[0,9],2],3],4]
[7,[6,[5,[7,0]]]]
[[6,[5,[7,0]]],3]
[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

        let inputs = parse_input(input_text);
        let outputs = parse_input(output_text);
        inputs.iter()
            .zip(outputs.iter())
            .for_each(|(input, output)| {
                assert_eq!(
                    &explode(input).unwrap(),
                    output
                );
            });
    }

    #[test]
    fn example4() {
        let inputs = vec![
            // [15,[0,13]]
            vec![Item{value: 15, path: vec![Left]}, Item{value: 0, path: vec![Right, Left]}, Item{value: 13, path: vec![Right, Right]}],
            // [[7,8],[0,13]]
            vec![Item{value: 7, path: vec![Left, Left]}, Item{value: 8, path: vec![Left, Right]}, Item{value: 0, path: vec![Right, Left]}, Item{value: 13, path: vec![Right, Right]}],
        ];
        let outputs = vec![
            //[[7,8],[0,13]]
            vec![Item{value: 7, path: vec![Left, Left]}, Item{value: 8, path: vec![Left, Right]}, Item{value: 0, path: vec![Right, Left]}, Item{value: 13, path: vec![Right, Right]}],
            //[[7,8],[0,[6,7]]]
            vec![Item{value: 7, path: vec![Left, Left]}, Item{value: 8, path: vec![Left, Right]}, Item{value: 0, path: vec![Right, Left]}, Item{value: 6, path: vec![Right, Right, Left]}, Item{value: 7, path: vec![Right, Right, Right]}],
        ];

        inputs.iter()
            .zip(outputs.iter())
            .for_each(|(input, output)| {
                assert_eq!(
                    &split(input).unwrap(),
                    output
                );
            });
    }

    #[test]
    fn example5() {
        let input_text =
"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";
        let output_text =
"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

        let inputs = parse_input(input_text);
        let outputs = parse_input(output_text);
        assert_eq!(
            add(&inputs[0], &inputs[1]),
            outputs[0]
        );
    }

    #[test]
    fn example6() {
        let input_text =
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let inputs = parse_input(input_text);
        assert_eq!(
            part_1(&inputs),
            4140
        );

        assert_eq!(
            part_2(&inputs),
            3993
        );
    }
}
