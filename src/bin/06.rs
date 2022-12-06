use std::collections::HashSet;

fn window_detector(stream: &str, size: usize) -> Option<u32> {
    let mut index = size;
    for candidate in stream.as_bytes().windows(size) {
        let test: HashSet<&u8> = HashSet::from_iter(candidate);
        if test.len() == size {
            return Some(index as u32);
        }
        index += 1;
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    window_detector(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    window_detector(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
