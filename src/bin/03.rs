use std::{collections::HashSet};

use itertools::Itertools;


pub fn part_one(input: &str) -> Option<u32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut max_score = 0;

    for inp in input.lines() {

        let left:HashSet<char> = HashSet::from_iter(inp[..inp.len()/2].chars());
        let right:HashSet<char>= HashSet::from_iter(inp[inp.len()/2..].chars());

        let c = left.intersection(&right).next().unwrap();
        max_score += (alphabet.find(*c).unwrap() as u32) + 1;

    }

    Some(max_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut max_score = 0;

    for inps in &input.lines().chunks(3) {
        let bags:Vec<&str> = inps.collect();

        let e0: HashSet<char> =  HashSet::from_iter(bags[0].chars());
        let e1: HashSet<char> =  HashSet::from_iter(bags[1].chars());
        let e2: HashSet<char> =  HashSet::from_iter(bags[2].chars());

        let c = e0.intersection(&e1).find(|x| e2.contains(*x)).unwrap();
        max_score += (alphabet.find(*c).unwrap() as u32) + 1;
    }

    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
