use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn part_one() {
    let file = File::open("inputs/day01.txt").expect("The input file must be present.");
    let lines = BufReader::new(file).lines();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for (idx, line) in lines.flatten().enumerate() {
        let mut sline = line.split_whitespace();

        let l = sline.next();
        let r = sline.next();

        assert!(l.is_some() && r.is_some(), "Failed to split line {}.", idx);

        let l: u32 = l
            .unwrap()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse left list as a number on line {}.", idx));
        let r: u32 = r
            .unwrap()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse right list as a number on line {}.", idx));

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let diff: u32 = left
        .iter()
        .zip(right.iter())
        .map(|values| values.0.abs_diff(*values.1))
        .sum();

    println!("Total diff: {}", diff)
}

#[test]
fn part_two() {
    let file = File::open("inputs/day01.txt").expect("The input file must be present.");
    let lines = BufReader::new(file).lines();

    let mut counts: HashMap<u32, (u32, u32)> = HashMap::new();
    for (idx, line) in lines.flatten().enumerate() {
        let mut sline = line.split_whitespace();

        let l = sline.next();
        let r = sline.next();

        assert!(l.is_some() && r.is_some(), "Failed to split line {}.", idx);

        let l: u32 = l
            .unwrap()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse left list as a number on line {}.", idx));
        let r: u32 = r
            .unwrap()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse right list as a number on line {}.", idx));

        let count = counts.entry(l).or_insert((0, 0));
        count.0 += 1;

        let count = counts.entry(r).or_insert((0, 0));
        count.1 += 1;
    }

    let mut sim = 0;
    for (k, v) in counts.iter() {
        sim += *k as u64 * v.0 as u64 * v.1 as u64;
    }

    println!("Similarity: {}", sim)
}
