mod line_reader;

use itertools::Itertools;
use line_reader::*;
use std::{collections::HashSet, io::Result};

/**
 * Item (char)
 *
 *
 * Rucksack (string of event size)
 * Compartment (rucksack split in the middle)
 *
 * find which item is present in both compartments
 *
 */
//////////////
///

type Item = char;

type Compartments<'a> = (&'a str, &'a str);

trait PriorityLevel {
    fn priority(self) -> i32;
}

impl PriorityLevel for Item {
    fn priority(self) -> i32 {
        match self {
            ('a'..='z') => (self as u8 - b'a' + 1) as i32,
            ('A'..='Z') => (self as u8 - b'A' + 27) as i32,
            _ => 0,
        }
    }
}

fn split_rucksack(rucksack: &str) -> Compartments {
    assert_eq!(0, rucksack.len() % 2);

    rucksack.split_at(rucksack.len() / 2)
}

fn find_common_items(compartments: Compartments) -> HashSet<Item> {
    let (comp_a, comp_b) = compartments;

    comp_a.chars().filter(|ca| comp_b.contains(*ca)).collect()
}

fn calc_rucksack_score(rucksack: Compartments) -> i32 {
    let common_items = find_common_items(rucksack);

    common_items.into_iter().map(|item| item.priority()).sum()
}

const INPUT_FILENAME: &str = "./inputs/input.txt";

fn part1() -> Result<()> {
    let lines_buffer = read_lines(INPUT_FILENAME)?;

    let mut total_score = 0;
    for line in lines_buffer.flatten() {
        let rucksack = split_rucksack(&line);
        total_score += calc_rucksack_score(rucksack)
    }

    println!("Part1 score: {}", total_score);
    Ok(())
}
fn part2() -> Result<()> {
    let lines_buffer = read_lines(INPUT_FILENAME)?;

    // partition into triples
    let lines_iter = lines_buffer.flatten();

    let total_score = (&lines_iter.chunks(3))
        .into_iter()
        .map(|elves| {
            let elves = elves.collect_tuple();
            if elves.is_none() {
                return 0;
            }

            let (rs_a, rs_b, rs_c) = elves.unwrap();
            let common_item = rs_a
                .chars()
                .find(|item_a| rs_b.contains(*item_a) && rs_c.contains(*item_a))
                .unwrap_or_default();
            common_item.priority()
        })
        .sum::<i32>();

    println!("Part2 score: {}", total_score);

    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()
}
