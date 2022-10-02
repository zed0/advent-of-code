use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use itertools::Itertools;
use std::convert::Infallible;
use std::ops::{Add};
use std::cmp::{max, min};
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct BadState;

impl FromStr for State {
    type Err = BadState;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        match word {
            "on" => Ok(State::On),
            "off" => Ok(State::Off),
            _ => Err(BadState),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Cube {
    state: State,
    lowest: Pos,
    highest: Pos,
}

impl Cube {
    fn volume(self) -> i64 {
        max(0, self.highest.x - self.lowest.x)
        * max(0, self.highest.y - self.lowest.y)
        * max(0, self.highest.z - self.lowest.z)
    }

    fn contains(self, other: &Self) -> bool {
        (self.lowest.x <= other.lowest.x && self.highest.x >= other.highest.x)
        && (self.lowest.y <= other.lowest.y && self.highest.y >= other.highest.y)
        && (self.lowest.z <= other.lowest.z && self.highest.z >= other.highest.z)
    }

    fn intersects(self, other: &Self) -> bool {
        !(self.lowest.x >= other.highest.x || self.highest.x <= other.lowest.x)
        && !(self.lowest.y >= other.highest.y || self.highest.y <= other.lowest.y)
        && !(self.lowest.z >= other.highest.z || self.highest.z <= other.lowest.z)
    }

    fn breakup(self, other: &Self) -> Vec<Cube> {
        let x_segments = vec![
            (self.lowest.x, min(other.lowest.x, self.highest.x)),
            (max(other.lowest.x, self.lowest.x), min(other.highest.x, self.highest.x)),
            (max(other.highest.x, self.lowest.x), self.highest.x),
        ];

        let y_segments = vec![
            (self.lowest.y, min(other.lowest.y, self.highest.y)),
            (max(other.lowest.y, self.lowest.y), min(other.highest.y, self.highest.y)),
            (max(other.highest.y, self.lowest.y), self.highest.y),
        ];

        let z_segments = vec![
            (self.lowest.z, min(other.lowest.z, self.highest.z)),
            (max(other.lowest.z, self.lowest.z), min(other.highest.z, self.highest.z)),
            (max(other.highest.z, self.lowest.z), self.highest.z),
        ];

        x_segments.iter()
            .cartesian_product(y_segments.iter())
            .cartesian_product(z_segments.iter())
            .map(|(((x_low, x_high), (y_low, y_high)), (z_low, z_high))| {
                Cube{
                    state: self.state,
                    lowest: Pos{x: *x_low, y: *y_low, z: *z_low},
                    highest: Pos{x: *x_high, y: *y_high, z: *z_high},
                }
            })
            .filter(|c| c.volume() > 0)
            .collect()
    }
}

impl FromStr for Cube {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (state, x1, x2, y1, y2, z1, z2) = scan_fmt!(line.trim(), "{} x={}..{},y={}..{},z={}..{}", String, i64, i64, i64, i64, i64, i64).unwrap();

        Ok(Cube{
            state: State::from_str(&state).unwrap(),
            lowest: Pos{
                x: x1,
                y: y1,
                z: z1,
            },
            highest: Pos{
                x: x2 + 1,
                y: y2 + 1,
                z: z2 + 1,
            },
        })
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input.trim()
        .lines()
        .map(Cube::from_str)
        .map(|c| c.unwrap())
        .collect()
}

fn add_to_existing(new_cube: &Cube, existing_cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut cubes_to_check = existing_cubes.clone();
    let mut next = vec![];
    while !cubes_to_check.is_empty() {
        let existing_cube = cubes_to_check.pop().unwrap();
        if new_cube.contains(&existing_cube) {
            continue;
        }
        else if !new_cube.intersects(&existing_cube) {
            next.push(existing_cube.clone());
        }
        else {
            existing_cube.breakup(new_cube)
                .iter()
                .for_each(|c| cubes_to_check.push(*c));
        }
    }
    next.push(new_cube.clone());

    next
}

fn apply_cubes(cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut existing_cubes = vec![];
    cubes.iter()
        .for_each(|new_cube| {
            existing_cubes = add_to_existing(&new_cube, &existing_cubes);
        });
    existing_cubes
}

fn part_1(cubes: &Vec<Cube>) -> i64 {
    let filter_zone = Cube{
        state: State::On,
        lowest: Pos{x: -50, y: -50, z: -50},
        highest: Pos{x: 51, y: 51, z: 51}
    };
    let small_cubes: Vec<Cube> = cubes.iter()
        .filter(|c| filter_zone.contains(c))
        .cloned()
        .collect();
    apply_cubes(&small_cubes)
    .iter()
    .filter(|c| c.state == State::On)
    .map(|c| c.volume())
    .sum()
}

fn part_2(cubes: &Vec<Cube>) -> i64 {
    apply_cubes(cubes)
    .iter()
    .filter(|c| c.state == State::On)
    .map(|c| c.volume())
    .sum()
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
        let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
        let cubes = parse_input(input);
        assert_eq!(part_1(&cubes), 39);
    }

    #[test]
    fn example2() {
        let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
        let cubes = parse_input(input);
        assert_eq!(part_1(&cubes), 590784);
    }

    #[test]
    fn example3() {
        let input = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        let cubes = parse_input(input);
        assert_eq!(part_2(&cubes), 2758514936282235);
    }
}
