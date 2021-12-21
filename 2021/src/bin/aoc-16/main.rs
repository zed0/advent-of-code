use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::convert::Infallible;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add};
use hex::FromHex;
use bit_vec::BitVec;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Packet {
    Literal{version: usize, packet_type: usize, value: usize},
    Operator{version: usize, packet_type: usize, subpackets: Vec<Packet>},
}

impl Packet {
    fn from_bits(mut bitstream: &mut bit_vec::Iter) -> Self {
        let version = bits_to_usize(bitstream.take(3).collect::<BitVec>());
        let packet_type = bits_to_usize(bitstream.take(3).collect::<BitVec>());
        match packet_type {
            4 => Packet::Literal{version, packet_type, value: read_literal_value(&mut bitstream)},
            _ => {
                match bitstream.next().unwrap() {
                    false => {
                        let bits_to_read = bits_to_usize(bitstream.take(15).collect::<BitVec>());
                        Packet::Operator{version, packet_type, subpackets: read_n_bits(&mut bitstream, bits_to_read)}
                    },
                    true => {
                        let packets_to_read = bits_to_usize(bitstream.take(11).collect::<BitVec>());
                        Packet::Operator{version, packet_type, subpackets: read_n_packets(&mut bitstream, packets_to_read)}
                    }
                }
            }
        }
    }
}

impl FromStr for Packet {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let bytes = Vec::<u8>::from_hex(&line.trim()).unwrap();
        let bits = BitVec::from_bytes(&bytes);

        Ok(Packet::from_bits(&mut bits.iter()))
    }
}

fn read_n_bits(bitstream: &mut bit_vec::Iter, n: usize) -> Vec<Packet> {
    let bits = bitstream.take(n).collect::<BitVec>();
    let mut result = vec![];
    let mut partial_bitstream = bits.iter();
    while partial_bitstream.clone().peekable().peek().is_some() {
        result.push(Packet::from_bits(&mut partial_bitstream));
    }
    result
}

fn read_n_packets(mut bitstream: &mut bit_vec::Iter, n: usize) -> Vec<Packet> {
    let mut packets = vec![];
    for _ in 0..n {
        packets.push(Packet::from_bits(&mut bitstream));
    }
    packets
}

fn read_literal_value(bitstream: &mut bit_vec::Iter) -> usize {
    let mut bits = BitVec::new();
    loop {
        let stop = !bitstream.next().unwrap();
        bits.append(&mut bitstream.take(4).collect::<BitVec>());
        if stop {
            break;
        }
    }
    bits_to_usize(bits)
}

fn bits_to_usize(mut bits: BitVec) -> usize {
    let bytes = (bits.len()+7)/8;
    let mut new_bits = BitVec::from_elem((bytes*8)-bits.len(), false);
    new_bits.append(&mut bits);
    let mut a: [u8; 8] = Default::default();
    a[8-bytes..8].copy_from_slice(&new_bits.to_bytes());
    a.reverse();
    usize::from_le_bytes(a)
}

fn parse_input(input: &str) -> Packet {
    Packet::from_str(input).unwrap()
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal{version, packet_type: _,  value: _} => *version,
        Packet::Operator{version, packet_type: _,  subpackets} => *version + subpackets.iter().map(sum_versions).sum::<usize>(),
    }
}

fn evaluate(packet: &Packet) -> usize {
    match packet {
        Packet::Literal{version: _, packet_type: _, value} => *value,
        Packet::Operator{version: _, packet_type: 0, subpackets} => subpackets.iter().map(evaluate).sum::<usize>(),
        Packet::Operator{version: _, packet_type: 1, subpackets} => subpackets.iter().map(evaluate).product::<usize>(),
        Packet::Operator{version: _, packet_type: 2, subpackets} => subpackets.iter().map(evaluate).min().unwrap(),
        Packet::Operator{version: _, packet_type: 3, subpackets} => subpackets.iter().map(evaluate).max().unwrap(),
        Packet::Operator{version: _, packet_type: 5, subpackets} => (evaluate(&subpackets[0]) > evaluate(&subpackets[1])) as usize,
        Packet::Operator{version: _, packet_type: 6, subpackets} => (evaluate(&subpackets[0]) < evaluate(&subpackets[1])) as usize,
        Packet::Operator{version: _, packet_type: 7, subpackets} => (evaluate(&subpackets[0]) == evaluate(&subpackets[1])) as usize,
        _ => panic!("Unknown packet type: {:?}", packet),
    }
}

fn part_1(packet: &Packet) -> usize {
    sum_versions(&packet)
}

fn part_2(packet: &Packet) -> usize {
    evaluate(&packet)
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
    use super::Packet;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input = "D2FE28";
        let packet = parse_input(input);
        assert_eq!(packet, Packet::Literal{version: 6, packet_type: 4, value: 2021});
        assert_eq!(part_1(&packet), 6);
        assert_eq!(part_2(&packet), 2021);
    }

    #[test]
    fn example2() {
        let input = "38006F45291200";
        let packet = parse_input(input);
        assert_eq!(packet, Packet::Operator{version: 1, packet_type: 6, subpackets: vec![Packet::Literal{version: 6, packet_type: 4, value: 10}, Packet::Literal{version: 2, packet_type: 4, value: 20}]});
        assert_eq!(part_1(&packet), 9);
        assert_eq!(part_2(&packet), 1);
    }

    #[test]
    fn example3() {
        let input = "EE00D40C823060";
        let packet = parse_input(input);
        assert_eq!(packet, Packet::Operator{version: 7, packet_type: 3, subpackets: vec![Packet::Literal{version: 2, packet_type: 4, value: 1}, Packet::Literal{version: 4, packet_type: 4, value: 2}, Packet::Literal{version: 1, packet_type: 4, value: 3}]});
        assert_eq!(part_1(&packet), 14);
        assert_eq!(part_2(&packet), 3);
    }
}
