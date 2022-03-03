use std::fs;
use std::env;
use std::time::SystemTime;
use std::str::FromStr;
use std::collections::HashSet;
use itertools::Itertools;
use std::ops::Sub;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl FromStr for Pos {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(",").collect();
        Ok(Pos{
            x: i32::from_str_radix(parts[0], 10).unwrap(),
            y: i32::from_str_radix(parts[1], 10).unwrap(),
            z: i32::from_str_radix(parts[2], 10).unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Scanner {
    beacons: HashSet<Pos>,
    id: i32,
}

impl FromStr for Scanner {
    type Err = std::string::ParseError;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = section.lines().collect();

        let id = scan_fmt!(
            parts[0],
            "--- scanner {} ---",
            i32
        ).unwrap();

        let beacons = parts[1..].iter()
            .map(|line| Pos::from_str(line).unwrap())
            .collect();

        Ok(Scanner{id, beacons})
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.trim()
        .split("\n\n")
        .map(|scanner| Scanner::from_str(scanner).unwrap())
        .collect()
}

fn get_rotations(scanner: &Scanner) -> Vec<Scanner> {
    let permutations: Vec<Box<dyn Fn(&Pos) -> Pos>> = vec![
        Box::new(|b: &Pos| Pos{x: b.x, y: b.y, z: b.z}),
        Box::new(|b: &Pos| Pos{x: b.x, y: b.z, z: b.y}),
        Box::new(|b: &Pos| Pos{x: b.y, y: b.x, z: b.z}),
        Box::new(|b: &Pos| Pos{x: b.y, y: b.z, z: b.x}),
        Box::new(|b: &Pos| Pos{x: b.z, y: b.x, z: b.y}),
        Box::new(|b: &Pos| Pos{x: b.z, y: b.y, z: b.x}),
    ];
    let rotations: Vec<Box<dyn Fn(&Pos) -> Pos>> = vec![
        Box::new(|b: &Pos| Pos{x:  b.x, y:  b.y, z:  b.z}),
        Box::new(|b: &Pos| Pos{x:  b.x, y:  b.y, z: -b.z}),
        Box::new(|b: &Pos| Pos{x:  b.x, y: -b.y, z:  b.z}),
        Box::new(|b: &Pos| Pos{x:  b.x, y: -b.y, z: -b.z}),
        Box::new(|b: &Pos| Pos{x: -b.x, y:  b.y, z:  b.z}),
        Box::new(|b: &Pos| Pos{x: -b.x, y:  b.y, z: -b.z}),
        Box::new(|b: &Pos| Pos{x: -b.x, y: -b.y, z:  b.z}),
        Box::new(|b: &Pos| Pos{x: -b.x, y: -b.y, z: -b.z}),
    ];

    permutations.iter()
        .cartesian_product(&rotations)
        .map(|(perm, rot)| {
            Scanner{
                id: scanner.id,
                beacons: scanner.beacons.iter()
                    .map(|b| perm(&b))
                    .map(|b| rot(&b))
                    .collect(),
            }
        })
        .collect()
}

fn correlate_map(scanners: &Vec<Scanner>) -> (HashSet<Pos>, Vec<Pos>) {
    let mut remaining = scanners.clone();
    let mut map: HashSet<_> = remaining.pop().unwrap().beacons;
    let mut scanner_positions = vec![Pos{x: 0, y: 0, z: 0}];

    while remaining.len() > 0 {
        let base_candidate = remaining.pop().unwrap();
        let mut found = false;

        'outer: for candidate in get_rotations(&base_candidate) {
            for map_beacon in map.clone() {
                for scanner_beacon in &candidate.beacons {
                    let difference = *scanner_beacon - map_beacon;

                    let test_beacons: HashSet<_> = candidate.beacons.iter()
                        .map(|beacon| *beacon - difference)
                        .collect();
                    let common: HashSet<_> = map.intersection(&test_beacons).collect();
                    if common.len() >= 12 {
                        println!("found match for scanner {} at {:?} ({} matches)!", candidate.id, difference, common.len());
                        map.extend(test_beacons);
                        found = true;
                        scanner_positions.push(difference);
                        break 'outer;
                    }
                }
            }
        }

        if !found {
            remaining.insert(0, base_candidate);
        }
    }

    (map, scanner_positions)
}

fn part_1(scanners: &Vec<Scanner>) -> usize {
    let (map, _) = correlate_map(&scanners);
    map.len()
}

fn part_2(scanners: &Vec<Scanner>) -> i32 {
    let (_, scanner_positions) = correlate_map(&scanners);
    scanner_positions.iter()
        .permutations(2)
        .map(|a| (a[0].x - a[1].x).abs() + (a[0].y - a[1].y).abs() + (a[0].z - a[1].z).abs())
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
    use super::part_1;
    use super::part_2;

    #[test]
    fn example1() {
        let input =
"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let scanners = parse_input(input);
        assert_eq!(part_1(&scanners), 79);
        assert_eq!(part_2(&scanners), 3621);
    }
}
